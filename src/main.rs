fn fibonacci(n: u8) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let inputs: [u8; 4] = [0, 1, 5, 10];

    for input in inputs.iter() {
        let result = fibonacci(*input);
        println!("fibonacci({}) = {}", input, result);
    }
}
