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

pub(crate) struct IndiceController;
use crate::app::model::SubModel;
use crate::controllers::{Controller, CurrentView, IndiceModel};
use crate::domain::index::Indice;
use crate::state::GLOBAL_STATE;
use crate::MainState;
use raylib::RaylibHandle;

impl Controller for IndiceController {
    type SubModel = IndiceModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.increment_frame_counter();

        if let Some(index) = state.indice_model.get_selected_index() {
            eprintln!("IndiceController:  L'utente ha selezionato indice {index}");
            eprintln!("IndiceController:  Let's update current view and go to SelezioneFileInput.");
            main_state.set_current_view(CurrentView::SelezioneFileInput)
        }
        if main_state.should_reset {
            eprintln!("IndiceController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
        }
    }
    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.clone()
    }
}

impl IndiceController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_indice_corrente(&self, index: Indice) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.set_selected_index(Some(index));
    }
}
