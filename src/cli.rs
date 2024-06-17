use clap::Parser;
use crate::calculator::{Calculator};

#[derive(Debug, Parser)]
struct Args {
    #[arg(help = "")]
    addend1: f64,
    #[arg(help = "")]
    addend2: f64,
}

pub fn start() {
    let args = Args::parse();
    let calculator = Calculator::new(args.addend1, args.addend2);
    println!("{}", calculator.calc());
}
