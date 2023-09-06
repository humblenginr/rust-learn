use std::error::Error;
use std::{env, fs};

// &str is the reference to immutable string slice stored in stack memory or static memory
// String is used when memory is allocated in the heap and you want to own and modify it

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
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            case_insensitive_search(query, contents)
        );
    }
}

pub fn case_insensitive_search(query: &str, contents: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line.to_string());
        }
    }
    result
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.to_string());
        }
    }
    result
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        for line in case_insensitive_search(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &content) {
            println!("{line}");
        }
    }

    Ok(())
}

pub struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search_string = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: search_string,
            file_path,
            ignore_case,
        })
    }
}
