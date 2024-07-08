use std::process;

pub struct Config<'a> {
  pub query: &'a String,
  pub file_path: &'a String
}

impl<'a> Config<'a> {
  pub fn build(args: &'a[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }
    Ok (
      Config {
        query: &args[1], 
        file_path: &args[2] 
      }
    )
  }
}

pub fn init_args(args: &[String]) -> Config {
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem processing args: {err}");
    process::exit(1);
  });

  config
}