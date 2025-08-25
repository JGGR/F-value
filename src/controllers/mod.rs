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

use crate::app::core::{CurrentView, MainState};
use crate::app::model::{
    ConsoleModel, FileInputModel, HomeModel, IndiceModel, InfoAggiuntiveModel, OutputModel,
    SecondModel, SubModel,
};
use crate::domain::{
    index::Indice,
};
use crate::state::GLOBAL_STATE;
use raylib::RaylibHandle;

pub(crate) mod home;
use home::HomeController;
pub(crate) mod indice;
use indice::IndiceController;
pub(crate) mod file_input;
use file_input::FileInputController;
pub(crate) mod info_aggiuntive;
use info_aggiuntive::InfoAggiuntiveController;
pub(crate) mod output;
use output::OutputController;
pub(crate) mod console;
use console::ConsoleController;

pub(crate) struct Controllers {
    pub(crate) home_controller: HomeController,
    pub(crate) second_controller: SecondController,
    pub(crate) indice_controller: IndiceController,
    pub(crate) fileinput_controller: FileInputController,
    pub(crate) infoaggiuntive_controller: InfoAggiuntiveController,
    pub(crate) output_controller: OutputController,
    pub(crate) console_controller: ConsoleController,
}

impl Controllers {
    pub(crate) fn new() -> Self {
        Self {
            home_controller: HomeController::new(),
            second_controller: SecondController::new(),
            indice_controller: IndiceController::new(),
            fileinput_controller: FileInputController::new(),
            infoaggiuntive_controller: InfoAggiuntiveController::new(),
            output_controller: OutputController::new(),
            console_controller: ConsoleController::new(),
        }
    }
    pub(crate) fn update(&self, rl: &mut RaylibHandle, main_state: &mut MainState) {
        // Current view update step
        match main_state.current_view {
            CurrentView::Home => {
                self.home_controller.update(rl, main_state);
            }
            CurrentView::Second => {
                self.second_controller.update(rl, main_state);
            }
            CurrentView::SelezioneIndice => {
                self.indice_controller.update(rl, main_state);
            }
            CurrentView::SelezioneFileInput | CurrentView::ValidazioneFileInput => {
                self.fileinput_controller.update(rl, main_state);
            }
            CurrentView::SelezioneInfoAggiuntive | CurrentView::ValidazioneInfoAggiuntive => {
                self.infoaggiuntive_controller.update(rl, main_state);
            }
            CurrentView::ProduzioneOutput | CurrentView::ProduzionePDF => {
                self.output_controller.update(rl, main_state);
            }
            CurrentView::Console => {
                self.console_controller.update(rl, main_state);
            }
        }
    }
}

pub(crate) trait Controller {
    type SubModel: SubModel; // Associated type for controller substate
    fn update(&self, rl: &mut RaylibHandle, main_state: &mut MainState);
    fn get_state(&self) -> Self::SubModel;
    fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.console_model.console.add_message(msg);
    }
    fn get_current_index(&self) -> Option<Indice> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.get_selected_index()
    }
}

// Controller to update and access the state

pub(crate) struct SecondController;

impl Controller for SecondController {
    type SubModel = SecondModel;
    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.increment_frame_counter();
        state.second_model.set_name("Updated".to_string());
        if state.second_model.get_user_continued() {
            eprintln!("SecondController:  L'utente ha premuto Continua");
            eprintln!("SecondController:  Let's update current view and go to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice)
        }
        if main_state.should_reset {
            eprintln!("SecondController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.second_model.clone()
    }
}

impl SecondController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name(name);
    }

    pub(crate) fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_value(val);
    }
    pub(crate) fn set_user_continued(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_user_continued(val);
    }
}
