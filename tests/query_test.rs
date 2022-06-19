mod common;
use common::md_test_file_creator;
use modo::{filter, query::Query};
use tempfile::TempDir;

#[test]
fn test_querys_1() {
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
    let query_string = String::from("done == true and filename << file");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 8, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path << file and filename << 2 and done != false");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 4, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("path !< file and filename << 2 and done != false");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 0, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("filename << 2 << head << a");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 0, "{}", query_string);

    dir.close().unwrap();
}

#[test]
fn test_querys_2() {
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
    let query_string = String::from("done == false and filename << file2.md");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 42, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 122, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name << todo and heading << work");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 24, "{}", query_string);

    let mut todos = md_todo::get_todos_from_path(&dir).unwrap();
    let query_string = String::from("name == todo and heading != work and filepath !< file1");
    let mut query = Query::new(&query_string);
    assert!(query.parse().is_ok());
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 6, "{}", query_string);

    dir.close().unwrap();
}
