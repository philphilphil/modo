use std::path::{Path, PathBuf};

use crate::Todo;
use modo::md_writer;
use ncurses::*;

enum Screen {
    Main,
    Add,
    Navigation,
    Reload,
}

pub fn draw_ui(query: &str, path: &Path) {
    let mut todos: Vec<Todo> = Vec::new();
    match modo::modo(&path, &query) {
        Ok(t) => todos = t,
        Err(_) => todo!(),
    }

    let bw: WINDOW = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);
    draw_table(&todos, query);
}

pub fn draw_table(todos: &Vec<Todo>, query: &str) {
    let mut cur_index: i32 = 0;
    while cur_index != -1 {
        addstr("Query: ");
        addstr(query);
        addstr("\n ");
        for _i in 0..50 {
            addch(ACS_HLINE());
        }
        addstr("\n");
        // list_todos(&todos, cur_index);
        for (i, todo) in todos.iter().enumerate() {
            if cur_index == i as i32 {
                addstr(&format!("> {} {}", &todo, "\n"));
            } else {
                addstr(&format!("- {} {}", &todo, "\n"));
            }
        }
        // Listens for key
        match listen_key(&mut cur_index, todos.len(), todos) {
            Screen::Navigation => {}
            Screen::Reload => {
                todo!();
            }
            _ => {}
        }
        refresh();
        clear();
        endwin();
    }
}

pub fn draw_query_edit() {}

fn listen_key(mut cur_index: &mut i32, max: usize, todos: &Vec<Todo>) -> Screen {
    noecho();
    let k: i32 = getch();
    echo();

    match k {
        106 | KEY_DOWN => {
            // J / UP
            if *cur_index != max as i32 - 1 {
                *cur_index += 1;
                return Screen::Navigation;
            }
        }
        107 | KEY_UP => {
            // K / Down
            if *cur_index != 0 {
                *cur_index -= 1
            }
            return Screen::Navigation;
        }
        113 => *cur_index = -1, // Q
        120 => {
            // X
            md_writer::toggle_todo(&todos[*cur_index as usize]).unwrap();
            return Screen::Reload;
        }
        101 => *cur_index = -1, // E
        _ => {}
    }

    Screen::Reload
}
