mod common;
use common::md_test_file_creator;
use modo::{filter, query::Query};
use tempfile::TempDir;

#[test]
fn test_filter_done() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();

    // done true
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("done == true");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 2);

    // done false
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("done == false");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 3);

    dir.close().unwrap();
}

#[test]
fn test_filter_name() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();

    // contains
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name << open");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 2, "op: contains");

    // does not contain
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name !< open");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 3, "op: does not contain");

    // equals
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name == todo with hEaDiNg");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 1, "op: equals");

    // does not equal
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name != todo with hEaDiNg");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 4, "op: does not equal");

    dir.close().unwrap();
}

#[test]
fn test_filter_heading() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();

    // contains
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("heading << heading");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 5, "op: contains");

    // does not contain
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("heading !< 2");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 3, "op: does not contain");

    // equals
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("heading == ### Heading DONE");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 3, "op: equals");

    // does not equal
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("heading != abc");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 5, "op: does not equal");

    dir.close().unwrap();
}

#[test]
fn test_filter_filename() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file2.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file3.md").unwrap();

    // contains
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("filename << file");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 20, "op: contains");

    // does not contain
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("filename !< 2");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 15, "op: does not contain");

    // equals
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("filename == file0.md");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 5, "op: equals");

    // does not equal
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("filename != file1.md");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 15, "op: does not equal");

    dir.close().unwrap();
}

#[test]
fn test_filter_path() {
    let dir = TempDir::new().unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file2.md").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file3.md").unwrap();

    // contains
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path << /file");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 20, "op: contains");

    // does not contain
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path !< /file2");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 15, "op: does not contain");

    // equals
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path == file0.md");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 0, "op: equals");

    // does not equal
    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path != /file1.md");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 20, "op: does not equal");

    dir.close().unwrap();
}
