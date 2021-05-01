use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

// Note: using primitive values when a complex type would be more appropriate is an anti-pattern knwon as primitive obsession.
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // If the slice isn't long enough, the program panics and displays a better error message than the
        // `index out of bounds` message.
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        // When we printed the vector, the program's name takes up the first value in the vector at args[0], so we're starting at the
        // index 1. 
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Extracting a run function containing the rest of program logic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something
    // goes wrong. This will let us further consolidate into main the logic around handling errors in a user-friendly way.
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // A failing test for the search function
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}