mod error;
pub mod filter;
pub mod md_reader;
pub mod md_writer;
mod predicate;
pub mod query;
pub mod todo;
use anyhow::Result;
use query::Query;
use std::path::{Path, PathBuf};
use todo::Todo;

use crate::error::InvalidQueryError;

pub fn modo(path: &Path, query: &str) -> Result<Vec<Todo>> {
    // Parse query
    let mut query = Query::new(query);
    if query.parse().is_err() {
        return Err(anyhow::Error::new(InvalidQueryError));
    }

    // println!("DEBUG: {:#?}", query.predicates);
    // loop {
    let mut todos: Vec<Todo> = vec![];
    md_reader::load_todos_from_dir(Path::new(&path), &mut todos)?;

    //println!("DEBUG: {:#?}", todos);
    //println!("DEBUG: Todo count: {}", todos.len());
    filter::filter(&query, &mut todos);
    todos.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    todos.sort_by(|a, b| a.done.cmp(&b.done));
    //println!("DEBUG: Todo count after filter: {}", todos.len());

    if todos.is_empty() {
        println!("No todos found.");
        // Ok(todo_strings);
    }

    // todo_strings = todos.iter().map(|t| t.to_string()).collect();
    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Choose todo to toggle:")
    //     .clear(true)
    //     .default(0)
    //     .items(&todo_strings)
    //     .interact_opt()
    //     .expect("Issue initializing selector.");

    // if let Some(selection) = selection {
    //     let selected_todo = &todos[selection];

    //     md_writer::toggle_todo(&todos[selection])?;

    //     println!(
    //         "Marked {} as {}.",
    //         selected_todo.name,
    //         if selected_todo.done { "open" } else { "done" }
    //     );
    // } else {
    //     break;
    // }
    // }

    Ok(todos)
}
