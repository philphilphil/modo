use crate::Todo;
use anyhow::Result;
use modo::{md_writer, todo};
use std::io::{stdin, stdout, Stdout, Write};
use std::path::Path;
use termion::color::Fg;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

enum UserAction {
    Navigation,
    Quit,
    Reload,
    Details,
}

// TODO: Change functionality so it always displays the todos from the initial query. if todos get
// done, move them to the closed section and the other way arround.
// add "r" to reload the query. indicate changed data and that the query isnt accurate anymore with a *: Query: * bla = bla
// add "e" to edit the query.

pub fn draw_ui(query: &str, path: &Path) -> Result<()> {
    // Outer loop with reload
    let mut selected_todo_index: usize = 0;

    loop {
        let todos = modo::modo(path, query)?;
        let (todos_open, todos_closed): (Vec<&Todo>, Vec<&Todo>) =
            todos.iter().partition(|t| !t.done);
        let mut stdout = stdout().into_raw_mode().unwrap();

        // Navigation
        loop {
            write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
            stdout.flush().unwrap();
            println!(
                "{}{}Query:{} {}",
                cursor::Goto(1, 1),
                style::Bold,
                style::Reset,
                query
            );
            println!(
                "{}{}─────────────────────────────────────",
                cursor::Goto(1, 2),
                selected_todo_index
            );
            println!(
                "{}{}{}Open:{} {}",
                cursor::Goto(1, 4),
                style::Bold,
                color::Fg(color::Red),
                style::Reset,
                todos_open.len(),
            );

            let mut line: u16 = 5;
            draw_todo_line(
                &todos_open,
                &mut line,
                &selected_todo_index,
                todos_open.len(),
            );

            line += 1;
            println!(
                "{}{}{}Closed:{} {}",
                cursor::Goto(1, line),
                style::Bold,
                color::Fg(color::Green),
                style::Reset,
                todos_closed.len(),
            );

            line += 1;
            draw_todo_line(
                &todos_closed,
                &mut line,
                &selected_todo_index,
                todos_open.len(),
            );

            stdout.flush().unwrap();
            let key_result = listen_nav_key(&mut selected_todo_index, &todos);
            match key_result {
                UserAction::Navigation => continue,
                UserAction::Details => {
                    draw_todo_details(&todos[selected_todo_index as usize], &mut stdout);
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

fn draw_todo_line(
    todos: &Vec<&Todo>,
    line: &mut u16,
    selected_todo_index: &usize,
    count_open: usize,
) {
    for todo in todos.iter() {
        write!(stdout(), "{}", termion::cursor::Goto(1, *line)).unwrap();

        if *selected_todo_index >= count_open && *line >= 7 {
            if *selected_todo_index == *line as usize - 7 {
                println!("> {}", &todo);
            } else {
                println!("- {}", &todo);
            }
        } else {
            if *selected_todo_index == *line as usize - 5 {
                println!("> {}", &todo);
            } else {
                println!("- {}", &todo);
            }
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
            Key::Char('x') => {
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

fn draw_todo_details(todo: &Todo, stdout: &mut Stdout) {
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    draw_todo_detail_line("Name", &todo.name, 1);

    if todo.done {
        draw_todo_detail_line_color("Status", "Done", color::Fg(color::Green), 2);
    } else {
        draw_todo_detail_line_color("Status", "Open", color::Fg(color::Red), 2);
    }

    draw_todo_detail_line("Heading", &todo.heading, 3);
    draw_todo_detail_line("Filename", &todo.filename, 4);
    draw_todo_detail_line("Lineno", &todo.line_no.to_string(), 5);

    if let Some(c) = stdin().keys().next() {
        if let Key::Char('x') = c.unwrap() {
            md_writer::toggle_todo(todo).unwrap();
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
