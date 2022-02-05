pub struct Todo {
    pub name: String,
    pub filename: String,
}

impl Todo {
    // Construct person
    pub fn new(name: &str, filename: &str) -> Todo {
        Todo {
            name: name.to_string(),
            filename: filename.to_string(),
        }
    }
}
