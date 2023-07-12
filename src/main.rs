use std::fmt::{write, Display, Formatter, Result};

enum MessageOne {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Display for MessageOne {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MessageOne::Quit => write!(f, "Quit"),
            MessageOne::Move { x, y } => write!(f, "Move to ({}, {})", x, y),
            MessageOne::Write(text) => write!(f, "Write: {}", text),
            MessageOne::ChangeColor(r, g, b) => {
                write!(f, "Change color to RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

fn show_message(msg: MessageOne) {
    println!("{}", msg);
}

fn main() {
    show_message(MessageOne::Move { x: 1, y: 2 })
}
