use std::fmt::{Display,Formatter,Result};

pub struct Todo {
    pub name: String,
    pub filename: String,
    pub line_no: i32,
    pub done: bool,
    pub filepath: String
}

impl Todo {
    // Construct person
    pub fn new(name: &str, filename: &str, line_no: i32, done: bool, filepath: &str) -> Todo {
        Todo {
            name: name.to_string(),
            filename: filename.to_string(),
            line_no: line_no,
            done: done,
            filepath: filepath.to_string()
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut todo_state = "➔";
        if self.done {
            todo_state = "✅"
        }
        write!(f, "{} {}", todo_state, self.name)
    }
}