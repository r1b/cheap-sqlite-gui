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

// Keys
static KEY_q : usize = 113;

// Wrapper for opaque struct
pub struct c_curses_window;

#[derive(Clone)]
pub struct Window {
    window : *const c_curses_window,
    sqlite : Sqlite
}

#[derive(Clone)]
pub struct Screen {
    sqlite : Sqlite,
    windows : Vec<Window>,
    active_window : Option<Window>
}

pub struct Curses {
    width : usize,
    height : usize,
    screens : Vec<Screen>,
    active_screen : Option<Screen>,
    sqlite : Sqlite
}

#[link(name="ncurses")]
extern {
    // Initialization & teardown
    fn initscr() -> *const c_curses_window;
    fn endwin() -> c_int;

    // Display
    fn printw(_ : *const c_char) -> c_int;
    fn wprintw(win : *const c_curses_window, fmt : CString) -> c_int;
    fn wstandout(win : *const c_curses_window) -> c_int;
    fn wstandend(win : *const c_curses_window) -> c_int;

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

    // Attributes
    static WA_STANDOUT : c_int;
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
            sqlite : sqlite
        }
    }

    pub fn refresh(&self) {
        unsafe { wrefresh(self.window); };
    }

    pub fn write(&self, s : &str) {
        let s = CString::from_vec(s.as_bytes().to_vec());
        unsafe { wprintw(self.window, s); }
        self.refresh();
    }

    pub fn read(&self) -> usize {
        let c : c_int = unsafe { wgetch(self.window) };
        c as usize
    }

    pub fn select(&self) {
        unsafe { wstandout(self.window); };
        self.refresh()
    }

    pub fn unselect(&self) {
        unsafe { wstandend(self.window); };
        self.refresh()
    }
}

#[unsafe_destructor]
impl Drop for Window {
    fn drop(&mut self) {
        unsafe { delwin(self.window); };
    }
}

impl Curses {
    pub fn new(filename : &str) -> Curses {
        unsafe { 
            initscr();
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

        let mut curses = Curses {
            width : width,
            height : height,
            screens : Vec::new(),
            active_screen : None,
            sqlite : Sqlite::new(filename)
        };

        let main_screen = Screen::new_table_listing(curses.sqlite.clone());
        curses.screens.push(main_screen);
        curses.active_screen = Some(curses.screens[0].clone());

        curses
    }

    // XXX: Move me somewhere else
    fn get_env_as<T: FromStr>(s : &str) -> Option<T> {
        match getenv(s) {
            Some(n) => n.parse::<T>(),
            None => None
        }
    }

    pub fn run_forever(&self) {
        loop {
            let ref active_window : Window = self.screens[0].windows[0];
            active_window.select();
            let c = active_window.read();

            if c == KEY_q {
                break;
            }
        }
    }

    // fn get_active_window(&self) -> Window {
    //     self.active_screen.clone().unwrap().active_window.clone().unwrap()
    // }

}

#[unsafe_destructor]
impl Drop for Curses {
    fn drop(&mut self) {
        unsafe { endwin(); };
    }
}

impl Screen {
    fn new_table_listing(sqlite : Sqlite) -> Screen {
        let mut screen = Screen {
            windows : Vec::new(),
            active_window : None,
            sqlite : sqlite
        };

        screen.sqlite.list_tables();

        let results = exec_results.lock().unwrap();
        let mut y : usize = 0;

        for result in (*results).iter() {
            for text in result.col_text.iter() {
                let window = Window::new(screen.sqlite.clone(),
                                         1,
                                         MAX_TABLE_NAME_LENGTH,
                                         y,
                                         0
                );
                window.write(text.as_slice());
                screen.windows.push(window);
                y = y + 1;
            }
        }

        screen.active_window = Some(screen.windows[0].clone());
        screen
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