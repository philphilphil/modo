mod common;
use common::md_test_file_creator;
use md_todo::todo::Todo;
use tempfile::TempDir;

#[test]
fn test_parse_single_file() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[0].done, false);
    assert_eq!(todos[0].filename, "file0.md");

    dir.close().unwrap();
}

#[test]
fn test_tick_single_file() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos[0].done, false);

    md_todo::toggle_todo(&mut todos[0]).unwrap();
    todos = vec![];
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos[0].done, true);

    dir.close().unwrap();
}

#[test]
fn test_parse_single_complex_file() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::complex_23_todos_15_open(&dir, "file0.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 23);

    dir.close().unwrap();
}

#[test]
fn test_parse_three_files() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file2.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 3);
    let open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 3);
    assert_eq!(todos[0].name, "A single open todo!");
    assert_eq!(todos[2].done, false);

    dir.close().unwrap();
}

#[test]
fn test_parse_multiple_file_types() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file0.md").unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file1.txt").unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file2.mad").unwrap();
    md_test_file_creator::simple_1_open_todo(&dir, "file3.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let closed_todos_count = todos.iter().filter(|t| t.done).count();
    assert_eq!(closed_todos_count, 0);
    assert_eq!(todos.len(), 2); // only md files are read

    dir.close().unwrap();
}

#[test]
fn test_parse_single_files_multiple_todos() {
    let mut todos: Vec<Todo> = vec![];
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file_123_323.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 5);
    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);

    dir.close().unwrap();
}

#[test]
fn test_parse_single_file_with_headers() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file1.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 5);
    assert_eq!(todos[0].headings[0], "# Heading2");
    assert_eq!(todos[0].done, false);
    assert_eq!(todos[0].name, "Todo with Heading");
    assert_eq!(todos[0].filename, "file1.md");
    assert_eq!(todos[0].line_no, 3);

    assert_eq!(todos[1].headings[0], "# Heading2");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[1].name, "A open todo!");
    assert_eq!(todos[1].filename, "file1.md");
    assert_eq!(todos[1].line_no, 4);

    assert_eq!(todos[2].headings[0], "### Heading Done");
    assert_eq!(todos[2].done, true);
    assert_eq!(todos[2].name, "A done todo!");
    assert_eq!(todos[2].filename, "file1.md");
    assert_eq!(todos[2].line_no, 7);

    assert_eq!(todos[3].headings[0], "### Heading Done");
    assert_eq!(todos[3].done, true);
    assert_eq!(todos[3].name, "A done todo!");
    assert_eq!(todos[3].filename, "file1.md");
    assert_eq!(todos[3].line_no, 12);

    dir.close().unwrap();
}

#[test]
fn test_parse_and_tick_multiple_files_multiple_todos() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file_123_323.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "afile_123_323.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 10);
    //assert eq mit map/clojure
    assert_eq!(todos[0].name, "A open todo!");
    assert_eq!(todos[1].done, false);
    assert_eq!(todos[2].done, true);

    let mut open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 8);

    md_todo::toggle_todo(&mut todos[1]).unwrap();
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    md_todo::toggle_todo(&mut todos[2]).unwrap();
    let todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos[1].done, true);
    assert_eq!(todos[2].done, false);
    open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 8); // should not change because changed one done one open

    dir.close().unwrap();
}

#[test]
fn test_parse_multiple_files_and_folders_multiple_todos() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file2.md").unwrap();
    let dir_depth1 = TempDir::new_in(&dir).unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir_depth1, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir_depth1, "file2.md").unwrap();
    let _dir_depth1_2 = TempDir::new_in(&dir).unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file2.md").unwrap();
    let _dir_depth2 = TempDir::new_in(&dir_depth1).unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file2.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 40);
    let open_todos_count = todos.iter().filter(|t| !t.done).count();
    assert_eq!(open_todos_count, 32);

    dir.close().unwrap();
}

#[test]
fn test_parse_multiple_complex_files_and_folders_multiple_todos() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file2.md").unwrap();
    let _dir_depth1 = TempDir::new_in(&dir).unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth1, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1, "file2.md").unwrap();
    let _dir_depth1_2 = TempDir::new_in(&dir).unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file2.md").unwrap();
    let _dir_depth2 = TempDir::new_in(&_dir_depth1).unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file1.md").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth2, "file2.md").unwrap();
    let _dir_depth3 = TempDir::new_in(&_dir_depth2).unwrap();
    let _dir_depth4 = TempDir::new_in(&_dir_depth3).unwrap();
    let _dir_depth5 = TempDir::new_in(&_dir_depth4).unwrap();
    let _dir_depth6 = TempDir::new_in(&_dir_depth5).unwrap();
    let _dir_depth7 = TempDir::new_in(&_dir_depth6).unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth7, "file1.md").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth7, "file2.md").unwrap();

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    assert_eq!(todos.len(), 122);
    let open_todos_count = todos.iter().filter(|t| !t.done).count();
    let closed_todos_count = todos.iter().filter(|t| t.done).count();
    assert_eq!(open_todos_count, 84);
    assert_eq!(closed_todos_count, 38);

    dir.close().unwrap();
}
