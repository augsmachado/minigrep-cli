use std::env;
use std::fs;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    // such as a vector, containing all the elements the iterator produces.
    let args: Vec<String> = env::args().collect();

    // Saves the values of the two arguments in variables so we can use the values throughout the rest of the program.
    let (query, filename) = parse_config(&args);
    
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // Reading the file
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

}

fn parse_config (args: &[String]) -> (&str, &str) {
    // When we printed the vector, the program's name takes up the first value in the vector at args[0], so we're starting at the
    // index 1. 
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}