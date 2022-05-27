use crate::Todo;
use anyhow::Result;
use std::io::{stdin, stdout};
use std::path::Path;
use termion::color::Fg;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

enum UserAction {
    Navigation,
    Toggle,
    Quit,
    Reload,
    Details,
}

pub fn draw_ui(query: &str, path: &Path) -> Result<()> {
    let mut selected_todo_index: usize = 0;
    // Outer loop with reload
    loop {
        let mut todos = modo::modo(path, query)?;
        let mut _stdout = stdout().into_raw_mode().unwrap();

        if todos.is_empty() {
            print!("{}No todos found.", cursor::Goto(1, 3));
            print!("{}", termion::cursor::Show);
            return Ok(());
        }

        if selected_todo_index > todos.len() - 1 {
            selected_todo_index = todos.len() - 1
        }

        // Navigation loop. draws the todo and a > for the selected todo
        loop {
            todos.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            todos.sort_by(|a, b| a.done.cmp(&b.done));
            let todos_open: Vec<String> = todos
                .iter()
                .filter(|t| !t.done)
                .map(|t| t.name.clone())
                .collect();
            let todos_closed: Vec<String> = todos
                .iter()
                .filter(|t| t.done)
                .map(|t| t.name.clone())
                .collect();
            print!("{}{}", clear::All, cursor::Hide);

            draw_header(query);

            draw_state_header("Open", color::Fg(color::Red), todos_open.len(), 4);

            let mut line: u16 = 5;
            draw_todo_lines(&todos_open, &mut line, &selected_todo_index, 5, false);

            line += 1;
            draw_state_header("Closed", color::Fg(color::Green), todos_closed.len(), line);

            line += 1;
            draw_todo_lines(&todos_closed, &mut line, &selected_todo_index, 7, true);

            let key_result = listen_nav_key(&mut selected_todo_index, &todos);
            match key_result {
                UserAction::Navigation => continue,
                UserAction::Details => {
                    draw_todo_details(&mut todos[selected_todo_index]);
                    break;
                }
                UserAction::Toggle => {
                    todos[selected_todo_index].toggle();
                    break;
                }
                UserAction::Quit => {
                    print!("{}", termion::cursor::Show);
                    return Ok(());
                }
                UserAction::Reload => break,
            }
        }
    }
}

fn draw_header(query: &str) {
    print!(
        "{}{}Query:{} {}",
        cursor::Goto(1, 1),
        style::Bold,
        style::Reset,
        query
    );
    print!(
        "{}─────────────────────────────────────",
        cursor::Goto(1, 2)
    );
}

fn draw_state_header<K: termion::color::Color>(state: &str, color: Fg<K>, count: usize, line: u16) {
    print!(
        "{}{}{}{}{}: {}",
        cursor::Goto(1, line),
        style::Bold,
        color,
        state,
        style::Reset,
        count,
    );
}

fn draw_todo_lines(
    todos: &[String],
    line: &mut u16,
    selected_todo_index: &usize,
    line_modifier: usize,
    done: bool,
) {
    for todo in todos.iter() {
        print!("{}", termion::cursor::Goto(1, *line));
        let braces = if done { "[x]" } else { "[ ]" };

        if *selected_todo_index == *line as usize - line_modifier {
            println!("> {} {}", braces, &todo);
        } else {
            println!("- {} {}", braces, &todo);
        }

        *line += 1;
    }
}

fn listen_nav_key(selected_todo_index: &mut usize, todos: &[Todo]) -> UserAction {
    if let Some(c) = stdin().keys().next() {
        match c.unwrap() {
            Key::Char('j') | Key::Down => {
                if *selected_todo_index != todos.len() - 1 {
                    *selected_todo_index += 1;
                }
            }
            Key::Char('k') | Key::Up => {
                if *selected_todo_index != 0 {
                    *selected_todo_index -= 1
                }
            }
            Key::Char('q') => return UserAction::Quit, // q
            Key::Char('x') => return UserAction::Toggle,
            Key::Char('r') => return UserAction::Reload, // r
            Key::Char('d') => return UserAction::Details, // d
            _ => {}
        }
        return UserAction::Navigation;
    }

    UserAction::Navigation
}

fn draw_todo_details(todo: &mut Todo) {
    println!("{}", termion::clear::All);

    draw_todo_detail_line("Name", &todo.name, 1);
    print!(
        "{}─────────────────────────────────────",
        cursor::Goto(1, 2)
    );

    if todo.done {
        draw_todo_detail_line_color("Status", "Done", color::Fg(color::Green), 3);
    } else {
        draw_todo_detail_line_color("Status", "Open", color::Fg(color::Red), 3);
    }

    draw_todo_detail_line("Heading", &todo.heading, 4);
    // draw_todo_detail_line("Filename", &todo.filename, 4);
    draw_todo_detail_line("Path", todo.filepath.to_str().unwrap(), 6);
    draw_todo_detail_line("Lineno", &todo.line_no.to_string(), 7);

    if let Some(c) = stdin().keys().next() {
        if let Key::Char('x') = c.unwrap() {
            todo.toggle();
        }
    }
}

fn draw_todo_detail_line(attribute: &str, value: &str, line_no: u16) {
    println!(
        "{}{}{}: {}{}",
        cursor::Goto(1, line_no),
        style::Bold,
        attribute,
        style::Reset,
        value
    );
}

fn draw_todo_detail_line_color<K: termion::color::Color>(
    attribute: &str,
    value: &str,
    color: Fg<K>,
    line_no: u16,
) {
    println!(
        "{}{}{}:{} {}{}{}",
        cursor::Goto(1, line_no),
        style::Bold,
        attribute,
        style::Reset,
        color,
        value,
        style::Reset,
    );
}
