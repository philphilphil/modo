use std::io;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use crate::todo::Todo;

fn read_md_file(file: &DirEntry, todos: &mut Vec<Todo>) {
    if file.file_name().to_str().unwrap().ends_with("md") {
        let data = fs::read_to_string(file.path()).expect("Unable to read file");

        let mut line_no = 1;
        for line in data.lines() {
            if line.starts_with("- [x]") {
                let todo = Todo::new(
                    &line[6..line.len()],
                    file.file_name().to_str().unwrap(),
                    line_no,
                    true,
                    file.path().to_str().unwrap(),
                );
                todos.push(todo);
            } else if line.starts_with("- [ ]") {
                let todo = Todo::new(
                    &line[6..line.len()],
                    file.file_name().to_str().unwrap(),
                    line_no,
                    false,
                    file.path().to_str().unwrap(),
                );
                todos.push(todo);
            }
            line_no += 1;
        }
    }
}

// Itterate all directorys and get md files
pub fn load_data(dir: &Path, todos: &mut Vec<Todo>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                load_data(&path, todos)?;
            } else {
                read_md_file(&entry, todos);
            }
        }
    }
    Ok(())
}

pub fn mark_as_done(todo: &Todo) {
    let data = fs::read_to_string(Path::new(&todo.filepath)).expect("Unable to read file");
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

    fs::write(&todo.filepath, new_data).expect("Unable to write file");
}
