use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    // such as a vector, containing all the elements the iterator produces.
    let args: Vec<String> = env::args().collect();

    // Saves the values of the two arguments in variables so we can use the values throughout the rest of the program.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // A nonzero exit status is a convention to signal to the process that called our program that the program exited with an
        // error state
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Reading the file
    if let Err(e) = minigrep::run(config) {
        println!("Application erro: {}", e);

        process::exit(1);
    };
}
