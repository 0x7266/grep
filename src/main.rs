fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let content = std::fs::read_to_string(&args[2]).expect("Error reading the file");
    println!("FILENAME: {}", content);
}
