pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn calculate(&self, x: f64, y: f64) -> Result<f64, String> {
        match self {
            Operation::Add => Ok(x + y),
            Operation::Subtract => Ok(x - y),
            Operation::Multiply => Ok(x * y),
            Operation::Divide => {
                if y == 0.0 {
                    Err("error: division by zero".to_string())
                } else {
                    Ok(x / y)
                }
            }
        }
    }
}

pub fn parse_operator(op: &str) -> Result<Operation, String> {
    match op {
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Subtract),
        "*" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        _ => Err("error: invalid operator".to_string()),
    }
}
