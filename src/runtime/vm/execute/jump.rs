use crate::runtime::vm::VM;

impl VM {
    pub(super) fn jump_not_true(&mut self, offset: i16) -> Result<(), String> {
        if !self.stack.pop_boolean()? {
            self.jump(offset)?;
        }

        Ok(())
    }

    pub(super) fn jump(&mut self, offset: i16) -> Result<(), String> {
        if let Some(pc) = self.pc.checked_add_signed(offset as isize) {
            self.pc = pc;
            Ok(())
        } else {
            Err("jump out of bounds".to_string())
        }
    }
}
