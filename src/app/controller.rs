// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

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

use super::core::{
    get_locale, CurrentView, GuiTheme, Localize, MainAction, MainState, ASHES_THEME_DATA,
    BLUISH_THEME_DATA, CANDY_THEME_DATA, CHERRY_THEME_DATA, CYBER_THEME_DATA, DARK_THEME_DATA,
    EXIT_KEY, JUNGLE_THEME_DATA, LAVANDA_THEME_DATA, TERMINAL_THEME_DATA,
};
use raylib::color::Color;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::KeyboardKey::*;
use raylib::RaylibHandle;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

pub(crate) fn update_main(
    rl: &mut RaylibHandle,
    actions: &mut Vec<MainAction>,
    main_state: &mut MainState,
) {
    main_state.should_quit = rl.window_should_close();

    main_state.frame_counter += 1;

    for a in actions.drain(..) {
        match a {
            MainAction::ShowInfo => {
                main_state.showing_info_box = true;
            }
            MainAction::CloseInfo => {
                main_state.showing_info_box = false;
            }
            MainAction::ShowLicense => {
                main_state.showing_license_box = true;
            }
            MainAction::CloseLicense => {
                main_state.showing_license_box = false;
            }
            MainAction::ShowReset => {
                main_state.showing_reset_win = true;
            }
            MainAction::CloseReset => {
                main_state.showing_reset_win = false;
            }
            MainAction::ResetSettings => {
                main_state.showing_reset_win = false;
                main_state.current_font_height = main_state.default_font_height;
                rl.gui_set_style(DEFAULT, TEXT_SIZE, main_state.current_font_height);
                main_state.gui_theme_combobox_active = GuiTheme::Light as i32;
                GuiTheme::load_and_set(&GuiTheme::Light, rl, main_state);
                main_state.locale_combobox_active = get_locale() as i32;
            }
            MainAction::ShowConsole => {
                main_state.set_current_view(CurrentView::Console);
            }
            MainAction::CloseConsole => {
                let prev = main_state.previous_view;
                main_state.set_current_view(prev);
            }
            MainAction::OpenSettings => {
                main_state.showing_settings_box = true;
            }
            MainAction::CloseSettings => {
                main_state.showing_settings_box = false;
            }
            MainAction::SetLocale(locale_idx) => {
                if locale_idx != main_state.locale as i32 {
                    match <Localize as TryFrom<i32>>::try_from(locale_idx) {
                        Ok(l) => {
                            main_state.locale = l;
                            main_state.locale_combobox_active = locale_idx;
                        }
                        Err(_) => eprintln!("unknown number in current locale check"),
                    }
                }
            }
            MainAction::SetFontHeight(height) => {
                main_state.current_font_height = height;
                rl.gui_set_style(DEFAULT, TEXT_SIZE, main_state.current_font_height);
            }
            MainAction::SetTheme(theme_idx) => {
                if theme_idx != main_state.theme as i32 {
                    match <GuiTheme as TryFrom<i32>>::try_from(theme_idx) {
                        Ok(t) => {
                            main_state.gui_theme_combobox_active = theme_idx;
                            t.load_and_set(rl, main_state);
                        }
                        Err(_) => eprintln!("unknown number in current theme check"),
                    }
                }
            }
            MainAction::Quit => {
                eprintln!("MainController:  got action {}", a);
            }
            MainAction::CloseQuit => {
                eprintln!("MainController:  got action {}", a);
            }
        }
    }

    if rl.is_key_pressed(EXIT_KEY) {
        main_state.showing_quit_win = true;
    }

    if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_F) {
        rl.toggle_fullscreen();
    }

    if rl.is_key_pressed(KEY_F7) {
        main_state.showing_info_box = true;
    }
}

fn write_temp_style_file(data: &[u8]) -> Result<(String, PathBuf), Box<dyn std::error::Error>> {
    // We employ a UUID to randomise the filename, as required
    // to avoid insecure temporary files vulnerabilities
    // See: https://doc.rust-lang.org/nightly/std/env/fn.temp_dir.html
    let mut temp_path = std::env::temp_dir();
    let id = Uuid::new_v4();
    temp_path.push(format!("{}.rgs", id));

    let mut file = File::create(&temp_path)?;
    file.write_all(data)?;

    let string = String::from(temp_path.to_string_lossy());
    Ok((string, temp_path))
}

fn load_style_from_memory(rl: &mut RaylibHandle, data: &[u8]) {
    // Al momento, raylib-rs non espone una funzione gui_load_style_from_memory().
    // Qui simuliamo la disponibilità runtime di un file .rgs, partendo dai byte
    // di include_bytes!(). Non la migliore idea, ma sembra funzionare.

    // Write the data to a temporary file
    let (temp_file_cstring, temp_file_path) =
        write_temp_style_file(data).expect("Failed to write temp style file");

    // Load the style
    rl.gui_load_style(temp_file_cstring.as_str());

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
            _ => 1,
        };
        let font_spacing_scale = match self {
            GuiTheme::Light => 2,
            _ => 1,
        };
        main_state.default_font_height = rl.gui_get_style(DEFAULT, TEXT_SIZE) * font_height_scale;
        rl.gui_set_style(DEFAULT, TEXT_SIZE, main_state.default_font_height);
        main_state.current_font_height = main_state.default_font_height;
        main_state.default_txt_spacing =
            rl.gui_get_style(DEFAULT, TEXT_SPACING) * font_spacing_scale;
        rl.gui_set_style(DEFAULT, TEXT_SPACING, main_state.default_txt_spacing);
        let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL) as u32;
        let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR) as u32;
        main_state.default_txt_color = Color::get_color(txt_color_int);
        main_state.default_bg_color = Color::get_color(bg_color_int);
        main_state.current_font = rl.gui_get_font();
    }
}
