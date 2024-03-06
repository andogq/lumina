use crate::runtime::{object::Object, vm::VM};

impl VM {
    pub(super) fn global_get(&mut self, id: u16) -> Result<(), String> {
        // Fetch the value from the symbol table and add it onto the stack
        self.stack
            .push(self.symbols.get(&id).cloned().unwrap_or(Object::null()))?;

        Ok(())
    }

    pub(super) fn global_set(&mut self, id: u16) -> Result<(), String> {
        // Fetch top value from the stack
        let value = self.stack.pop()?;

        // Assign it in the symbol table
        self.symbols.insert(id, value);

        Ok(())
    }
}
