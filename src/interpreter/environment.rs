use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use super::object::Object;

pub struct Environment {
    env: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }
}

impl Deref for Environment {
    type Target = HashMap<String, Object>;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl DerefMut for Environment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env
    }
}
