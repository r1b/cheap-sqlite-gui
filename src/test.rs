use csgui::{CSG};
use csgui::{KEY_h, KEY_j, KEY_k, KEY_l, KEY_e, KEY_q};

#[test]
fn gui_setup() {
	let my_csgui = CSG::new("test.db");
	assert_eq!(my_csgui.active_screen, 0);
	assert_eq!(my_csgui.screens.len(), 1);
}

#[test]
fn main_screen_invalid_movement() {
	let mut my_csgui = CSG::new("test.db");
	// Coords should not change when moving outside boundary
	let coords = my_csgui.get_active_window_coords();
	let result = my_csgui.dispatch_key(KEY_h);
	let new_coords = my_csgui.get_active_window_coords();
	assert!(result.is_some());
	assert_eq!(coords, new_coords);
}

#[test]
fn main_screen_valid_movement() {
	let mut my_csgui = CSG::new("test.db");
	// Coords should change when moving inside boundary
	let coords = my_csgui.get_active_window_coords();
	let result = my_csgui.dispatch_key(KEY_j);
	let new_coords = my_csgui.get_active_window_coords();
	assert!(result.is_some());
	assert!(coords != new_coords);
}

#[test]
fn main_screen_edit() {
	// Edit should activate table dump screen
	let mut my_csgui = CSG::new("test.db");
	assert_eq!(my_csgui.active_screen, 0);
	let result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	assert_eq!(my_csgui.active_screen, 1);
}

#[test]
fn main_screen_quit() {
	// Quit should cease mainloop execution by returning None
	let mut my_csgui = CSG::new("test.db");
	let result = my_csgui.dispatch_key(KEY_q);
	assert!(result.is_none());
}

// #[test]
fn table_dump_screen_invalid_movement() {
	let mut my_csgui = CSG::new("test.db");
	// Coords should not change, column titles are not selectable
	let mut result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	let coords = my_csgui.get_active_window_coords();
	result = my_csgui.dispatch_key(KEY_k);
	let new_coords = my_csgui.get_active_window_coords();
	assert!(result.is_some());
	assert_eq!(coords, new_coords);
}

// #[test]
fn table_dump_screen_valid_movement() {
	let mut my_csgui = CSG::new("test.db");
	// Movement to a new column is allowed
	let mut result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	let coords = my_csgui.get_active_window_coords();
	result = my_csgui.dispatch_key(KEY_l);
	let new_coords = my_csgui.get_active_window_coords();
	assert!(result.is_some());
	assert!(coords != new_coords);
}

#[test]
fn table_dump_screen_edit() {
	// XXX: Edit unsupported
	let mut my_csgui = CSG::new("test.db");
	assert_eq!(my_csgui.active_screen, 0);
	let mut result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	assert_eq!(my_csgui.active_screen, 1);
	result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	assert_eq!(my_csgui.active_screen, 1);
}

#[test]
fn table_dump_screen_quit() {
	// Quit should return to main screen and not exit mainloop
	let mut my_csgui = CSG::new("test.db");
	assert_eq!(my_csgui.active_screen, 0);
	let mut result = my_csgui.dispatch_key(KEY_e);
	assert!(result.is_some());
	assert_eq!(my_csgui.active_screen, 1);
	result = my_csgui.dispatch_key(KEY_q);
	assert!(result.is_some());
	assert_eq!(my_csgui.active_screen, 0);
}