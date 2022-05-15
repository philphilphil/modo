use crate::Todo;
use anyhow::Result;
use modo::md_writer;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
enum UserAction {
    Navigation,
    Quit,
    Reload,
    Details,
}

pub fn draw_ui(query: &str, path: &Path) -> Result<()> {
    // Outer loop with reload
    loop {
        let todos = modo::modo(path, query)?;
        let mut selected_todo_index: usize = 0;
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Navigation
        loop {
            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
            stdout.flush().unwrap();
            println!(
                "{}Query: {}{}",
                termion::cursor::Goto(1, 1),
                query,
                selected_todo_index
            );

            for (i, todo) in todos.iter().enumerate() {
                write!(stdout, "{}", termion::cursor::Goto(1, i as u16 + 3)).unwrap();
                if selected_todo_index == i {
                    println!("> {}", &todo);
                } else {
                    println!("- {}", &todo);
                }
            }

            stdout.flush().unwrap();
            let key_result = listen_nav_key(&mut selected_todo_index, &todos);
            match key_result {
                UserAction::Navigation => continue,
                UserAction::Details => {
                    draw_todo_details(&todos[selected_todo_index as usize]);
                    break;
                }
                UserAction::Quit => {
                    write!(stdout, "{}", termion::cursor::Show).unwrap();
                    return Ok(());
                }
                UserAction::Reload => break,
            }
        }
    }
}

fn draw_todo_details(todo: &Todo) {
    //Todo with additonal screen thing?
    // clear();
    // addstr(&format!("Name: {}\n", todo.name));
    // addstr(&format!("Status: {}\n", todo.done));
    // addstr(&format!("Heading: {}\n", todo.heading));
    // addstr(&format!("Filename: {}\n", todo.filename));
    // addstr(&format!("Line Number: {}\n", todo.line_no));
    // match getch() {
    //     120 | 10 => {
    //         // x / enter
    //         md_writer::toggle_todo(todo).unwrap();
    //     }
    //     _ => {}
    // }
}

fn listen_nav_key(selected_todo_index: &mut usize, todos: &[Todo]) -> UserAction {
    if let Some(c) = stdin().keys().next() {
        match c.unwrap() {
            Key::Char('j') | Key::Down => {
                // j / arrow down
                if *selected_todo_index != todos.len() - 1 {
                    *selected_todo_index += 1;
                }
            }
            Key::Char('k') | Key::Up => {
                // k / arrow up
                if *selected_todo_index != 0 {
                    *selected_todo_index -= 1
                }
            }
            Key::Char('q') => return UserAction::Quit, // q
            Key::Char('x') => {
                // x
                md_writer::toggle_todo(&todos[*selected_todo_index]).unwrap();
                return UserAction::Reload;
            }
            Key::Char('r') => return UserAction::Reload, // r
            Key::Char('d') => return UserAction::Details, // d
            _ => {}
        }
        return UserAction::Navigation;
    }

    UserAction::Navigation
}
