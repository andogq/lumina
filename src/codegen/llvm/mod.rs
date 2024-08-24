use std::collections::HashMap;

use index_vec::IndexVec;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module as LlvmModule,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValue, FunctionValue, IntValue, PointerValue},
    IntPredicate,
};

use crate::{
    compiler::Compiler,
    repr::{
        identifier::{FunctionIdx, ScopedBinding},
        ir::{
            BasicBlockIdx, BinaryOp, ConstantValue, Function, Terminator, Triple, TripleRef,
            UnaryOp, Value,
        },
        ty::Ty,
    },
};

/// A single LLVM module.
pub struct Module<'compiler, 'ink> {
    /// Compiler state.
    compiler: &'compiler Compiler,

    /// LLVM context.
    llvm_ctx: &'ink Context,

    /// The underlying LLVM module.
    module: LlvmModule<'ink>,

    /// All functions available within this module.
    functions: IndexVec<FunctionIdx, FunctionValue<'ink>>,
}

impl<'compiler, 'ink> Module<'compiler, 'ink> {
    /// Create a new module from an existing [`Compiler`] and LLVM [`Context`].
    pub fn new(compiler: &'compiler Compiler, llvm_ctx: &'ink Context) -> Self {
        let module = llvm_ctx.create_module("module");

        // TODO: Declare all functions in the compiler
        let functions = compiler
            .functions
            .iter()
            .map(|(idx, registration)| {
                module.add_function(
                    // Pull out the name of the function
                    compiler
                        .symbols
                        .resolve(
                            compiler
                                .functions
                                .symbol_for(idx)
                                .expect("registered function must have symbol"),
                        )
                        .unwrap(),
                    // Build up the type for the function
                    {
                        let signature = registration.get_signature();

                        llvm_ctx.get_ty(&signature.return_ty).fn_type(
                            &signature
                                .arguments
                                .iter()
                                .map(|ty| llvm_ctx.get_ty(ty).into())
                                .collect::<Vec<_>>(),
                            false,
                        )
                    },
                    None,
                )
            })
            // WARN: Assumes that the enumeration and collection happens in the same order.
            .collect();

        Self {
            compiler,
            llvm_ctx,
            module,
            functions,
        }
    }

    /// Compile a [`Function`] into the module.
    pub fn compile(&self, function: &Function) -> FunctionValue<'ink> {
        let value = self.functions[function.identifier];

        // TODO: Don't do this
        FunctionGenerator::new(self, value, function).codegen();

        value
    }

    /// Produce the inner LLVM module.
    pub fn into_inner(self) -> LlvmModule<'ink> {
        self.module
    }
}

/// Utility methods for LLVM context.
trait ContextExt {
    /// Get the internal representation for a [`Ty`].
    fn get_ty(&self, ty: &Ty) -> BasicTypeEnum;
}

impl ContextExt for Context {
    fn get_ty(&self, ty: &Ty) -> BasicTypeEnum {
        match ty {
            Ty::Int => self.i64_type().into(),
            Ty::Uint => self.i64_type().into(),
            Ty::Boolean => self.bool_type().into(),
            Ty::Unit => todo!(),
            Ty::Never => todo!(),
        }
    }
}

pub struct FunctionGenerator<'module, 'compiler, 'ink> {
    module: &'module Module<'compiler, 'ink>,

    /// IR for this function
    function: &'module Function,
    /// LLVM function value assigned for this function
    llvm_function: FunctionValue<'ink>,
    builder: Builder<'ink>,

    /// Resulting values for each of the triples
    results: HashMap<TripleRef, Option<IntValue<'ink>>>,
    bindings: HashMap<ScopedBinding, PointerValue<'ink>>,

    blocks: HashMap<BasicBlockIdx, BasicBlock<'ink>>,
}

