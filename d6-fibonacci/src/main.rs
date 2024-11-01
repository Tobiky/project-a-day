use std::process::exit;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Calculates the N-th Fibonacci starting at 1 and 2
struct CLI {
    /// N-th Fibonacci number to calculate
    nth: usize,
}

fn main() {
    let cli = CLI::parse();

    let mut count = 0;
    let mut a: usize = 1;
    let mut b: usize = 2;

    while count < cli.nth {
        // Fibonacci
        a = a.checked_add(b).unwrap_or_else(|| {
            println!(
                "{:#}-th Fibonacci does not fit in calculation type",
                count + 1
            );
            exit(1);
        });

        // Swap so a < b
        a = b ^ a;
        b = a ^ b;
        a = b ^ a;

        count += 1;
    }

    println!("{:#}-nth Fibonacci: {:#}", cli.nth, b);
}
