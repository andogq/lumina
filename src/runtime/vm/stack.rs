use crate::runtime::object::Object;

pub struct Stack {
    stack: [Option<Object>; 1024],
    pointer: usize,
    pub(super) last_popped: Option<Object>,
}

const ARRAY_REPEAT_VALUE: Option<Object> = None;

impl Default for Stack {
    fn default() -> Self {
        Self {
            stack: [ARRAY_REPEAT_VALUE; 1024],
            pointer: 0,
            last_popped: None,
        }
    }
}

impl Stack {
    pub fn push(&mut self, object: Object) -> Result<(), String> {
        if self.pointer < self.stack.len() {
            self.stack[self.pointer] = Some(object);
            self.pointer += 1;
            Ok(())
        } else {
            Err("stack overflow".to_string())
        }
    }

    pub fn pop(&mut self) -> Result<Object, String> {
        if self.pointer > 0 {
            self.pointer -= 1;

            let value = std::mem::replace(&mut self.stack[self.pointer], None)
                .ok_or_else(|| "value not found on stack".to_string())?;

            // WARN: Probably bad
            self.last_popped = Some(value.clone());

            Ok(value)
        } else {
            Err("stack underflow(?)".to_string())
        }
    }

    pub fn pop_integer(&mut self) -> Result<i64, String> {
        if let Object::Integer(integer) = self.pop()? {
            Ok(integer.value)
        } else {
            Err("expected integer".to_string())
        }
    }

    pub fn pop_boolean(&mut self) -> Result<bool, String> {
        if let Object::Boolean(boolean) = self.pop()? {
            Ok(boolean.value)
        } else {
            Err("expected boolean".to_string())
        }
    }

    pub fn top(&self) -> Option<&Object> {
        self.pointer
            .checked_sub(1)
            .and_then(|i| self.stack[i].as_ref())
    }
}
