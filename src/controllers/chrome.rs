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

use crate::app::core::{
    get_locale, Action, CurrentView, GuiTheme, Localize, ASHES_THEME_DATA, BLUISH_THEME_DATA,
    CANDY_THEME_DATA, CHERRY_THEME_DATA, CYBER_THEME_DATA, DARK_THEME_DATA, EXIT_KEY,
    JUNGLE_THEME_DATA, LAVANDA_THEME_DATA, TERMINAL_THEME_DATA,
};
use crate::app::model::{AppModel, Model};
use crate::controllers::Controller;
use raylib::color::Color;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::KeyboardKey::*;
use raylib::error::LoadStyleFromMemoryError;
use raylib::prelude::RaylibGuiState;
use raylib::RaylibHandle;

pub(crate) struct ChromeController;

impl Controller for ChromeController {
    type SubModel = AppModel;

    fn update(&self, rl: &mut RaylibHandle, state: &mut Model, actions: &mut Vec<Action>) {
        state.app_model.should_quit = rl.window_should_close();

        state.app_model.frame_counter += 1;

        actions.retain(|a| {
            match a {
                Action::ShowInfo => {
                    state.app_model.showing_info_box = true;
                    false
                }
                Action::CloseInfo => {
                    state.app_model.showing_info_box = false;
                    false
                }
                Action::ShowLicense => {
                    state.app_model.showing_license_box = true;
                    false
                }
                Action::CloseLicense => {
                    state.app_model.showing_license_box = false;
                    false
                }
                Action::Reset => {
                    state.app_model.showing_reset_win = false;
                    state.app_model.should_reset = true;
                    false
                }
                Action::ShowReset => {
                    state.app_model.showing_reset_win = true;
                    false
                }
                Action::CloseReset => {
                    state.app_model.showing_reset_win = false;
                    false
                }
                Action::ResetSettings => {
                    state.app_model.showing_reset_win = false;
                    state.app_model.current_font_height = state.app_model.default_font_height;
                    rl.gui_set_style(DEFAULT, TEXT_SIZE, state.app_model.current_font_height);
                    state.app_model.gui_theme_combobox_active = GuiTheme::Light as i32;
                    if let Err(load_err) =
                        GuiTheme::load_and_set(&GuiTheme::Light, rl, &mut state.app_model)
                    {
                        let error_msg = format!("Error while loading theme: {load_err}");
                        self.add_console_message(state, error_msg);
                        state.app_model.set_current_view(CurrentView::Console);
                    };
                    let locale = get_locale();
                    state.app_model.locale_combobox_active = locale as i32;
                    state.app_model.locale = locale;
                    false
                }
                Action::ShowConsole => {
                    state.app_model.set_current_view(CurrentView::Console);
                    false
                }
                Action::CloseConsole => {
                    let prev = state.app_model.previous_view;
                    state.app_model.set_current_view(prev);
                    false
                }
                Action::OpenSettings => {
                    state.app_model.showing_settings_box = true;
                    false
                }
                Action::CloseSettings => {
                    state.app_model.showing_settings_box = false;
                    false
                }
                Action::SetLocale(locale_idx) => {
                    if *locale_idx != state.app_model.locale as i32 {
                        match <Localize as TryFrom<i32>>::try_from(*locale_idx) {
                            Ok(l) => {
                                state.app_model.locale = l;
                                state.app_model.locale_combobox_active = *locale_idx;
                            }
                            Err(_) => eprintln!("unknown number in current locale check"),
                        }
                    }
                    false
                }
                Action::SetFontHeight(height) => {
                    state.app_model.current_font_height = *height;
                    rl.gui_set_style(DEFAULT, TEXT_SIZE, state.app_model.current_font_height);
                    false
                }
                Action::SetTheme(theme_idx) => {
                    if *theme_idx != state.app_model.theme as i32 {
                        match <GuiTheme as TryFrom<i32>>::try_from(*theme_idx) {
                            Ok(t) => {
                                state.app_model.gui_theme_combobox_active = *theme_idx;
                                if let Err(load_err) = t.load_and_set(rl, &mut state.app_model) {
                                    let error_msg =
                                        format!("Error while loading theme: {load_err}");
                                    self.add_console_message(state, error_msg);
                                    state.app_model.set_current_view(CurrentView::Console);
                                };
                            }
                            Err(_) => eprintln!("unknown number in current theme check"),
                        }
                    }
                    false
                }
                Action::Quit => {
                    state.app_model.should_quit = true;
                    false
                }
                Action::CloseQuit => {
                    state.app_model.showing_quit_win = false;
                    false
                }
                _ => true, // keep in vec
            }
        });

        if rl.is_key_pressed(EXIT_KEY) {
            state.app_model.showing_quit_win = true;
        }

        if rl.is_key_down(KEY_LEFT_ALT) && rl.is_key_pressed(KEY_F) {
            rl.toggle_fullscreen();
        }

        if rl.is_key_pressed(KEY_F7) {
            state.app_model.showing_info_box = true;
        }
    }
}

impl ChromeController {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl GuiTheme {
    fn load_and_set(
        &self,
        rl: &mut RaylibHandle,
        main_state: &mut AppModel,
    ) -> Result<(), LoadStyleFromMemoryError> {
        match self {
            GuiTheme::Dark => {
                rl.gui_load_style_from_memory(DARK_THEME_DATA)?;
                main_state.theme = GuiTheme::Dark;
            }
            GuiTheme::Bluish => {
                rl.gui_load_style_from_memory(BLUISH_THEME_DATA)?;
                main_state.theme = GuiTheme::Bluish;
            }
            GuiTheme::Candy => {
                rl.gui_load_style_from_memory(CANDY_THEME_DATA)?;
                main_state.theme = GuiTheme::Candy;
            }
            GuiTheme::Cherry => {
                rl.gui_load_style_from_memory(CHERRY_THEME_DATA)?;
                main_state.theme = GuiTheme::Cherry;
            }
            GuiTheme::Cyber => {
                rl.gui_load_style_from_memory(CYBER_THEME_DATA)?;
                main_state.theme = GuiTheme::Cyber;
            }
            GuiTheme::Jungle => {
                rl.gui_load_style_from_memory(JUNGLE_THEME_DATA)?;
                main_state.theme = GuiTheme::Jungle;
            }
            GuiTheme::Lavanda => {
                rl.gui_load_style_from_memory(LAVANDA_THEME_DATA)?;
                main_state.theme = GuiTheme::Lavanda;
            }
            GuiTheme::Terminal => {
                rl.gui_load_style_from_memory(TERMINAL_THEME_DATA)?;
                main_state.theme = GuiTheme::Terminal;
            }
            GuiTheme::Ashes => {
                rl.gui_load_style_from_memory(ASHES_THEME_DATA)?;
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
        main_state.colors.default_txt_color = Color::get_color(txt_color_int);
        main_state.colors.default_bg_color = Color::get_color(bg_color_int);
        main_state.current_font = rl.gui_get_font();
        Ok(())
    }
}
