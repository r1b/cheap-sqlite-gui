/// Curses FFI wrapper

extern crate libc;

use curses::libc::{c_char, c_int};
use cext::{TRUE, FALSE, str_to_cstr};

const CURSOR_INVISIBLE : c_int = 0;

// Wrapper for opaque struct
#[repr(C)]
pub struct c_curses_window;

/// Wrapper for curses Window
#[derive(Clone)]
pub struct Window {
    window : *const c_curses_window,
}

/// Opaque curses struct, needed for setup & teardown
pub struct Curses;

#[link(name="ncurses")]
extern {
    // Initialization & teardown
    fn initscr() -> *const c_curses_window;
    fn start_color() -> c_int;
    fn curs_set(visibility : c_int) -> c_int;
    fn endwin() -> c_int;

    // Display
    fn printw(_ : *const c_char) -> c_int;
    fn wprintw(win : *const c_curses_window, fmt : *const c_char) -> c_int;
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
    pub fn new(nlines : usize, 
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
            window : window
        }
    }

    /// Writes window content to screen
    pub fn refresh(&self) {
        unsafe { wrefresh(self.window); };
    }

    /// Writes text to a window and refresh
    pub fn write(&self, s : &str) {
        self.clear();
        let s = str_to_cstr(s);
        unsafe { wprintw(self.window, s.as_ptr()); }
        self.refresh()
    }

    /// Clears the window
    pub fn clear(&self) {
        unsafe { wclear(self.window); }
        self.refresh()
    }

    /// Reads in keystrokes
    pub fn read_in(&self) -> usize {
        let c : c_int = unsafe { wgetch(self.window) };
        c as usize
    }
}

impl Drop for Window {
    /// Destroys the curses window
    fn drop(&mut self) {
        unsafe { delwin(self.window); };
    }
}

impl Curses {
    /// Initializes curses
    pub fn new() -> Curses {
        unsafe { 
            initscr();
            curs_set(CURSOR_INVISIBLE);
            cbreak();
            noecho();
            nonl();
        }
        Curses
    }

}

impl Drop for Curses {
    /// Destroys the curses session
    fn drop(&mut self) {
        unsafe { endwin(); };
    }
}