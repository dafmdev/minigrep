use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("The query: {}", query);
    println!("The file_path: {}", file_path);
}
