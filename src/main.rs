use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;
mod todo;
use todo::*;

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
fn load_data(dir: &Path, todos: &mut Vec<Todo>) -> io::Result<()> {
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

fn main() {
    let query: String = Input::new().with_prompt(".. where ").interact().unwrap();

    if query == "all" {
        loop {
            let mut todos: Vec<Todo> = vec![];
            load_data(Path::new("/Users/phil/TestingNotes"), &mut todos).unwrap();
            let todo_strings: Vec<String> = todos.iter().map(|t| t.to_string()).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose todo to toggle:")
                .clear(true)
                .default(0)
                .items(&todo_strings)
                .interact_opt()
                .unwrap();

            let selected_todo = &todos[selection.unwrap()];
            println!("Marked {} as {}!", selected_todo.name, if selected_todo.done { "open"} else { "done"});

            mark_as_done(&todos[selection.unwrap()]);
        }
    }
}

fn mark_as_done(todo: &Todo) {
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
