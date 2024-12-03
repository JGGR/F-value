use crate::core::*;
use raylib::RaylibHandle;
use raylib::consts::GuiDefaultProperty::TEXT_SIZE;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GuiControl::DEFAULT;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::ffi::CString;


pub fn update_main(rl : &mut RaylibHandle, main_state : &mut MainState, default_font_size : &mut i32, current_font_size : &mut i32) {
    main_state.should_quit = rl.window_should_close();

    main_state.frame_counter += 1;

    if main_state.reapply_theme {
        match main_state.theme {
            GuiTheme::Dark => {
                // Al momento, raylib-rs non espone una funzione gui_load_style_from_memory().
                // Qui simuliamo la disponibilità runtime di un file .rgs, partendo dai byte
                // di include_bytes!(). Non la migliore idea, ma sembra funzionare.
                load_style_from_memory(rl, DARK_THEME_DATA);
            }
            GuiTheme::Light => {
                rl.gui_load_style_default();
            }
        }
        *default_font_size = rl.gui_get_style(DEFAULT, TEXT_SIZE as i32);
        *current_font_size = *default_font_size;
        main_state.reapply_theme = false;
    }

    rl.gui_set_style(DEFAULT, TEXT_SIZE as i32, *current_font_size); // Update font size

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

fn write_temp_style_file(data: &[u8]) -> Result<(CString, PathBuf), Box<dyn std::error::Error>> {
    let mut temp_path = std::env::temp_dir();
    temp_path.push("temp_style.rgs");

    let mut file = File::create(&temp_path)?;
    file.write_all(data)?;

    let c_string = CString::new(temp_path.to_string_lossy().as_bytes())?;
    Ok((c_string, temp_path))
}

fn load_style_from_memory(rl: &mut RaylibHandle, data: &[u8]) {
     // Write the data to a temporary file
    let (temp_file_cstring, temp_file_path) = write_temp_style_file(data).expect("Failed to write temp style file");

    // Load the style
    rl.gui_load_style(Some(temp_file_cstring.as_c_str()));

    // Remove the temp file after loading the style
    std::fs::remove_file(temp_file_path).expect("Failed to delete temp style file");
}
