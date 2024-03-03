use crate::runtime::{object::Object, vm::VM};

impl VM {
    pub(super) fn stack_push_constant(&mut self, offset: u32) -> Result<(), String> {
        self.stack.push(self.constants[offset as usize].clone())
    }

    pub(super) fn stack_push_boolean(&mut self, boolean: bool) -> Result<(), String> {
        self.stack.push(Object::boolean(boolean))
    }

    pub(super) fn stack_push_null(&mut self) -> Result<(), String> {
        self.stack.push(Object::null())
    }

    pub(super) fn stack_pop(&mut self) -> Result<(), String> {
        self.stack.pop()?;
        Ok(())
    }
}
