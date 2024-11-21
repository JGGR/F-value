use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::model::core::*;
// Global shared state using Mutex for thread safety
pub static GLOBAL_STATE: Lazy<Mutex<Model>> = Lazy::new(|| Mutex::new(Model::new()));
