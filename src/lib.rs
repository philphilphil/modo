mod error;
pub mod filter;
mod predicate;
pub mod query;
use anyhow::Result;
use md_todo::todo::Todo;
use query::Query;
use std::path::Path;

use crate::error::InvalidQueryError;

pub fn modo(path: &Path, query: &str) -> Result<Vec<Todo>> {
    // Parse query
    let mut query = Query::new(query);
    if query.parse().is_err() {
        return Err(anyhow::Error::new(InvalidQueryError));
    }

    let mut todos = md_todo::get_todos_from_path(&path)?;
    filter::filter(&query, &mut todos);

    print!("{}", termion::cursor::Show);

    Ok(todos)
}
