/// Curses FFI wrapper

extern crate libc;
extern crate core;

use curses::libc::{c_char, c_int};
use cext::{TRUE, FALSE};
use std::ffi::{CString, c_str_to_bytes};
use std::mem;
use std::os::{getenv};
use sqlite::{Sqlite, MAX_TABLE_NAME_LENGTH, exec_results};
use self::core::str::FromStr;

pub static DEFAULT_WIDTH : usize = 80;
pub static DEFAULT_HEIGHT : usize = 40;
pub static CELL_WIDTH : usize = 10;

// Keys
const KEY_q : usize = 113;
const KEY_h : usize = 104;
const KEY_j : usize = 106;
const KEY_k : usize = 107;
const KEY_l : usize = 108;

// Attributes
static WA_BOLD : i32 = 2097152;

// Wrapper for opaque struct
#[repr(C)]
pub struct c_curses_window;

#[derive(Clone)]
pub struct Window {
    sqlite : Sqlite,
    window : *const c_curses_window,
    text : String
}

#[derive(Clone)]
pub struct Screen {
    sqlite : Sqlite,
    rows : usize,
    cols : usize,
    windows : Vec<Vec<Option<Window>>>,
    active_window : (usize, usize)
}

pub struct Curses {
    sqlite : Sqlite,
    width : usize,
    height : usize,
    screens : Vec<Screen>,
    active_screen : usize
}

#[link(name="ncurses")]
extern {
    // Initialization & teardown
    fn initscr() -> *const c_curses_window;
    fn start_color() -> c_int;
    fn curs_set(visibility : c_int) -> c_int;
    fn endwin() -> c_int;

    // Display
    fn printw(_ : *const c_char) -> c_int;
    fn wprintw(win : *const c_curses_window, fmt : CString) -> c_int;
    fn wattron(win : *const c_curses_window, attrs : c_int) -> c_int;
    fn wattroff(win : *const c_curses_window, attrs : c_int) -> c_int;
    fn wstandout(win : *const c_curses_window) -> c_int;
    fn wstandend(win : *const c_curses_window) -> c_int;
    fn wclear(win : *const c_curses_window) -> c_int;

    // Character input
    fn cbreak();
    fn noecho();
    fn nonl();
    fn keypad(win : *const c_curses_window, bf : c_int) -> c_int;
    fn wgetch(win : *const c_curses_window) -> c_int;

    // Window management
    fn newwin(nlines : c_int, 
              ncols : c_int, 
              begin_y : c_int,
              begin_x : c_int) -> *const c_curses_window;
    fn wrefresh(window : *const c_curses_window) -> c_int;
    fn delwin(window : *const c_curses_window) -> c_int;
}

impl Window {
    pub fn new(sqlite : Sqlite,
               nlines : usize, 
               ncols : usize, 
               begin_y : usize, 
               begin_x : usize) -> Window {
        let window : *const c_curses_window = unsafe { 
            newwin(nlines as c_int, 
                   ncols as c_int, 
                   begin_y as c_int, 
                   begin_x as c_int)
        };
        unsafe { keypad(window, TRUE); }
        Window {
            window : window,
            sqlite : sqlite,
            text : "".to_string()
        }
    }

    pub fn refresh(&self) {
        unsafe { wrefresh(self.window); };
    }

    pub fn write(&self) {
        self.clear();
        let s = CString::from_vec(self.text.as_bytes().to_vec());
        unsafe { wprintw(self.window, s); }
        self.refresh()
    }

    pub fn clear(&self) {
        unsafe { wclear(self.window); }
        self.refresh()
    }

    pub fn read_in(&self) -> usize {
        let c : c_int = unsafe { wgetch(self.window) };
        c as usize
    }

    pub fn select(&mut self) {
        self.text = ["*".to_string(), self.text.to_string(), "*".to_string()].concat();
        self.write()
    }

    pub fn unselect(&mut self) {
        self.text = self.text.as_slice().trim_matches('*').to_string();
        self.write()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { delwin(self.window); };
    }
}

impl Curses {
    pub fn new(filename : &str) -> Curses {
        unsafe { 
            initscr();
            curs_set(0 as c_int);
            cbreak();
            noecho();
            nonl();
        }

        let width = match Curses::get_env_as::<usize>("COLUMNS") {
            Some(n) => n,
            None => DEFAULT_WIDTH
        };
        let height = match Curses::get_env_as::<usize>("LINES") {
            Some(n) => n,
            None => DEFAULT_HEIGHT
        };

        let sqlite = Sqlite::new(filename);
        let mut screens : Vec<Screen> = Vec::new();
        let main_screen = Screen::new_table_listing(sqlite.clone(), width, height);
        screens.push(main_screen);

        Curses {
            width : width,
            height : height,
            screens : screens,
            active_screen : 0,
            sqlite : sqlite
        }
    }

