use crate::core::*;
use crate::controllers::*;
use raylib::prelude::*;

// A view responsible for rendering the state
// Tightly coupled with its respective controller
pub struct HomeView {
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl HomeView {
    pub fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &HomeController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(&d, 100) as f32, propheight(&d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color
        );


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 100), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
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
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl SecondView {
    pub fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &SecondController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(&d, 100) as f32, propheight(&d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color
        );


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 100), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
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

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, _controller: &IndiceController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

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

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);


        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

pub struct ValidazioneFileInputView {

}

impl ValidazioneFileInputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

pub struct SelezioneInfoAggiuntiveView {

}

impl SelezioneInfoAggiuntiveView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

pub struct ValidazioneInfoAggiuntiveView {

}

impl ValidazioneInfoAggiuntiveView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

pub struct ProduzioneOutputView {

}

impl ProduzioneOutputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

pub struct ProduzionePDFView {

}

impl ProduzionePDFView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        draw_todo_view_text(d, frame_counter, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height);
    }
}

fn rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    let rainbow_color = Color::new(red, green, blue, 255);
    return rainbow_color;
}

fn draw_todo_view_text(d: &mut RaylibDrawHandle, frame_counter: u32, font: &WeakFont, text_spacing: i32, current_font_height: i32) {

    let rainbow_speed = 0.03; // Smaller speed = slower cycle
    let rainbow_color = rainbow_color_from_framecounter(frame_counter, rainbow_speed);

    let todo_label = "TODO: Implement this View";
    let todo_label_font_height = current_font_height *2;
    let todo_label_x = d.get_screen_width() / 2 - d.measure_text(todo_label, todo_label_font_height) / 2;
    let todo_label_y = d.get_screen_height() / 2 - propheight(&d, todo_label_font_height) / 2;
    d.draw_text_ex(font, todo_label, Vector2::new(todo_label_x as f32, todo_label_y as f32), todo_label_font_height as f32, text_spacing as f32, rainbow_color);
}
