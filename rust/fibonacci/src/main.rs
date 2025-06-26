use std::env;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let number = &args[1];

    let n = number.parse::<u32>().expect("Please provide a valid number");
    let result = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, result);
}
