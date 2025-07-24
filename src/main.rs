
mod application;

fn main() {

    match dirs::data_local_dir() {
        Some(local_dir) => {
            let paths_file_path = local_dir.join("cdmark/path.json");
            let mut application = application::Application::new(std::env::args().collect(), &paths_file_path);
            match application.run() {
                Ok(sstring) => {
                    println!("{}", sstring);
                },
                Err(estring) => {
                    panic!("{}", estring);
                }
            };

            match application.save_paths() {
                Ok(sstring) => {
                    println!("{}", sstring);
                },
                Err(estring) => {
                    panic!("{}", estring);
                }
            }
        },
        None => {
            panic!("Local directory not found");
        }
    }
}
