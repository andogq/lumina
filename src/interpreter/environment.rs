use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::object::Object;

#[derive(Clone, Debug)]
struct Inner {
    env: HashMap<String, Object>,
    parent: Option<Environment>,
}

impl Inner {
    pub fn new() -> Self {
        Inner {
            env: HashMap::new(),
            parent: None,
        }
    }

    pub fn extend(env: &Environment) -> Self {
        let mut s = Self::new();

        s.parent = Some(env.clone());

        s
    }
}

#[derive(Clone, Debug)]
pub struct Environment(Rc<RefCell<Inner>>);

impl Environment {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Inner::new())))
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<Object> {
        let inner = self.0.borrow();

        inner
            .env
            .get(key.as_ref())
            .cloned()
            .or_else(|| inner.parent.as_ref().and_then(|parent| parent.get(key)))
    }

    pub fn set(&self, key: impl ToString, value: Object) {
        self.0.borrow_mut().env.insert(key.to_string(), value);
    }

    pub fn nest(&self) -> Self {
        Self(Rc::new(RefCell::new(Inner::extend(&self))))
    }
}
