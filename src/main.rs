fn main() {
    let fibonacci = fibonacci_sequence::Fibonacci::new();

    for num in fibonacci.take(10) {
        println!("{}", num);
    }
}
