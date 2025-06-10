use clap::{{Arg, App}};

pub struct Input {
    pub x: f64,
    pub y: f64,
    pub operation: String,
}

pub fn parse_input() -> Result<Input, String> {
    let matches = App::new("Rust CLI Calculator")
        .version("0.1.0")
        .author("Your Name")
        .about("A simple calculator CLI application written in Rust")
        .arg(
            Arg::new("x")
                .required(true)
                .help("The first number"),
        )
        .arg(
            Arg::new("y")
                .required(true)
                .help("The second number"),
        )
        .arg(
            Arg::new("operation")
                .required(true)
                .help("The operation to perform (+, -, *, /)"),
        )
        .get_matches();

    let x_str = matches.value_of("x").unwrap();
    let y_str = matches.value_of("y").unwrap();
    let operation = matches.value_of("operation").unwrap().to_string();

    let x = x_str.parse::<f64>().map_err(|_| "error: invalid number".to_string())?;
    let y = y_str.parse::<f64>().map_err(|_| "error: invalid number".to_string())?;

    Ok(Input { x, y, operation })
}
