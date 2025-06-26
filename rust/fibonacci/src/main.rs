use std::env;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let number = &args[1];

    let n = number.parse::<u32>().expect("Please provide a positive integer number");
    let result = fibonacci(n);
    println!("The {n}th Fibonacci number is: {result}");
}
