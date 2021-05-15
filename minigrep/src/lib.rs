use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// Note: using primitive values when a complex type would be more appropriate is an anti-pattern knwon as primitive obsession.
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // If the slice isn't long enough, the program panics and displays a better error message than the
        // `index out of bounds` message.
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        // When we printed the vector, the program's name takes up the first value in the vector at args[0], so we're starting at the
        // index 1. 
        let query = args[1].clone();
        let filename = args[2].clone();

        // Checking for an environment variable named CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Extracting a run function containing the rest of program logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something
    // goes wrong. This will let us further consolidate into main the logic around handling errors in a user-friendly way.
    let contents = fs::read_to_string(config.filename)?;

    // Check the case_sensitive field's value and use that to decide whether to call the search function or the
    // search_case_insensitive function
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Search query's content in file
    for line in results {
        println!("{}", line);
    }

    Ok(())
}


pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A failing test for the search function
    fn search_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    // Failing test for case-sensitive search function
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

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
            search_case_insensitive(query, contents)
        );
    }
}