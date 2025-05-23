fn fibonacci(n: u8) -> u8 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci sequence for small inputs:");
    for i in 0..=5 {
        let result = fibonacci(i);
        println!("fibonacci({}) = {}", i, result);
    }
}