use minigrep::Input;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Input = Input::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1) // exit without panic
    });
    if let Err(err) = minigrep::run(input) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
