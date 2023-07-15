use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    let contents: String = fs::read_to_string(file_path).expect("Check if the filepath is correct");

    println!("The query: {}", query);
    println!("The text:\n {}", contents);
}
