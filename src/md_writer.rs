use crate::todo::Todo;
use std::fs;
use std::io;
use std::path::Path;

pub fn toggle_todo(todo: &Todo) -> io::Result<()> {
    let data = fs::read_to_string(Path::new(&todo.filepath))?;
    let mut new_data: String = String::new();

    let mut line_no = 1;
    for line in data.lines() {
        if line_no == todo.line_no {
            if todo.done {
                new_data += &line.replace("- [x]", "- [ ]");
            } else {
                new_data += &line.replace("- [ ]", "- [x]");
            }
        } else {
            new_data += line;
        }
        new_data += "\n";
        line_no += 1;
    }

    fs::write(&todo.filepath, new_data)?;

    Ok(())
}
