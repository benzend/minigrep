use std::error::Error;
use std::fs;
use std::env;


const FLAG: &str = "__";
const READ_FILE_FLAG: &str = "read-file";
const CHECK_IF_FILE_EXISTS_FLAG: &str = "check-file";
const SEARCH_FILE: &str = "search-file";

pub struct Config{
  pub flags: Vec<String>,
  pub non_flags: Vec<String>,
  pub filename: String,
  pub case_sensitive: bool,
  pub query: String
}

#[derive(Eq, PartialEq)]
enum ArgKind {
  FLAG,
  ARG
}

#[derive(Eq, PartialEq)]
pub struct LineArg {
  kind: ArgKind,
  arg: String
}

impl LineArg {
  pub fn new(arg: &String) -> LineArg {
    if is_flag(arg) {
      LineArg {
        kind: ArgKind::FLAG,
        arg: parse_flag(arg)
      }
    } else {
      LineArg {
        kind: ArgKind::ARG,
        arg: String::from(arg)
      }
    }
  }
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
      args.next();

      if args.len() < 2 {
          return Err("not enough arguments");
      }

      let args: Vec<LineArg> = args.map(|a| {
        LineArg::new(&a)
      }).collect();

      let mut non_flags = Vec::new();
      let mut flags = Vec::new();

      for arg in args {
        if arg.kind == ArgKind::ARG {
          non_flags.push(arg.arg);
        } else {
          flags.push(arg.arg);
        }
      }

      let filename = non_flags[0].clone();
      let query = if non_flags.len() == 2 { non_flags[1].to_string() } else { String::new() };
      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Config { 
        non_flags,
        flags, 
        case_sensitive,
        filename,
        query 
      })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  if config.flags.contains(&READ_FILE_FLAG.to_string()) {
      if config.non_flags.len() == 1 {

          let contents = fs
              ::read_to_string(config.filename)
              .expect("Something went wrong reading the file");

          for line in contents.lines() {
            println!("{}", line);
          }
      } else {
          println!("No file to read");
      }
  } else if config.flags.contains(&CHECK_IF_FILE_EXISTS_FLAG.to_string()) {
      if config.non_flags.len() == 1 {
          let exists = std::path::Path::new(&config.filename).exists();
          
          println!("File {} {} exist", config.filename, if exists { "does" } else { "doesn't" });
      } else {
          println!("No file to read");
      }
  } else if config.flags.contains(&SEARCH_FILE.to_string()) {
    if config.non_flags.len() == 2 {
      let contents = fs
        ::read_to_string(config.filename)
        .expect("file to exist");

      if config.case_sensitive {
        let found = search(&config.query, &contents);
        if found.len() > 0 {
          for line in found {
            println!("{}", line);
          }
        } else {
          println!("Couldn't find any matching text '{}'", &config.query);
        }
      } else {
        let found = search_case_insensitive(&config.query, &contents);
        if found.len() > 0 {
          for line in found {
            println!("{}", line);
          }
        } else {
          println!("Couldn't find any matching text '{}'", &config.query);
        }
      }
    }
  } else {
      println!("Not a valid flag");
  }

  Ok(())
}

fn is_flag(string: &String) -> bool {
  if &string[0..FLAG.len()] == FLAG {
      true
  } else {
      false
  }
}

fn parse_flag(flag: &String) -> String {
  flag[2..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_sensitive() {
      let query = "duct";
      let contents = "\
  Rust:
  safe, fast, productive.
  Pick three.
  Duct tape.";
      assert_eq!(vec!["safe, fast, produtive"], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}
