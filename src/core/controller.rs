// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2025 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::core::*;
use raylib::RaylibHandle;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::KeyboardKey::*;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use std::ffi::CString;
use uuid::Uuid;
use std::io::Write;

pub fn update_main(rl: &mut RaylibHandle, main_state: &mut MainState) {
    main_state.should_quit = rl.window_should_close();

    main_state.frame_counter += 1;

    let current_theme_idx = main_state.gui_theme_combobox_active;

    if current_theme_idx != main_state.theme as i32 {
        match <i32 as TryInto<GuiTheme>>::try_into(current_theme_idx) {
            Ok(theme) => {
                theme.load_and_set(rl, main_state);
            }
            Err(_) => eprintln!("unknown number"),
        }
    }

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
    // We employ a UUID to randomise the filename, as required
    // to avoid insecure temporary files vulnerabilities
    // See: https://doc.rust-lang.org/nightly/std/env/fn.temp_dir.html
    let mut temp_path = std::env::temp_dir();
    let id = Uuid::new_v4();
    temp_path.push(format!("{}.rgs", id));

    let mut file = File::create(&temp_path)?;
    file.write_all(data)?;

    let c_string = CString::new(temp_path.to_string_lossy().as_bytes())?;
    Ok((c_string, temp_path))
}

fn load_style_from_memory(rl: &mut RaylibHandle, data: &[u8]) {
    // Al momento, raylib-rs non espone una funzione gui_load_style_from_memory().
    // Qui simuliamo la disponibilità runtime di un file .rgs, partendo dai byte
    // di include_bytes!(). Non la migliore idea, ma sembra funzionare.

    // Write the data to a temporary file
    let (temp_file_cstring, temp_file_path) = write_temp_style_file(data).expect("Failed to write temp style file");

    // Load the style
    rl.gui_load_style(Some(temp_file_cstring.as_c_str()));

    // Remove the temp file after loading the style
    std::fs::remove_file(temp_file_path).expect("Failed to delete temp style file");
}

impl GuiTheme {
    fn load_and_set(&self, rl: &mut RaylibHandle, main_state: &mut MainState) {
        match self {
            GuiTheme::Dark => {
                load_style_from_memory(rl, DARK_THEME_DATA);
                main_state.theme = GuiTheme::Dark;
            }
            GuiTheme::Bluish => {
                load_style_from_memory(rl, BLUISH_THEME_DATA);
                main_state.theme = GuiTheme::Bluish;
            }
            GuiTheme::Candy => {
                load_style_from_memory(rl, CANDY_THEME_DATA);
                main_state.theme = GuiTheme::Candy;
            }
            GuiTheme::Cherry => {
                load_style_from_memory(rl, CHERRY_THEME_DATA);
                main_state.theme = GuiTheme::Cherry;
            }
            GuiTheme::Cyber => {
                load_style_from_memory(rl, CYBER_THEME_DATA);
                main_state.theme = GuiTheme::Cyber;
            }
            GuiTheme::Jungle => {
                load_style_from_memory(rl, JUNGLE_THEME_DATA);
                main_state.theme = GuiTheme::Jungle;
            }
            GuiTheme::Lavanda => {
                load_style_from_memory(rl, LAVANDA_THEME_DATA);
                main_state.theme = GuiTheme::Lavanda;
            }
            GuiTheme::Terminal => {
                load_style_from_memory(rl, TERMINAL_THEME_DATA);
                main_state.theme = GuiTheme::Terminal;
            }
            GuiTheme::Ashes => {
                load_style_from_memory(rl, ASHES_THEME_DATA);
                main_state.theme = GuiTheme::Ashes;
            }
            GuiTheme::Light => {
                rl.gui_load_style_default();
                main_state.theme = GuiTheme::Light;
            }
        }
        let font_height_scale = match self {
            GuiTheme::Light => 2, // 10 is way too small for the default font height
            _ => 1
        };
        main_state.default_font_height = rl.gui_get_style(DEFAULT, TEXT_SIZE as i32) * font_height_scale;
        rl.gui_set_style(DEFAULT, TEXT_SIZE as i32, main_state.default_font_height);
        main_state.current_font_height = main_state.default_font_height;
        main_state.default_txt_spacing = rl.gui_get_style(DEFAULT, TEXT_SPACING as i32);
        let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL as i32);
        let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR as i32);
        main_state.default_txt_color = Color::get_color(txt_color_int as u32);
        main_state.default_bg_color = Color::get_color(bg_color_int as u32);
        main_state.current_font = rl.gui_get_font();
    }
}
