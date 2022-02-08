mod md_handler;
mod todo;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use std::path::Path;
use todo::Todo;

fn main() {
    loop {
        //println!("\x1B[2J\x1B[1;1H");
        let query: String = Input::new()
            .with_prompt(".. where ")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if !is_valid_query(&query) {
            //println!("{}", "Invalid query.");
            continue;
        }
        loop {
            let mut todos: Vec<Todo> = vec![];
            md_handler::load_data(Path::new("/Users/phil/TestingNotes"), &mut todos).unwrap();

            if query == "" {
                // load all open
                todos = todos.into_iter().filter(|t| !t.done).collect();
            } else {
                let mut query_parts: Vec<&str> = vec![];
                query_parts = query.split("and").collect();

                for q in query_parts {
                    // https://regex101.com/r/1g3YHS/1
                    let re = Regex::new("(done|path|filename|filepath|heading) (==|<>|<<) (.*)")
                        .unwrap();

                    if !re.is_match(q) {
                        panic!("query wrong");
                    }

                    let caps = re.captures(q).unwrap();

                    match &caps[1] {
                        "path" => {
                            todos = todos
                                .into_iter()
                                .filter(|t| t.filename.to_string().contains(&caps[2]))
                                .collect()
                        }
                        _ => println!("{}", "Error"),
                    }

                    println!("{:?}", caps);
                    //match re.
                }
            }

            if todos.len() < 1 {
                println!("{}", "No todos found.");
                break;
            }

            let todo_strings: Vec<String> = todos.iter().map(|t| t.to_string()).collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose todo to toggle:")
                .clear(true)
                .default(0)
                .items(&todo_strings)
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                let selected_todo = &todos[selection];
                println!(
                    "Marked {} as {}!",
                    selected_todo.name,
                    if selected_todo.done { "open" } else { "done" }
                );
            } else {
                break;
            }

            md_handler::mark_as_done(&todos[selection.unwrap()]);
        }
    }
}

fn is_valid_query(query: &str) -> bool {
    true
}
