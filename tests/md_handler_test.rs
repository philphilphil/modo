mod common;
use modo::{md_handler, todo::Todo};

#[test]
fn test_single_file() {
    let temp_folder = common::create_md_test_file_with_1_open_todo(1).unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[0].done, false);
    assert_eq!(todos[0].filename, "file0.md");
    temp_folder.close().unwrap();
}

#[test]
fn test_three_files() {
    let temp_folder = common::create_md_test_file_with_1_open_todo(3).unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[2].done, false);
    temp_folder.close().unwrap();
}

#[test]
fn test_single_files_multiple_todos() {
    let temp_folder = common::create_md_test_file_with_5_todos_4_open(1).unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 5);
    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);
    temp_folder.close().unwrap();
}

#[test]
fn test_single_file_with_headers() {
    let temp_folder = common::create_md_test_file_with_5_todos_3_open_headers().unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 5);
    assert_eq!(todos[0].first_heading, "# Heading2");
    assert_eq!(todos[0].done, false);
    assert_eq!(todos[0].name, "Todo with Heading");
    assert_eq!(todos[0].filename, "file1.md");
    assert_eq!(todos[0].line_no, 3);

    assert_eq!(todos[1].first_heading, "# Heading2");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[1].name, "A open todo!");
    assert_eq!(todos[1].filename, "file1.md");
    assert_eq!(todos[1].line_no, 4);

    assert_eq!(todos[2].first_heading, "### Heading Done");
    assert_eq!(todos[2].done, true);
    assert_eq!(todos[2].name, "A done todo!");
    assert_eq!(todos[2].filename, "file1.md");
    assert_eq!(todos[2].line_no, 7);

    assert_eq!(todos[3].first_heading, "### Heading Done");
    assert_eq!(todos[3].done, true);
    assert_eq!(todos[3].name, "A done todo!");
    assert_eq!(todos[3].filename, "file1.md");
    assert_eq!(todos[3].line_no, 12);

    temp_folder.close().unwrap();
}

#[test]
fn test_multiple_files_multiple_todos() {
    let temp_folder = common::create_md_test_file_with_5_todos_4_open(2).unwrap();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 10);
    //assert eq mit map/clojure

    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);
    temp_folder.close().unwrap();
}

#[test]
fn test_multiple_files_and_folders_multiple_todos() {
    let temp_folder = common::create_folder_with_2_files_10_todos_8_open().unwrap();
    let temp_folder_depth1 =
        common::create_folder_with_2_files_10_todos_8_open_in(&temp_folder).unwrap();
    let _temp_folder_depth1_2 =
        common::create_folder_with_2_files_10_todos_8_open_in(&temp_folder).unwrap();
    let _temp_folder_depth2 =
        common::create_folder_with_2_files_10_todos_8_open_in(&temp_folder_depth1).unwrap();

    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(temp_folder.path(), &mut todos).unwrap();
    assert_eq!(todos.len(), 40);
    temp_folder.close().unwrap(); //deletes other temp folders with it
}

// TODO: test multiple_files_multiple_folders2, very deep
