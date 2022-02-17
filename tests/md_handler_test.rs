mod common;
use modo::{todo::Todo, md_handler};

#[test]
fn test_single_file() {
    let temp_folder = common::create_md_test_files_1().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[0].done, false);
    temp_folder.close().unwrap();
}

#[test]
fn test_three_files() {
    let temp_folder = common::create_md_test_files_2().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[2].done, false);
    temp_folder.close().unwrap();
}

#[test]
fn test_single_files_multiple_todos() {
    let temp_folder = common::create_md_test_files_3().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 5);
    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);
    temp_folder.close().unwrap();
}

#[test]
fn test_multiple_files_multiple_todos() {
    let temp_folder = common::create_md_test_files_4().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 10);
    //assert eq mit map/clojure

    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);
    temp_folder.close().unwrap();
}

// TODO: Test multiple_files_multiple_folders
// TODO: test multiple_files_multiple_folders2, very deep