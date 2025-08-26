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
pub(crate) struct ConsoleController;
use crate::app::model::SubModel;
use crate::controllers::{ConsoleModel, Controller, CurrentView};
use crate::state::GLOBAL_STATE;
use crate::MainState;
use raylib::consts::KeyboardKey::*;
use raylib::RaylibHandle;

impl Controller for ConsoleController {
    type SubModel = ConsoleModel;

    fn update(&self, rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        if main_state.should_reset {
            eprintln!("ConsoleController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.fileinput_model.reset();
            state.infoaggiuntive_model.reset();
            state.data_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }

        if state.console_model.should_backout() {
            state.console_model.set_should_backout(false);
            let prev = main_state.previous_view;
            main_state.set_current_view(prev);
            return;
        }

        // Handle input

        let mwheel_move = rl.get_mouse_wheel_move() as i32;

        if mwheel_move != 0 {
            if mwheel_move > 0 {
                // Positive is to scroll up
                state.console_model.console.scroll_up(mwheel_move as usize);
            } else {
                state
                    .console_model
                    .console
                    .scroll_down(-mwheel_move as usize);
            }
        }

        if rl.is_key_pressed(KEY_UP) {
            state.console_model.console.scroll_up(1);
        }
        if rl.is_key_pressed(KEY_DOWN) {
            state.console_model.console.scroll_down(1);
        }

        // Detect and pass keys to the console
        while let Some(c) = rl.get_char_pressed() {
            state
                .console_model
                .console
                .handle_input(rl, Some(c), false, false);
        }

        // Check for Enter key press
        if rl.is_key_pressed(KEY_ENTER) {
            state
                .console_model
                .console
                .handle_input(rl, None, true, false);
        }

        // Check for Backspace key press
        if rl.is_key_pressed(KEY_BACKSPACE) {
            state
                .console_model
                .console
                .handle_input(rl, None, false, true);
        }
        state.console_model.set_name("Updated".to_string());
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.console_model.clone()
    }
}

impl ConsoleController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn backout(&self) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.console_model.set_should_backout(true);
    }

    pub(crate) fn _set_console_env(&self, (key, val): (String, String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key, val));
    }
}
