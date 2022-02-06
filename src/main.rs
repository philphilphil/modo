mod md_handler;
mod todo;
use std::path::Path;
use todo::Todo;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let query: String = Input::new().with_prompt(".. where ").interact().unwrap();

    if query == "all" {
        loop {
            let mut todos: Vec<Todo> = vec![];
            md_handler::load_data(Path::new("/Users/phil/TestingNotes"), &mut todos).unwrap();
            let todo_strings: Vec<String> = todos.iter().map(|t| t.to_string()).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose todo to toggle:")
                .clear(true)
                .default(0)
                .items(&todo_strings)
                .interact_opt()
                .unwrap();

            let selected_todo = &todos[selection.unwrap()];
            println!("Marked {} as {}!", selected_todo.name, if selected_todo.done { "open"} else { "done"});

            md_handler::mark_as_done(&todos[selection.unwrap()]);
        }
    }
}
