fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (query, filename) = parse_config(&args);
    let content = std::fs::read_to_string(filename).expect("Error reading the file");
    println!("FILENAME: {}", content);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}
