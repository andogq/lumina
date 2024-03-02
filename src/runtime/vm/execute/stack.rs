use crate::runtime::{
    object::{BooleanObject, Object},
    vm::VM,
};

impl VM {
    pub(super) fn stack_push_constant(&mut self, offset: u32) -> Result<(), String> {
        self.stack.push(self.constants[offset as usize].clone())
    }

    pub(super) fn stack_push_boolean(&mut self, boolean: bool) -> Result<(), String> {
        self.stack
            .push(Object::Boolean(BooleanObject { value: boolean }))
    }

    pub(super) fn stack_pop(&mut self) -> Result<(), String> {
        self.stack.pop()?;
        Ok(())
    }
}
