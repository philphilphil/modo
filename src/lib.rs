pub mod filter;
pub mod md_handler;
mod predicate;
pub mod query;
pub mod todo;
use dialoguer::{theme::ColorfulTheme, Select};
use query::Query;
use std::path::{Path, PathBuf};
use todo::Todo;

pub fn modo(path: PathBuf, query: String) {
    let mut query = Query::new(&query);
    if query.parse().is_err() {
        println!("{}", "Invalid query.");
        return;
    }

    // println!("DEBUG: {:#?}", query.predicates);
    loop {
        let mut todos: Vec<Todo> = vec![];
        md_handler::load_data(Path::new(&path), &mut todos)
            .expect("Something went wrong reading the notes");

        //println!("DEBUG: {:#?}", todos);
        //println!("DEBUG: Todo count: {}", todos.len());
        filter::filter(&query, &mut todos);
        //println!("DEBUG: Todo count after filter: {}", todos.len());

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
            .expect("Issue initializing selector.");

        if let Some(selection) = selection {
            let selected_todo = &todos[selection];

            md_handler::toggle_todo(&todos[selection])
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
