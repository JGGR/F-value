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

pub struct SelezioneIndiceView {

}

impl SelezioneIndiceView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &IndiceController, current_font_size : i32) {
        d.clear_background(Color::RAYWHITE);

        let button_niseci_width = propwidth(&d, 200);
        let button_niseci_x = d.get_screen_width() / 2 - button_niseci_width /2;
        let button_niseci_height = propwidth(&d, 50);

        let button_indice_y_spacing = button_niseci_height;

        let button_niseci_y = d.get_screen_height() / 2 - button_indice_y_spacing / 2 - button_niseci_height;

        let button_hfbi_width = button_niseci_width;
        let button_hfbi_x = button_niseci_x;
        let button_hfbi_height = button_niseci_height;
        let button_hfbi_y = button_niseci_y + button_niseci_height + button_indice_y_spacing;

        let groupbox_width = button_niseci_width + propwidth(&d, 100);
        let groupbox_x = button_niseci_x - propwidth(&d, 50);
        let groupbox_height = button_niseci_height * 2 + button_indice_y_spacing + propheight(&d, 100);
        let groupbox_y = button_niseci_y - propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Seleziona Indice"))
        );

        if d.gui_button(
            rrect(
                button_niseci_x,
                button_niseci_y,
                button_niseci_width,
                button_niseci_height
            ),
            Some(rstr!("NISECI"))
        ) {
            println!("TODO: call controller to update model. Controller can update main_state.current_view on next frame in update()");
        }

        if d.gui_button(
            rrect(
                button_hfbi_x,
                button_hfbi_y,
                button_hfbi_width,
                button_hfbi_height,
            ),
            Some(rstr!("HFBI"))
        ) {
            println!("TODO: call controller to update model. Controller can update main_state.current_view on next frame in update()");
        }

    }
}

pub struct SelezioneFileInputView {

}

impl SelezioneFileInputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

pub struct ValidazioneFileInputView {

}

impl ValidazioneFileInputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

pub struct SelezioneInfoAggiuntiveView {

}

impl SelezioneInfoAggiuntiveView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

pub struct ValidazioneInfoAggiuntiveView {

}

impl ValidazioneInfoAggiuntiveView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

pub struct ProduzioneOutputView {

}

impl ProduzioneOutputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

pub struct ProduzionePDFView {

}

impl ProduzionePDFView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, current_font_size : i32) {
        d.clear_background(Color::GRAY);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, current_font_size);
    }
}

fn rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    let rainbow_color = Color::new(red, green, blue, 255);
    return rainbow_color;
}

fn draw_todo_view_text(d: &mut RaylibDrawHandle, frame_counter: u32, current_font_size: i32) {

    let rainbow_speed = 0.01; // Smaller speed = slower cycle
    let rainbow_color = rainbow_color_from_framecounter(frame_counter, rainbow_speed);

    let todo_label = "TODO: Implement this View";
    let todo_label_font_size = current_font_size *2;
    let todo_label_x = d.get_screen_width() / 2 - d.measure_text(todo_label, todo_label_font_size) / 2;
    let todo_label_y = d.get_screen_height() / 2 - propheight(&d, todo_label_font_size) / 2;
    d.draw_text(todo_label, todo_label_x, todo_label_y, todo_label_font_size, rainbow_color);
}