impl<'module, 'compiler, 'ink> FunctionGenerator<'module, 'compiler, 'ink> {
    pub fn new(
        module: &'module Module<'compiler, 'ink>,
        llvm_function: FunctionValue<'ink>,
        function: &'module Function,
    ) -> Self {
        let builder = module.llvm_ctx.create_builder();

        // Set up the entry block for this function
        assert_eq!(
            llvm_function.count_basic_blocks(),
            0,
            "function should not have any basic blocks"
        );
        let entry = module.llvm_ctx.append_basic_block(llvm_function, "entry");

        builder.position_at_end(entry);

        Self {
            module,
            llvm_function,
            builder,
            function,
            bindings: HashMap::new(),
            results: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn codegen(&mut self) {
        let registration = self
            .module
            .compiler
            .functions
            .get(self.function.identifier)
            .unwrap();

        // Create stack allocations for all of the variables in scope
        self.bindings = self
            .function
            .scope
            .iter()
            .map(|binding| {
                let (symbol, ty) = registration.get_binding(*binding).unwrap();

                (
                    *binding,
                    self.alloca(ty, self.module.compiler.symbols.resolve(symbol).unwrap()),
                )
            })
            .collect::<HashMap<_, _>>();

        let user_entry = self.gen_block(
            &self
                .function
                .basic_blocks
                .iter_enumerated()
                .next()
                .unwrap()
                .0,
        );

        self.builder
            .position_at_end(self.llvm_function.get_first_basic_block().unwrap());
        self.builder.build_unconditional_branch(user_entry).unwrap();
    }

    fn gen_block(&mut self, block_idx: &BasicBlockIdx) -> BasicBlock<'ink> {
        if let Some(block) = self.blocks.get(block_idx) {
            return *block;
        }

        let prev_builder = self.builder.get_insert_block();

        let basic_block = self
            .module
            .llvm_ctx
            .append_basic_block(self.llvm_function, &format!("bb_{block_idx:?}"));
        self.blocks.insert(*block_idx, basic_block);
        self.builder.position_at_end(basic_block);

        let block = self.function.basic_blocks.get(*block_idx).unwrap().clone();

        for (idx, triple) in block.triples.iter_enumerated() {
            let result = match triple {
                Triple::BinaryOp { lhs, rhs, op } => Some(self.gen_op_binary(lhs, rhs, op)),
                Triple::UnaryOp { rhs, op } => Some(self.gen_op_unary(rhs, op)),
                Triple::Copy(value) => Some(self.gen_copy(value)),
                Triple::Call(function, params) => Some(self.gen_call(function, params)),
                Triple::Assign(symbol, value) => {
                    self.gen_assign(symbol, value);
                    None
                }
                Triple::Load(binding) => Some(self.gen_load(binding)),
                Triple::Phi(values) => Some(self.gen_phi(values)),
            };
            self.results.insert(TripleRef::new(*block_idx, idx), result);
        }

        // Lower the block terminator
        match &block.terminator {
            Terminator::Jump(bb) => self.gen_jump(bb),
            Terminator::Return(value) => self.gen_return(value),
            Terminator::Switch {
                value,
                default,
                branches,
            } => self.gen_switch(value, default, branches),
        }

        if let Some(prev) = prev_builder {
            self.builder.position_at_end(prev);
        }

        basic_block
    }

    fn gen_op_binary(&mut self, lhs: &Value, rhs: &Value, op: &BinaryOp) -> IntValue<'ink> {
        let lhs = self
            .retrieve_value(lhs)
            .expect("lhs of binary op cannot be unit");
        let rhs = self
            .retrieve_value(rhs)
            .expect("rhs of binary cannot be unit");

        match op {
            BinaryOp::Add => self.builder.build_int_add(lhs, rhs, "add_result").unwrap(),
            BinaryOp::Sub => self.builder.build_int_sub(lhs, rhs, "sub_result").unwrap(),
            BinaryOp::Multiply => self.builder.build_int_mul(lhs, rhs, "mul_result").unwrap(),
            BinaryOp::Divide => self
                .builder
                .build_int_signed_div(lhs, rhs, "div_result")
                .unwrap(),
            BinaryOp::Eq => self
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq_result")
                .unwrap(),
            BinaryOp::NotEq => self
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, "not_eq_result")
                .unwrap(),
            BinaryOp::And => self.builder.build_and(lhs, rhs, "and_result").unwrap(),
            BinaryOp::Or => self.builder.build_or(lhs, rhs, "or_result").unwrap(),
            BinaryOp::Greater => self
                .builder
                .build_int_compare(IntPredicate::SGT, lhs, rhs, "greater_result")
                .unwrap(),
            BinaryOp::Less => self
                .builder
                .build_int_compare(IntPredicate::SLT, lhs, rhs, "less_result")
                .unwrap(),
            BinaryOp::GreaterEq => self
                .builder
                .build_int_compare(IntPredicate::SGE, lhs, rhs, "greater_eq_result")
                .unwrap(),
            BinaryOp::LessEq => self
                .builder
                .build_int_compare(IntPredicate::SLE, lhs, rhs, "less_eq_result")
                .unwrap(),
        }
    }

    fn gen_op_unary(&mut self, rhs: &Value, op: &UnaryOp) -> IntValue<'ink> {
        let rhs = self
            .retrieve_value(rhs)
            .expect("rhs of unary cannot be unit");

        match op {
            UnaryOp::Minus => self.builder.build_int_neg(rhs, "neg_result").unwrap(),
            UnaryOp::Not => self.builder.build_not(rhs, "not_result").unwrap(),
        }
    }

    fn gen_copy(&mut self, value: &Value) -> IntValue<'ink> {
        self.retrieve_value(value).unwrap()
    }

    fn gen_jump(&mut self, bb: &BasicBlockIdx) {
        // Ensure the basic block is compiled
        let bb = self.gen_block(bb);

        // Insert the branch to it
        self.builder.build_unconditional_branch(bb).unwrap();
    }

    fn gen_call(&mut self, function: &FunctionIdx, params: &[Value]) -> IntValue<'ink> {
        // Ensure the function is compiled
        let function_value = self.module.functions.get(*function).unwrap();

        self.builder
            .build_call(
                function_value.to_owned(),
                params
                    .iter()
                    .map(|param| self.retrieve_value(param).unwrap().into())
                    .collect::<Vec<_>>()
                    .as_slice(),
                &format!(
                    "{}_result",
                    self.module
                        .compiler
                        .functions
                        .symbol_for(*function)
                        .and_then(|symbol| self.module.compiler.symbols.resolve(symbol))
                        .unwrap()
                ),
            )
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value()
    }

    fn gen_return(&self, value: &Value) {
        let value = self.retrieve_value(value);

        self.builder
            .build_return(value.as_ref().map(|value| value as &dyn BasicValue))
            .unwrap();
    }

    fn gen_assign(&self, ident: &ScopedBinding, value: &Value) {
        let value = self
            .retrieve_value(value)
            .expect("unit value cannot be assigned");
        let ptr = self.bindings.get(ident).unwrap();

        self.builder.build_store(*ptr, value).unwrap();
    }

    fn gen_load(&self, binding: &ScopedBinding) -> IntValue<'ink> {
        let (symbol, _) = self
            .module
            .compiler
            .functions
            .get(self.function.identifier)
            .unwrap()
            .get_binding(*binding)
            .unwrap();

        let ptr = self.bindings.get(binding).expect("symbol must be defined");
        let name = self.module.compiler.symbols.resolve(symbol).unwrap();

        self.builder
            .build_load(self.module.llvm_ctx.i64_type(), *ptr, name)
            .unwrap()
            .into_int_value()
    }

    fn gen_switch(
        &mut self,
        value: &Value,
        default: &BasicBlockIdx,
        branches: &[(Value, BasicBlockIdx)],
    ) {
        // Emit the switch instruction
        let cases = branches
            .iter()
            .map(|(case, bb)| {
                (
                    // Compile the case value
                    self.retrieve_value(case)
                        .expect("cannot use unit value as switch case"),
                    // Compile the destination basic block
                    self.gen_block(bb),
                )
            })
            .collect::<Vec<_>>();

        let else_block = self.gen_block(default);

        self.builder
            .build_switch(
                // Build the value to switch on
                self.retrieve_value(value)
                    .expect("cannot switch on unit value"),
                // Compile out the default branch
                else_block,
                &cases,
            )
            .unwrap();
    }

    fn gen_phi(&mut self, values: &[(Value, BasicBlockIdx)]) -> IntValue<'ink> {
        values
            .iter()
            .fold(
                self.builder
                    .build_phi(self.module.llvm_ctx.i64_type(), "switch phi")
                    .unwrap(),
                |phi, (value, bb)| {
                    let bb = self.gen_block(bb);
                    let value = self.retrieve_value(value).unwrap();

                    phi.add_incoming(&[(&value, bb)]);

                    phi
                },
            )
            .as_basic_value()
            .into_int_value()
    }

    /// Emit an allocation instruction in the entry basic block
    fn alloca(&self, ty: Ty, name: &str) -> PointerValue<'ink> {
        // Find the entry for this function
        let entry = self.llvm_function.get_first_basic_block().unwrap();

        // Create and position a builder to the end of the entry
        let builder = self.module.llvm_ctx.create_builder();
        builder.position_at_end(entry);

        builder
            .build_alloca(self.module.llvm_ctx.get_ty(&ty), name)
            .unwrap()
    }

    fn retrieve_value(&self, value: &Value) -> Option<IntValue<'ink>> {
        match value {
            Value::Constant(value) => Some(match value {
                ConstantValue::Integer(value) => self
                    .module
                    .llvm_ctx
                    .i64_type()
                    .const_int(*value as u64, false),
                ConstantValue::Boolean(value) => {
                    let ty = self.module.llvm_ctx.bool_type();

                    if *value {
                        ty.const_all_ones()
                    } else {
                        ty.const_zero()
                    }
                }
            }),
            Value::Triple(triple) => Some(
                self.results
                    .get(triple)
                    .expect("triple must exist")
                    .expect("triple must produce value"),
            ),
            Value::Parameter(i) => Some(
                self.llvm_function
                    .get_nth_param(*i as u32)
                    .unwrap()
                    .into_int_value(),
            ),
            Value::Unit => None,
        }
    }
}
