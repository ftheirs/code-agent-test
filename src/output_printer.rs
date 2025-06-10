pub fn print_output(result: Result<f64, String>) {
    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }
}
