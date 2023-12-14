use clap::Parser;

#[derive(Parser)]
struct Args {
    operation: String,
    operand1: f32,
    operand2: f32,
}

fn main() {
    println!("Hello, world!");
}