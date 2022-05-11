use std::path::Path;

use crate::Todo;
use modo::md_writer;
use ncurses::*;

pub enum Screen {
    Navigation,
    EditQuery,
    Reload,
    Quit,
}

pub fn draw_ui(query: &str, path: &Path) {
    let mut query = query.to_string();
    let bw: WINDOW = initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);
    loop {
        refresh();
        clear();
        endwin();
        let mut todos: Vec<Todo> = vec![];
        match modo::modo(path, &query) {
            Ok(t) => todos = t,
            Err(_) => todo!(),
        }

        match draw_navigation(&todos, &query) {
            Screen::Quit => return,
            Screen::EditQuery => draw_query_edit(&mut query),
            _ => {}
        }
    }
}

pub fn draw_navigation(todos: &[Todo], query: &str) -> Screen {
    let mut cur_index: i32 = 0;
    while cur_index != -1 {
        refresh();
        clear();
        endwin();
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
        let key_result = listen_navigation_key(&mut cur_index, todos.len(), todos);

        match key_result {
            Screen::Navigation => continue,
            Screen::Reload => return Screen::Reload,
            Screen::Quit => return Screen::Quit,
            Screen::EditQuery => return Screen::EditQuery,
        }
    }

    return Screen::Quit;
}

pub fn draw_query_edit(query: &mut String) {
    *query = String::from("done == false");
}

fn listen_navigation_key(cur_index: &mut i32, max: usize, todos: &[Todo]) -> Screen {
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
        113 => return Screen::Quit, // Q
        120 => {
            // X
            md_writer::toggle_todo(&todos[*cur_index as usize]).unwrap();
            return Screen::Reload;
        }
        101 => return Screen::EditQuery, // E
        _ => {}
    }

    Screen::Reload
}
