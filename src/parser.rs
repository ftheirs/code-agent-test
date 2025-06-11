se clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(help = "The first number")]
    pub num1: f64,

    #[clap(help = "The operator (+, -, *, /)")]
    pub operator: char,

    #[clap(help = "The second number")]
    pub num2: f64,
}
