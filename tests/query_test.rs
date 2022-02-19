mod common;
use common::md_test_file_creator;
use modo::{filter, md_handler, query::Query, todo::Todo};
use tempdir::TempDir;

#[test]
fn test_querys_1() {
    let mut todos: Vec<Todo> = vec![];
    let dir = TempDir::new("modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file2.md").unwrap();
    let dir_depth1 = TempDir::new_in(&dir, "modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir_depth1, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir_depth1, "file2.md").unwrap();
    let _dir_depth1_2 = TempDir::new_in(&dir, "modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file2.md").unwrap();
    let _dir_depth2 = TempDir::new_in(&dir_depth1, "modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file2.md").unwrap();
    md_handler::load_data(dir.path(), &mut todos).unwrap();

    let mut todos2 = todos.clone();
    let query_string = String::from("done == true and filename << file");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos2);
    assert_eq!(todos2.len(), 8, "{}", query_string);

    let mut todos3 = todos.clone();
    let query_string = String::from("path << file and filename << 2 and done != false");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos3);
    assert_eq!(todos3.len(), 4, "{}", query_string);

    let mut todos4 = todos.clone();
    let query_string = String::from("path !< file and filename << 2 and done != false");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos4);
    assert_eq!(todos4.len(), 0, "{}", query_string);

    let mut todos5 = todos.clone();
    let query_string = String::from("filename << 2 << head << a");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos5);
    assert_eq!(todos5.len(), 0, "{}", query_string);

    dir.close().unwrap();
}

#[test]
fn test_querys_2() {
    let mut todos: Vec<Todo> = vec![];
    let dir = TempDir::new("modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&dir, "file2.md").unwrap();
    let _dir_depth1 = TempDir::new_in(&dir, "modo_integrationtests").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth1, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1, "file2.md").unwrap();
    let _dir_depth1_2 = TempDir::new_in(&dir, "modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file1.md").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth1_2, "file2.md").unwrap();
    let _dir_depth2 = TempDir::new_in(&_dir_depth1, "modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_4_open(&_dir_depth2, "file1.md").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth2, "file2.md").unwrap();
    let _dir_depth3 = TempDir::new_in(&_dir_depth2, "modo_integrationtests").unwrap();
    let _dir_depth4 = TempDir::new_in(&_dir_depth3, "modo_integrationtests").unwrap();
    let _dir_depth5 = TempDir::new_in(&_dir_depth4, "modo_integrationtests").unwrap();
    let _dir_depth6 = TempDir::new_in(&_dir_depth5, "modo_integrationtests").unwrap();
    let _dir_depth7 = TempDir::new_in(&_dir_depth6, "modo_integrationtests").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth7, "file1.md").unwrap();
    md_test_file_creator::complex_23_todos_15_open(&_dir_depth7, "file2.md").unwrap();
    md_handler::load_data(dir.path(), &mut todos).unwrap();

    let mut todos2 = todos.clone();
    let query_string = String::from("done == false and filename << file2.md");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos2);
    assert_eq!(todos2.len(), 42, "{}", query_string);

    let mut todos2 = todos.clone();
    let query_string = String::from("");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos2);
    assert_eq!(todos2.len(), 84, "{}", query_string);

    let mut todos3 = todos.clone();
    let query_string = String::from("name << todo and heading << work");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos3);
    assert_eq!(todos3.len(), 24, "{}", query_string);

    let mut todos4 = todos.clone();
    let query_string = String::from("name == todo and heading != work and filepath !< file1");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos4);
    assert_eq!(todos4.len(), 6, "{}", query_string);

    dir.close().unwrap();
}
