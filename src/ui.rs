use crate::Todo;
use modo::md_writer;
use ncurses::*;

enum SCREEN {
    Main,
    Add,
    Edit,
}

pub fn draw_ui(todos: &Vec<Todo>, query: &str) {
    let bw: WINDOW = initscr();
    let mut cur_index: i32 = 0;
    let mut screen: i8 = SCREEN::Main as i8; // Set the screen
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // Don't show the terminal cursor
    keypad(bw, true);

    while cur_index != -1 {
        addstr("Query: ");
        addstr(query);
        addstr("\n ");
        for _i in 0..50 {
            addch(ACS_HLINE());
        }
        addstr("\n");
        let mut i = 0;
        if screen == SCREEN::Main as i8 {
            // list_todos(&todos, cur_index);
            for todo in todos.iter() {
                if cur_index == i {
                    addstr(&format!("> {} {}", &todo, "\n"));
                } else {
                    addstr(&format!("- {} {}", &todo, "\n"));
                }
                i += 1;
            }
            // Listens for key
            listen_key(&mut cur_index, todos.len(), &mut screen, &todos);
        }
        refresh();
        clear();
    }
    endwin();
}

fn listen_key(mut cur_index: &mut i32, max: usize, screen: &mut i8, todos: &Vec<Todo>) {
    noecho();
    let k: i32 = getch();
    echo();

    match k {
        106 | KEY_DOWN => {
            // J / UP
            if *cur_index != max as i32 - 1 {
                *cur_index += 1
            }
        }
        107 | KEY_UP => {
            // K / Down
            if *cur_index != 0 {
                *cur_index -= 1
            }
        }
        113 => *cur_index = -1, // Q
        120 => {
            // X
            md_writer::toggle_todo(&todos[*cur_index as usize]).unwrap();
        }
        101 => *cur_index = -1, // E
        _ => {}
    }
}
