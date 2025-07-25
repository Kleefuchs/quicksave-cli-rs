use std::{
    fmt::Debug,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueManager {
    pub paths: std::collections::HashMap<String, Box<std::path::Path>>,
}

impl ValueManager {
    pub fn new() -> ValueManager {
        ValueManager {
            paths: std::collections::HashMap::new(),
        }
    }

    pub fn add_path(&mut self, short_name: String, path: Box<std::path::Path>) {
        if !path.exists() {
            println!("Non existant path found for {short_name:?} : {path:?}");
        } else {
            self.paths.insert(short_name, path);
        }
    }
}
