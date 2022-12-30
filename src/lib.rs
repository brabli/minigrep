use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_text = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_text)
    } else {
        search(&config.query, &file_text)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn one_result() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENT));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        assert_eq!(Vec::<&str>::new(), search(query, CONTENT));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, CONTENT));
    }
}
