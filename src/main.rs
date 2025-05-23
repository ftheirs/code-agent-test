n fibonacci(n: u8) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            let mut result: u64 = 0;
            for _ in 2..=n {
                result = a + b;
                a = b;
                b = result;
            }
            result
        }
    }
}

fn main() {
    let values_to_test = [3, 5, 8, 10];
    for n in values_to_test.iter() {
        let result = fibonacci(*n);
        println!("Fibonacci({}) = {}", n, result);
    }
}
