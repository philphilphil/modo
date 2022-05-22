use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

use crate::md_writer;

#[derive(Debug, Clone)]
pub struct Todo {
    pub name: String,
    pub filter_name: String,
    pub filename: String,
    pub line_no: u32,
    pub done: bool,
    pub filepath: PathBuf,
    pub filepath_as_string: String,
    pub heading: String,
}

impl Todo {
    // Construct person
    pub fn new(
        name: &str,
        filename: &str,
        line_no: u32,
        done: bool,
        filepath: PathBuf,
        heading: String,
    ) -> Todo {
        Todo {
            name: name.to_string(),
            filter_name: name.to_lowercase(),
            filename: filename.to_string(),
            line_no,
            done,
            filepath_as_string: filepath.to_str().unwrap().to_string().to_lowercase(),
            filepath,
            heading,
        }
    }
}

impl Todo {
    pub fn toggle(&mut self) {
        md_writer::toggle_todo(self).unwrap();
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut todo_state = "[ ]";
        if self.done {
            todo_state = "[x]"
        }
        write!(f, "{} {}", todo_state, self.name)
    }
}
