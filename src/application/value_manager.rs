use std::{
    fmt::Debug,
    io::{Read, Write},
};

use crate::application::json_file_io;
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

impl json_file_io::JSONFileIO for ValueManager {
    /*
     * Writes Instance of PathManager to json file already handles all serialization*/
    fn write_to_file(&self, path: &std::path::Path) -> Result<i32, String> {
        /*
         * God forgive me for my sins of being too much of a nester*/
        match path.parent() {
            Some(p) => {
                match std::fs::create_dir_all(p) {
                    Ok(_) => {
                        match std::fs::File::create(path) {
                            Ok(mut f) => {
                                match serde_json::to_string(self) {
                                    Ok(data_string) => {
                                        match f.write_all(data_string.as_bytes()) {
                                            Ok(_) => Ok(0),
                                            Err(e) => Err("Error at writing data to file: "
                                                .to_owned()
                                                + &e.to_string()),
                                        } //Match 5 End
                                    }
                                    Err(e) => Err("Error at serialization of PathManager: "
                                        .to_owned()
                                        + &e.to_string()),
                                } //Match 4 End
                            }
                            Err(e) => Err("Error at file creation: ".to_owned() + &e.to_string()),
                        } //Match 3 End
                    }
                    Err(e) => Err("Error at recurive directory creation after getting the parent directory: ".to_owned() + &e.to_string()),
                } //Match 2 End
            }
            None => Err("Error at parent directory creation: There is no parent directory specified in given path".to_owned()),
        } //Match 1 End
    }

    /*
     * The following function returns a PathManager, even if the given path does not exist.*/
    fn read_from_file(path: &std::path::Path) -> ValueManager {
        /*
         * These match statements are horrible*/
        match std::fs::File::open(path) {
            Ok(mut f) => {
                let mut strbuf = String::new();
                match f.read_to_string(&mut strbuf) {
                    Ok(_) => match serde_json::from_str(&strbuf) {
                        Ok(pm) => pm,
                        Err(_) => ValueManager::new(),
                    }, // Match 3 End
                    Err(_) => ValueManager::new(),
                } //Match 2 End
            }
            Err(_) => ValueManager::new(),
        } //Match 1 End
    }
}
