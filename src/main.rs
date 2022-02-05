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
    let mut todos: Vec<Todo> = vec![];

    load_data(Path::new("/Users/phil/TestingNotes"), &mut todos).unwrap();

    // for t in todos {
    //     println!(
    //         "Todo: {}  Done:   {}  Line:   {}  Path    {}",
    //         t.name, t.done, t.line_no, t.filepath
    //     );
    // }

    let todo_strings:Vec<String> = todos.iter().map(|p| p.name.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Tick todo:")
        .default(0)
        .items(&todo_strings)
        .interact()
        .unwrap();

    println!("Marked {} as done!", todos[selection].name);
}
