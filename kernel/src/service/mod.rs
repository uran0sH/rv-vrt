mod service;

use alloc::string::String;
use hashbrown::HashMap;
use spin::Mutex;
use crate::task::PidHandle;
use lazy_static::*;

pub use service::Service;

pub struct Registry {
    pub table: Mutex<HashMap<String, usize>> // Service.path, PidHandle
}

impl Registry {
    pub fn new() -> Self {
        Self {
            table: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, pid: &PidHandle, service: &Service) {
        self.table.lock().insert(service.path.clone(), pid.0);
    }

    #[allow(dead_code)]
    pub fn remove(&self, service: &Service) {
        self.table.lock().remove(&service.path);
    }

    pub fn find_task(&self, service: &Service) -> usize {
        *self.table.lock().get(&service.path).unwrap()
    }
}

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}