    // XXX: Move me somewhere else
    fn get_env_as<T: FromStr>(s : &str) -> Option<T> {
        match getenv(s) {
            Some(n) => n.parse::<T>(),
            None => None
        }
    }

    pub fn run_forever(&mut self) {
        loop {
            let prev = self.get_active_window_coords();
            let (mut x, mut y) = prev;
            let c = self.read_current_window();

            match c {
                KEY_q => { break; },
                KEY_h => {
                    x = x - 1;
                    self.set_active_window(prev, (x, y));
                },
                KEY_j => {
                    y = y + 1;
                    self.set_active_window(prev, (x, y));
                },
                KEY_k => {
                    y = y - 1;
                    self.set_active_window(prev, (x, y));
                },
                KEY_l => {
                    x = x + 1;
                    self.set_active_window(prev, (x, y));
                }
                _ => { continue; }
            }
        }
    }

    fn get_active_window_coords(&self) -> (i64, i64) {
        let coords = self.screens[self.active_screen].active_window;
        (coords.0 as i64, coords.1 as i64)
    }

    fn get_active_window(&self) -> Option<&Window> {
        self.screens[self.active_screen].get_active_window()
    }

    fn set_active_window(&mut self, prev : (i64, i64), next : (i64, i64)) {
        self.screens[self.active_screen].set_active_window(prev, next)
    }

    fn get_active_screen_ref(&self) -> &Screen {
        &self.screens[self.active_screen]
    }

    fn get_active_screen_mut(&mut self) -> &mut Screen {
        &mut self.screens[self.active_screen]
    }

    fn read_current_window(&self) -> usize {
        let current_window : &Window = self.get_active_window().unwrap();
        current_window.read_in()
    }

}

impl Drop for Curses {
    fn drop(&mut self) {
        unsafe { endwin(); };
    }
}

impl Screen {
    fn init_windows(rows : usize, cols : usize) -> Vec<Vec<Option<Window>>> {
        let mut windows : Vec<Vec<Option<Window>>> = Vec::new();
        for i in (0..rows) {
            let mut row : Vec<Option<Window>> = Vec::new();
            for j in (0..cols) {
                row.push(None);
            }
            windows.push(row);
        }
        windows
    }

    fn new_table_listing(sqlite : Sqlite, width : usize, height : usize) -> Screen {
        sqlite.list_tables();

        let results = exec_results.lock().unwrap();
        let rows = height;
        let cols = width / CELL_WIDTH;
        let mut windows : Vec<Vec<Option<Window>>> = Screen::init_windows(rows, cols);
        let mut y : usize = 0;

        for result in (*results).iter() {
            for text in result.col_text.iter() {
                let mut window = Window::new(sqlite.clone(), 1, CELL_WIDTH, y, 0);
                window.text = text.clone();
                window.write();
                if y == 0 {
                    window.select();
                }
                windows[0][y] = Some(window);
                y = y + 1;
            }
        }

        Screen {
            rows : rows,
            cols : cols,
            windows : windows,
            active_window : (0, 0),
            sqlite : sqlite
        }
    }

    fn get_active_window(&self) -> Option<&Window> {
        self.windows[self.active_window.0][self.active_window.1].as_ref()
    }

    fn set_active_window(&mut self, prev : (i64, i64), next : (i64, i64)) {
        let (x, y) = match self.get_window(next.0, next.1) {
            Some(w) => {
                // println!("{} {}", prev.0, prev.1);
                // println!("{} {}", next.0, next.1);
                w.select();
                next
            },
            None => { prev }
        };
        if x != prev.0 || y != prev.1 {
            let prev_w = self.get_window(prev.0, prev.1).unwrap();
            prev_w.unselect();
        }
        self.active_window = (x as usize, y as usize);
    }

    fn get_window(&mut self, x : i64, y : i64) -> Option<&mut Window> {
        if x < 0 || x > self.rows as i64 || y < 0 || y > self.cols as i64 {
            return None
        }
        self.windows[x as usize][y as usize].as_mut()
    }
    // fn new_table_dump(sqlite : &Sqlite) -> Screen {
    //     let windows : Vec<Windows> = Vec::new();

    //     // I make all the windows that I need

    //     Screen::sqlite.exec(format!("select * from {}", tablename));
    //         let results = exec_results.lock().unwrap();
    //         for result in (*results).iter() {
    //             for text in result.col_text.iter() {
    //                 let w = Window::new()
    //             }
    //         }

    //     Screen {
    //         windows : windows,
    //         active_window : // reference to the top left cell
    //     }
    // }
}