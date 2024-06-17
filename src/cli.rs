use clap::Parser;
use crate::calculator::{Calculator};

/// A structure which captures the input arguments.
///
/// # Usage
///
/// ```
/// let args = Args::parse();
/// ```
#[derive(Debug, Parser)]
struct Args {
    /// The first number to add.
    #[arg(help = "The first number to add.")]
    addend1: f64,
    /// The second number to add.
    #[arg(help = "The second number to add.")]
    addend2: f64,
}

/// Begins the addition operation based on command line arguments. 
///
/// It uses the `Args` struct to parse the arguments, creates a new `Calculator`, 
/// and prints the result of the addition.
///
/// # Example
///
/// If you were to run your program with the following command:
///
/// ```
/// cargo run -- 1 2
/// ```
///
/// It would output `3` to the console.
/// ```
/// cargo run -- 1.0 2.0
/// ```
///
/// It would output `3.0` to the console.
pub fn start() {
    let args = Args::parse();
    let calculator = Calculator::new(args.addend1, args.addend2);
    println!("{}", calculator.calc());
}