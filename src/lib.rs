use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config,&str> {
    if args.len() < 3 {
      return Err("Not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {query, filename, case_sensitive})
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();

  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
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

  #[test]
  pub fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));

  }
  
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
    
    assert_eq!(
      vec!["Rust:", "Trust me"],
      search_case_insensitive(query, contents)
    );

  }
}
