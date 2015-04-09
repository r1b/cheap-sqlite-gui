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

pub struct Window<'a> {
    window : *const c_curses_window,
    sqlite : &'a Sqlite
}

pub struct Screen<'a> {
    sqlite : &'a Sqlite,
    windows : Vec<Window<'a>>,
    active_window : Option<&'a mut Window<'a>>
}

pub struct Curses<'a> {
    width : usize,
    height : usize,
    screens : Vec<Screen<'a>>,
    active_screen : Option<&'a mut Screen<'a>>,
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
    fn wattrset(win : *const c_curses_window, attrs : c_int) -> c_int;
    fn wattroff(win : *const c_curses_window, attrs : c_int) -> c_int;

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
    static A_STANDOUT : c_int;
}

impl<'a> Window<'a> {
    pub fn new(sqlite : &Sqlite,
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
        unsafe { wattrset(self.window, A_STANDOUT); };
    }

    pub fn unselect(&self) {
        unsafe { wattroff(self.window, A_STANDOUT); };
    }
}

#[unsafe_destructor]
impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe { delwin(self.window); };
    }
}

impl<'a> Curses<'a> {
    pub fn new(filename : &'static str) -> Curses {
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

        curses
    }

    pub fn init(&'a mut self) {
        let mut main_screen = Screen::new_table_listing(&self.sqlite);
        self.screens.push(main_screen);
        self.active_screen = Some(&mut self.screens[0]);
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
            let active_window : &Window = self.get_active_window();
            let c = active_window.read();

            if c == KEY_q {
                break;
            }
        }
    }

    fn get_active_window(&self) -> &Window {
        self.active_screen.unwrap().active_window.unwrap()
    }

}

#[unsafe_destructor]
impl<'a> Drop for Curses<'a> {
    fn drop(&mut self) {
        unsafe { endwin(); };
    }
}

impl<'a> Screen<'a> {
    fn new_table_listing(sqlite : &Sqlite) -> Screen {
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
                let window = Window::new(screen.sqlite,
                                         1,
                                         MAX_TABLE_NAME_LENGTH,
                                         y,
                                         0
                );
                window.write(text.as_slice());
                window.select();
                screen.windows.push(window);
                y = y + 1;
            }
        }
        screen.init();
        screen
    }

    fn init(&'a mut self) {
        self.active_window = Some(&mut self.windows[0]);
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