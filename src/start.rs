use std::fs;
use std::env;

use config::{
  Config,
  init_args
};

pub mod config;

pub fn start() {
  let args: Vec<String> = env::args().collect();

  let config = init_args(&args);

  run_config(config);
}

fn run_config(config: Config) {
  let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

  // Alternative ways to write the above

  // let contents: String = fs::read_to_string(file_path).unwrap_or_else(|error: std::io::Error| {
  //     eprintln!("Problem reading: {error}");
  //     format!("Error: {error}").to_string()
  // });
  // let contents = fs::read_to_string(file_path);
      // .unwrap_or_else(|error: std::io::Error| {
      //     eprintln!("Problem reading: {error}");
      //     return format!("Error: {error}").to_string();
      // });
  
  // let contents = match fs::read_to_string(file_path) {
  //     Ok(t) => t,
  //     Err(e) => {
  //         eprintln!("Error found in e-print: {e}");
  //         format!("Error returned: {e}").to_string()
  //     }
  // };

  println!("With text:\n{contents}");
}