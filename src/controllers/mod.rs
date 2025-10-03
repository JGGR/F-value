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
    HelpModel, SubModel,
};
use crate::domain::index::Indice;
use crate::state::GLOBAL_STATE;
use raylib::RaylibHandle;

pub(crate) mod home;
use home::HomeController;
pub(crate) mod help;
use help::HelpController;
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
    pub(crate) help_controller: HelpController,
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
            help_controller: HelpController::new(),
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
            CurrentView::Help => {
                self.help_controller.update(rl, main_state);
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
