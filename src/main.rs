use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
