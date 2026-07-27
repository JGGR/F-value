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

use crate::app::core::{Action, CurrentView};
use crate::app::model::{
    ConsoleModel, FileInputModel, HelpModel, HomeModel, IndiceModel, InfoAggiuntiveModel, Model,
    OutputModel, SubModel,
};
use raylib::RaylibHandle;

pub(crate) mod chrome;
use chrome::ChromeController;
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
    pub(crate) chrome_controller: ChromeController,
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
            chrome_controller: ChromeController::new(),
            home_controller: HomeController::new(),
            help_controller: HelpController::new(),
            indice_controller: IndiceController::new(),
            fileinput_controller: FileInputController::new(),
            infoaggiuntive_controller: InfoAggiuntiveController::new(),
            output_controller: OutputController::new(),
            console_controller: ConsoleController::new(),
        }
    }
    pub(crate) fn update(
        &self,
        rl: &mut RaylibHandle,
        state: &mut Model,
        actions: &mut Vec<Action>,
    ) {
        // Base update step
        self.chrome_controller.update(rl, state, actions);
        // Current view update step
        match state.app_model.current_view {
            CurrentView::Home => {
                self.home_controller.update(rl, state, actions);
            }
            CurrentView::Help => {
                self.help_controller.update(rl, state, actions);
            }
            CurrentView::SelezioneIndice => {
                self.indice_controller.update(rl, state, actions);
            }
            CurrentView::SelezioneFileInput | CurrentView::ValidazioneFileInput => {
                self.fileinput_controller.update(rl, state, actions);
            }
            CurrentView::SelezioneInfoAggiuntive | CurrentView::ValidazioneInfoAggiuntive => {
                self.infoaggiuntive_controller.update(rl, state, actions);
            }
            CurrentView::ProduzioneOutput | CurrentView::ProduzionePDF => {
                self.output_controller.update(rl, state, actions);
            }
            CurrentView::Console => {
                self.console_controller.update(rl, state, actions);
            }
        }
    }
}

pub(crate) trait Controller {
    type SubModel: SubModel; // Associated type for controller substate
    fn update(&self, rl: &mut RaylibHandle, state: &mut Model, actions: &mut Vec<Action>);
    fn add_console_message(&self, state: &mut Model, msg: String) {
        state.console_model.console.add_message(msg);
    }
}
