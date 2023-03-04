use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip 1st element
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query is missing."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path is missing."),
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
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "bold";
        let contents = "\
Rust, a language bold and new,
Memory-safe and fast, it's true.
With syntax clean and easy to view,
Coding in Rust, a pleasure to pursue.";
        assert_eq!(
            vec!["Rust, a language bold and new,"],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "BoLd";
        let contents = "\
Rust, a language bold and new,
Memory-safe and fast, it's true.
With syntax clean and easy to view,
Coding in Rust, a pleasure to pursue.";
        assert_eq!(
            vec!["Rust, a language bold and new,"],
            search_case_insensitive(query, contents)
        );
    }
}
