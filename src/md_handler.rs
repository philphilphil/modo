use crate::todo::Todo;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

fn read_md_file(file: &DirEntry, todos: &mut Vec<Todo>) -> io::Result<()> {
    if file.file_name().to_str().unwrap().ends_with("md") {
        let data = fs::read_to_string(file.path())?;

        let mut line_no = 0;
        for line in data.lines() {
            line_no += 1;
            let line_trimed = line.trim_start();

            if !line_trimed.starts_with("- [x]") && !line_trimed.starts_with("- [ ]") {
                continue;
            }

            let done = if line_trimed.starts_with("- [x]") {
                true
            } else {
                false
            };

            let first_heading = get_first_heading(&data, line_no);
            let todo = Todo::new(
                &line_trimed[6..line_trimed.len()],
                file.file_name().to_str().unwrap(),
                line_no,
                done,
                file.path().to_path_buf(),
                first_heading.to_lowercase(),
            );

            todos.push(todo);
        }
    }

    Ok(())
}

fn get_first_heading(data: &String, todo_line_no: i32) -> String {
    // TODO: Search in other direction
    let mut line_no = 1;
    let mut heading = String::from("");
    for line in data.lines() {
        if line_no == todo_line_no {
            break;
        }

        if line.trim_start().starts_with("#") {
            heading = line.trim_start().to_string();
        }

        line_no += 1;
    }

    heading
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
                read_md_file(&entry, todos)?;
            }
        }
    }
    Ok(())
}

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
