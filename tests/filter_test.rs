mod common;
use common::md_test_file_creator;
use modo::{filter, md_handler, query::Query, todo::Todo};
use tempdir::TempDir;

#[test]
fn test_filter_done() {
    let mut todos: Vec<Todo> = vec![];
    let dir = TempDir::new("modo_integrationtests").unwrap();
    md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();

    // done true
    md_handler::load_data(dir.path(), &mut todos).unwrap();
    let query_string = String::from("done == true");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 2);

    // done false
    todos = vec![];
    md_handler::load_data(dir.path(), &mut todos).unwrap();
    let query_string = String::from("done == false");
    let mut query = Query::new(&query_string);
    assert_eq!(query.parse(), true);
    filter::filter(&query, &mut todos);
    assert_eq!(todos.len(), 3);

    dir.close().unwrap();
}

// #[test]
// fn test_filter_name() {
//     let mut todos: Vec<Todo> = vec![];
//     let dir = TempDir::new("modo_integrationtests").unwrap();
//     md_test_file_creator::simple_5_todos_3_open_with_headings(&dir, "file0.md").unwrap();

//     // contains
//     md_handler::load_data(dir.path(), &mut todos).unwrap();
//     let query_string = String::from("name << open");
//     let mut query = Query::new(&query_string);
//     assert_eq!(query.parse(), true);
//     filter::filter(&query, &mut todos);
//     assert_eq!(todos.len(), 2);

//     // equals
//     todos = vec![];
//     md_handler::load_data(dir.path(), &mut todos).unwrap();
//     println!("DEBUG: {:#?}", todos);
//     let query_string = String::from("name << todo with heading");
//     let mut query = Query::new(&query_string);
//     assert_eq!(query.parse(), true);
//     println!("DEBUG: {:#?}", query.predicates);

//     filter::filter(&query, &mut todos);
//     assert_eq!(todos.len(), 1);

//     dir.close().unwrap();
// }