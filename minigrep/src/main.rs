use std::env;

fn main() {
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection
    // such as a vector, containing all the elements the iterator produces.
    let args: Vec<String> = env::args().collect();
    
    // Saves the values of the two arguments in variables so we can use the values throughout the rest of the program.
    // When we printed the vector, the program's name takes up the first value in the vector at args[0], so we're starting at the
    // index 1. 
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
