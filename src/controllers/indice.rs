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

pub(crate) struct IndiceController;
use crate::app::core::Action;
use crate::app::model::{Model, SubModel};
use crate::controllers::{Controller, CurrentView, IndiceModel};
use esox::domain::index::Indice;
use raylib::RaylibHandle;

impl Controller for IndiceController {
    type SubModel = IndiceModel;

    fn update(&self, _rl: &mut RaylibHandle, state: &mut Model, actions: &mut Vec<Action>) {
        state.indice_model.increment_frame_counter();

        if let Some(index) = state.indice_model.get_selected_index() {
            eprintln!("IndiceController:  L'utente ha selezionato indice {index}");
            eprintln!("IndiceController:  Let's update current view and go to SelezioneFileInput.");
            state
                .app_model
                .set_current_view(CurrentView::SelezioneFileInput)
        }
        if state.app_model.should_reset {
            eprintln!("IndiceController: Resetting");
            state.app_model.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.console_model.reset();
            state.app_model.set_current_view(CurrentView::Home);
        }

        actions.retain(|a| match a {
            Action::PickIndice(indice) => {
                self.set_indice_corrente(state, *indice);
                false
            }
            _ => {
                println!("IndiceController:  Got action {}", a);
                true
            }
        });
    }
}

impl IndiceController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_indice_corrente(&self, state: &mut Model, index: Indice) {
        state.indice_model.set_selected_index(Some(index));
    }
}
