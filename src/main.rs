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
        "p" => {
            pow(a as u32, b as u32) as f32
        }
        _ => panic!("Please enter a valid operation (+, -, *, /, %, p)")
    };
    println!("Your answer is {ans}")
}

fn pow(b:u32, e:u32) -> u32 {
    let mut ans:u32 = b;
    for _ in 0..e-1 {
        ans = ans * b;
    }
    ans
}