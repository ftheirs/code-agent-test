fn fibonacci(n: u8) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("Fibonacci number at position {} is: {}", n, result);
}