use std::path::Path;

use crate::Todo;
use modo::md_writer;
use ncurses::*;

enum UserAction {
    Navigation,
    Quit,
    Refresh,
}

pub fn draw_ui(query: &str, path: &Path) {
    let query = query.to_string();
    let bw: WINDOW = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);

    loop {
        refresh();
        clear();
        endwin();
        let mut cur_index: i32 = 0;
        let todos: Vec<Todo>;
        if let Ok(t) = modo::modo(path, &query) {
            todos = t
        } else {
            todo!()
        }

        loop {
            refresh();
            clear();
            endwin();
            addstr("Query: ");
            addstr(&query);
            addstr("\n ");
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
                UserAction::Quit => return,
                UserAction::Refresh => break,
            }
        }
    }
}

fn listen_key(cur_index: &mut i32, max: usize, todos: &[Todo]) -> UserAction {
    noecho();
    let k: i32 = getch();
    echo();

    match k {
        106 | KEY_DOWN => {
            // J / Arrow Down
            if *cur_index != max as i32 - 1 {
                *cur_index += 1;
            }
        }
        107 | KEY_UP => {
            // K / Arrow Up
            if *cur_index != 0 {
                *cur_index -= 1
            }
        }
        113 => return UserAction::Quit, // Q
        120 => {
            // X
            md_writer::toggle_todo(&todos[*cur_index as usize]).unwrap();
            return UserAction::Refresh;
        }
        114 => return UserAction::Refresh,
        _ => {}
    }

    UserAction::Navigation
}
