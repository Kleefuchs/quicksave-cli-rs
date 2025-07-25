
mod value_manager;

pub struct Application<'a> {
    args: Vec<String>,
    value_manager: value_manager::ValueManager,
    paths_file_path: &'a std::path::Path,
}

impl Application<'_> {
    pub fn new(args: Vec<String>, paths_file_path: &std::path::Path) -> Application {
        let mut strbuf = String::new();
        Application {
            args,
            value_manager: match serde_json_file_io::read_from_file::<value_manager::ValueManager>(paths_file_path, &mut strbuf) {
                Ok(vm) => {
                    vm
                },
                Err(e) => {
                    println!("{}", e);
                    value_manager::ValueManager::new()
                },
            },
            paths_file_path,
        }
    }

    fn list(&self) -> Result<String, String> {
        for path_entry in &self.value_manager.paths {
            println!("{path_entry:?}");
        }
        Ok("Listing file entries in ValueManager".to_owned())
    }

    fn add(&mut self, short_name: String, path: &std::path::Path) -> Result<String, String> {
        self.value_manager.add_path(short_name, path.into());
        Ok("Added path to ValueManager".to_owned())
    }

    fn rm(&mut self, short_name: String) -> Result<String, String> {
        match self.value_manager.paths.remove(&short_name) {
            Some(_) => {
                Ok("The following entry has been removed successfully: ".to_owned() + &short_name)
            }
            None => Err(
                "The following entry has not been found in ValueManager: ".to_owned() + &short_name,
            ),
        }
    }

    fn get(&self, short_name: String) -> Result<i32, String> {
        match self.value_manager.paths.get(&short_name) {
            Some(path) => {
                println!("{}", path.display());
                Ok(0)
            }
            None => Err(
                "Path not found in ValueManager try the list command. If command is not in list try adding it"
                    .to_owned(),
            ),
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        if self.args.len() < 2 {
            return Err("A minimum of 1 extra argument is required. Try help subcommand".to_owned());
        }

        match self.args[1].as_str() {
            "list" => self.list(),
            "add" => {
                if self.args.len() < 4 {
                    return Err("Subcommand \"add\" requires 2 extra arguments".to_owned());
                }

                let name_argument: String = self.args[2].clone();
                let path_argument: String = self.args[3].clone();
                self.add(name_argument, std::path::Path::new(&path_argument))
            }
            "rm" => {
                if self.args.len() < 3 {
                    return Err("Subcommand \"rm\" requires 1 extra argument".to_owned());
                }
                let name_argument: String = self.args[2].clone();
                match self.rm(name_argument) {
                    Ok(sstring) => Ok(sstring),
                    Err(estring) => Err(estring),
                }
            }
            "get" => {
                if self.args.len() < 3 {
                    return Err("Subcommand \"add\" requires 1 extra argument".to_owned());
                }

                let name_argument: String = self.args[2].clone();
                match self.get(name_argument) {
                    Ok(_) => Ok("".to_owned()),
                    Err(e) => Err(e),
                }
            }
            _ => Err("Unknown subcommand".to_owned()),
        }
    }

    pub fn save_paths(&self) -> Result<String, String> {
        match serde_json_file_io::write_to_file(&self.value_manager, self.paths_file_path) {
            Ok(_) => {
                Ok("".to_owned())
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
