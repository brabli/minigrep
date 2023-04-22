use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lower))
        .collect()
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
