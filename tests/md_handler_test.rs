mod common;
use std::path::Path;
use modo::todo::Todo;
use modo::md_handler;

#[test]
#[should_panic]
fn read_not_existing_path_should_fail() {
    common::setup();
    let mut todos: Vec<Todo> = vec![];
    md_handler::load_data(Path::new("/This/PathDoes/n/o/t/Exist"), &mut todos).unwrap();
}
