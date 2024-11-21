use crate::core::*;
use crate::controllers::*;
use raylib::prelude::*;

// A view responsible for rendering the state
// Tightly coupled with its respective controller
pub struct HomeView {
    spinner_value : i32,
    spinner_edit_mode : bool,
}

impl HomeView {
    pub fn new() -> Self {
        Self {
            spinner_value : 0,
            spinner_edit_mode : false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &HomeController, current_font_size : i32) {
        d.clear_background(Color::RAYWHITE);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text(&line, 10, 10, current_font_size, Color::BLACK);


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 50), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
            None,
            &mut self.spinner_value,
            0,
            100,
            self.spinner_edit_mode,
        );
        if updated_spinner {
            self.spinner_edit_mode = !self.spinner_edit_mode;
        }

        // gui_value_box() (and gui_spinner() too since it's used by it. The "value" argument
        // must be a value living for the whole draw loop, so we just dup them
        // to the View and ensure to set them on all frames to the model via
        // the controller.
        controller.set_value(self.spinner_value);
    }
}

pub struct SecondView {
    spinner_value : i32,
    spinner_edit_mode : bool,
}

impl SecondView {
    pub fn new() -> Self {
        Self {
            spinner_value : 0,
            spinner_edit_mode : false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &SecondController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text(&line, 10, 10, current_font_size, Color::BLACK);


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 50), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
            None,
            &mut self.spinner_value,
            0,
            100,
            self.spinner_edit_mode,
        );
        if updated_spinner {
            self.spinner_edit_mode = !self.spinner_edit_mode;
        }

        // gui_value_box() (and gui_spinner() too since it's used by it. The "value" argument
        // must be a value living for the whole draw loop, so we just dup them
        // to the View and ensure to set them on all frames to the model via
        // the controller.
        controller.set_value(self.spinner_value);
    }
}
