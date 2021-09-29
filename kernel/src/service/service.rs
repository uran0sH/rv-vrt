use alloc::{string::String, sync::{Arc, Weak}, vec::Vec};

pub struct Service {
    pub path: String,
    pub parent: Option<Weak<Service>>,
    pub child: Vec<Arc<Service>>,
}

impl Service {
    pub fn new(path: String) -> Self {
        Self {
            path,
            parent: None,
            child: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_child(&mut self, another: Arc<Service>) {
        self.child.push(another.clone())
    }

    #[allow(dead_code)]
    pub fn remove_child(&mut self, another: Arc<Service>) {
        let mut index = -1isize;
        for (i, s) in self.child.iter().enumerate() {
            if s.path == another.path {
                index = i as isize;
            }
        }
        if index != -1 {
            self.child.remove(index as usize);
        }
    }
}