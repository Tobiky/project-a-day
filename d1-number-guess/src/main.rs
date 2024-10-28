use std::io::{self, BufRead, Read, Write};

use fastrand;

fn main() {
    let number = fastrand::u8(..);
    println!("Please guess the number! The number is inclusive between 0 and 255.");

    let mut input = String::with_capacity(3);
    loop {
        input.clear();
        print!(": ");
        let _ = io::stdout().flush();

        match io::stdin().lock().take(4).read_line(&mut input) {
            Err(error) => {
                eprintln!("Problems reading input! (Error: {error})");
                std::process::exit(1);
            }
            _ => (),
        };

        if input.trim().is_empty() {
            break;
        }

        let guess: u8 = match str::parse(&input[..input.len() - 1]) {
            Err(error) => {
                eprintln!("Unable to understand number! (Error: {error})");
                continue;
            }
            Ok(value) => value,
        };

        if guess < number {
            println!("Too low, guess again!");
        } else if guess > number {
            println!("Too high, guess again!");
        } else {
            println!("Correct!");
            break;
        }
    }
}
