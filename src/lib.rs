use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config,&str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config {query, filename})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("Text:\n{}", contents);

  Ok(())

}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn args_len_check () {
    let string1 = String::from("a");
    let string2 = String::from("b");
    let array = [string1, string2];
    match Config::new(&array) {
      Ok(_) => panic!("Not okay. Should give an arg error"),
      _ => (),
    };
  }

  #[test]
  pub fn args_valid_check () -> Result<(), Box<dyn Error>>{
    let arg1 = String::from("program");
    let arg2 = String::from("query");
    let arg3 = String::from("filename");
    let args = [arg1, arg2, arg3];
    let config = Config::new(&args)?;
    assert_eq!(config.query, args[1]);
    assert_eq!(config.filename, args[2]);
    Ok(())
  }
}
