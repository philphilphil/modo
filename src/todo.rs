pub struct Todo {
    pub name: String,
    pub filename: String,
    pub line_no: i32,
    pub done: bool,
    pub filepath: String
}

impl Todo {
    // Construct person
    pub fn new(name: &str, filename: &str, line_no: i32, done: bool, filepath: &str) -> Todo {
        Todo {
            name: name.to_string(),
            filename: filename.to_string(),
            line_no: line_no,
            done: done,
            filepath: filepath.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", if self.done { "✅" } else { "➔" }, self.name)
    }
}
