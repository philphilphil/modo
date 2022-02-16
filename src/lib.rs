mod clause;
pub mod md_handler;
pub mod query;
pub mod todo;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use query::Query;
use std::path::Path;
use crate::todo::Todo;

pub fn modo() {

        //println!("\x1B[2J\x1B[1;1H");
        let user_input: String = Input::new()
            .with_prompt(".. where ")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        let mut query = Query::new(&user_input);
        if !query.parse() {
            println!("{}", "Invalid query.");
            return;
        }

        println!("{:?}", query.clauses);

        let mut todos: Vec<Todo> = vec![];
        
        md_handler::load_data(Path::new("/Users/phil/TestddingNotes"), &mut todos)
            .expect("Something went wrong reading the notes");

        loop {
            if todos.len() < 1 {
                println!("{}", "No todos found.");
                break;
            }

            let todo_strings: Vec<String> = todos.iter().map(|t| t.to_string()).collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose todo to toggle:")
                .clear(true)
                .default(0)
                .items(&todo_strings)
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                let selected_todo = &todos[selection];

                md_handler::mark_as_done(&todos[selection])
                    .expect("Something went wront writing back to the md file.");

                println!(
                    "Marked {} as {}!",
                    selected_todo.name,
                    if selected_todo.done { "open" } else { "done" }
                );
            } else {
                break;
            }
        }
    
}