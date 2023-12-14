use clap::Parser;

#[derive(Parser)]
struct Args {
    operand1: f32,
    operation: String,
    operand2: f32,
}

fn main() {
    let args = Args::parse();
    let (a, b) = (args.operand1, args.operand2);
    let ans = 
    match args.operation.as_str() {
        "+" => a+b,
        "-" => a-b,
        "*" => a*b,
        "/" => a/b,
        "%" => a%b,
        _ => panic!("Please enter a valid operation (+, -, *, /, %)")
    };
    print!("Your answer is {ans}")
}