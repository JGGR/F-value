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
// Controller to update and access the state
pub(crate) struct HomeController;

use super::{Controller, CurrentView, HomeModel};
use crate::app::model::SubModel;
use crate::state::GLOBAL_STATE;
use crate::MainState;
use raylib::RaylibHandle;

impl Controller for HomeController {
    type SubModel = HomeModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.increment_frame_counter();
        if state.home_model.get_user_continued() {
            eprintln!("HomeController:  L'utente ha premuto Continua");
            eprintln!("HomeController:  Let's update current view and go to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice)
        }
        if main_state.should_reset {
            eprintln!("HomeController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.console_model.reset();
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.home_model.clone()
    }
}

impl HomeController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_user_continued(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_user_continued(val);
    }
}
