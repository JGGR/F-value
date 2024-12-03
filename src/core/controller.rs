use crate::core::*;
use raylib::RaylibHandle;
use raylib::consts::GuiDefaultProperty::TEXT_SIZE;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GuiControl::DEFAULT;

pub fn update_main(rl : &mut RaylibHandle, main_state : &mut MainState, current_font_size : i32) {
    main_state.should_quit = rl.window_should_close();

    main_state.frame_counter += 1;

    rl.gui_set_style(DEFAULT, TEXT_SIZE as i32, current_font_size); // Update font size

    if rl.is_key_pressed(crate::EXIT_KEY) {
        main_state.showing_quit_win = true;
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_F) {
        rl.toggle_fullscreen();
    }

    if rl.is_key_pressed(KEY_F7) {
        main_state.showing_info_box = true;
    }
}
