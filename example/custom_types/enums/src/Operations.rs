enum Operations {
    Add,
    Subtract,
    Mult,
    Div
}

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Mult => x * y,
            Self::Div => x / y,
        }
    }
}