use crate::Todo;
use anyhow::Result;
use modo::md_writer;
use ncurses::*;
use std::path::Path;

enum UserAction {
    Navigation,
    Quit,
    Reload,
    Details,
}

pub fn draw_ui(query: &str, path: &Path) -> Result<()> {
    let win: WINDOW = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(win, true);

    // Outer loop with reload
    loop {
        let todos = modo::modo(path, query)?;
        let mut selected_todo_index: usize = 0;

        // Navigation
        loop {
            clear();
            addstr(&format!("Query: {}\n", query));
            for _i in 0..50 {
                addch(ACS_HLINE());
            }
            addstr("\n");

            for (i, todo) in todos.iter().enumerate() {
                if selected_todo_index == i {
                    addstr(&format!("> {} {}", &todo, "\n"));
                } else {
                    addstr(&format!("- {} {}", &todo, "\n"));
                }
            }

            let key_result = listen_key(&mut selected_todo_index, &todos);
            match key_result {
                UserAction::Navigation => continue,
                UserAction::Details => {
                    draw_todo_details(&todos[selected_todo_index as usize]);
                    break;
                }
                UserAction::Quit => {
                    endwin();
                    return Ok(());
                }
                UserAction::Reload => break,
            }
        }
    }
}

fn draw_todo_details(todo: &Todo) {
    clear();
    addstr(&todo.heading);

    match getch() {
        120 | 10 => {
            // x / enter
            md_writer::toggle_todo(todo).unwrap();
        }
        _ => {}
    }
}

fn listen_key(selected_todo_index: &mut usize, todos: &[Todo]) -> UserAction {
    match getch() {
        106 | KEY_DOWN => {
            // j / Arrow Down
            if *selected_todo_index != todos.len() - 1 {
                *selected_todo_index += 1;
            }
        }
        107 | KEY_UP => {
            // k / Arrow Up
            if *selected_todo_index != 0 {
                *selected_todo_index -= 1
            }
        }
        113 => return UserAction::Quit, // q
        120 | 10 => {
            // x / enter
            md_writer::toggle_todo(&todos[*selected_todo_index]).unwrap();
            return UserAction::Reload;
        }
        114 => return UserAction::Reload,  // r
        100 => return UserAction::Details, // d
        _ => {}
    }

    UserAction::Navigation
}
