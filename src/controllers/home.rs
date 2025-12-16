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
use crate::app::core::Action;
use crate::app::model::{Model, SubModel};
use crate::MainState;
use raylib::RaylibHandle;

impl Controller for HomeController {
    type SubModel = HomeModel;

    fn update(
        &self,
        _rl: &mut RaylibHandle,
        state: &mut Model,
        actions: &mut Vec<Action>,
        main_state: &mut MainState,
    ) {
        state.home_model.increment_frame_counter();
        if state.home_model.get_user_continued() {
            eprintln!("HomeController:  L'utente ha premuto Continua");
            eprintln!("HomeController:  Let's update current view and go to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice)
        }
        if state.home_model.get_user_wants_info() {
            eprintln!("HomeController:  L'utente ha premuto Info");
            eprintln!("HomeController:  Let's update current view and go to Help.");
            main_state.set_current_view(CurrentView::Help)
        }

        if main_state.should_reset {
            eprintln!("HomeController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.console_model.reset();
        }

        for a in actions.drain(..) {
            match a {
                Action::UserContinued => {
                    self.set_user_continued(state, true);
                }
                Action::UserWantsInfo => {
                    self.set_user_wants_info(state, true);
                }
                _ => {
                    println!("HomeController:  Got action {}", a);
                }
            }
        }
    }
}

impl HomeController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_user_continued(&self, state: &mut Model, val: bool) {
        state.home_model.set_user_continued(val);
    }
    pub(crate) fn set_user_wants_info(&self, state: &mut Model, val: bool) {
        state.home_model.set_user_wants_info(val);
    }
}
