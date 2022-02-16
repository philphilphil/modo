mod common;
use std::path::Path;
use modo::todo::Todo;
use modo::md_handler;

#[test]
fn should_get_one_open_todo() {
    let temp_folder = common::create_md_test_files_1().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[0].done, false);
    temp_folder.close().unwrap();
}

#[test]
fn should_get_three_open_todos() {
    let temp_folder = common::create_md_test_files_3().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[2].done, false);
    temp_folder.close().unwrap();
}
