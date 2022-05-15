use crate::Todo;
use anyhow::Result;
use modo::md_writer;
use ncurses::*;
use std::path::Path;

enum UserAction {
    Navigation,
    Quit,
    Reload,
}

pub fn draw_ui(query: &str, path: &Path) -> Result<()> {
    let win: WINDOW = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(win, true);

    // Outer loop with reload
    loop {
        let todos = modo::modo(path, query)?;
        let mut cur_index: i32 = 0;

        // Navigation
        loop {
            clear();
            addstr(&format!("Query: {}\n", query));
            for _i in 0..50 {
                addch(ACS_HLINE());
            }
            addstr("\n");

            for (i, todo) in todos.iter().enumerate() {
                if cur_index == i as i32 {
                    addstr(&format!("> {} {}", &todo, "\n"));
                } else {
                    addstr(&format!("- {} {}", &todo, "\n"));
                }
            }

            let key_result = listen_key(&mut cur_index, todos.len(), &todos);
            match key_result {
                UserAction::Navigation => continue,
                UserAction::Quit => {
                    endwin();
                    return Ok(());
                }
                UserAction::Reload => break,
            }
        }
    }
}

fn listen_key(cur_index: &mut i32, max: usize, todos: &[Todo]) -> UserAction {
    match getch() {
        106 | KEY_DOWN => {
            // j / Arrow Down
            if *cur_index != max as i32 - 1 {
                *cur_index += 1;
            }
        }
        107 | KEY_UP => {
            // k / Arrow Up
            if *cur_index != 0 {
                *cur_index -= 1
            }
        }
        113 => return UserAction::Quit, // q
        120 | 10 => {
            // X
            md_writer::toggle_todo(&todos[*cur_index as usize]).unwrap();
            return UserAction::Reload;
        }
        114 => return UserAction::Reload, // r
        _ => {}
    }

    UserAction::Navigation
}
