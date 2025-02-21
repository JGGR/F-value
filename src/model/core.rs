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

use super::index::Indice;
use super::niseci::{RiferimentoNISECI, CampionamentoNISECI, AnagraficaNISECI, RisultatoNISECI};
use std::path::PathBuf;
use std::collections::HashMap;

use crate::console::*;
use crate::SHORT_PROJECT_VERSION;

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct HomeModel {
    value: i32,
    name: String,
}

impl HomeModel {
    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}


// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct SecondModel {
    value: i32,
    name: String,
}

impl SecondModel {
    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct IndiceModel {
    frame_counter: u32,
    selected_index: Option<Indice>
}

impl IndiceModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }

    pub fn set_selected_index(&mut self, index: Indice) -> () {
        self.selected_index = Some(index);
    }

    pub fn get_selected_index(&self) -> Option<Indice> {
        return self.selected_index;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct FileInputModel {
    frame_counter: u32,
    riferimento_path: Option<PathBuf>,
    riferimento_path_valid: bool,
    campionamento_path: Option<PathBuf>,
    campionamento_path_valid: bool,
    errors_occurred: bool,
}

impl FileInputModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    pub fn set_riferimento_path(&mut self, path: Option<PathBuf>) {
        self.riferimento_path = path;
    }

    pub fn get_riferimento_path(&self) -> Option<PathBuf> {
        return self.riferimento_path.clone();
    }

    pub fn set_riferimento_path_valid(&mut self, valid: bool) {
        self.riferimento_path_valid = valid;
    }

    pub fn get_riferimento_path_valid(&self) -> bool {
        return self.riferimento_path_valid;
    }

    pub fn set_campionamento_path(&mut self, path: Option<PathBuf>) {
        self.campionamento_path = path;
    }

    pub fn get_campionamento_path(&self) -> Option<PathBuf> {
        return self.campionamento_path.clone();
    }

    pub fn set_campionamento_path_valid(&mut self, valid: bool) {
        self.campionamento_path_valid = valid;
    }

    pub fn get_campionamento_path_valid(&self) -> bool {
        return self.campionamento_path_valid;
    }

    pub fn get_errors_occurred(&self) -> bool {
        return self.errors_occurred;
    }

    pub fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct InfoAggiuntiveModel {
    frame_counter: u32,
    done_editing: bool,
    valid: bool,
    errors_occurred: bool,
}

impl InfoAggiuntiveModel {
    pub fn get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }

    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }

    pub fn is_done_editing(&self) -> bool {
        return self.done_editing;
    }

    pub fn set_done_editing(&mut self, val: bool) {
        self.done_editing = val;
    }

    pub fn is_valid(&self) -> bool {
        return self.valid;
    }

    pub fn set_valid(&mut self, val: bool) {
        self.valid = val;
    }

    pub fn get_errors_occurred(&self) -> bool {
        return self.errors_occurred;
    }

    pub fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct OutputModel {
    frame_counter: u32,
}

impl OutputModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
}

#[derive(Clone)]
pub struct ConsoleModel {
    pub console: Console,
    name: String,
}

impl ConsoleModel {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

#[derive(Clone)]
pub struct DataModelNISECI {
    riferimento: Option<RiferimentoNISECI>,
    campionamento: Option<CampionamentoNISECI>,
    anagrafica: Option<AnagraficaNISECI>,
    risultato: Option<RisultatoNISECI>,
}

#[derive(Clone)]
pub struct DataModel {
    pub errors_occurred: bool,
    pub niseci: DataModelNISECI,
}

impl DataModel {
    pub fn new(niseci: DataModelNISECI) -> Self {
        Self {
            errors_occurred: false,
            niseci: niseci,
        }
    }
    pub fn get_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        return self.niseci.riferimento.clone();
    }
    pub fn set_riferimento_niseci(&mut self, riferimento: Option<RiferimentoNISECI>) {
        self.niseci.riferimento = riferimento;
    }
    pub fn get_campionamento_niseci(&self) -> Option<CampionamentoNISECI> {
        return self.niseci.campionamento.clone();
    }
    pub fn set_campionamento_niseci(&mut self, campionamento: Option<CampionamentoNISECI>) {
        self.niseci.campionamento = campionamento;
    }
    pub fn get_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        return self.niseci.anagrafica.clone();
    }
    pub fn set_anagrafica_niseci(&mut self, anagrafica: Option<AnagraficaNISECI>) {
        self.niseci.anagrafica = anagrafica;
    }
    pub fn get_risultato_niseci(&self) -> Option<RisultatoNISECI> {
        return self.niseci.risultato.clone();
    }
    pub fn set_risultato_niseci(&mut self, risultato: Option<RisultatoNISECI>) {
        self.niseci.risultato = risultato;
    }

    pub fn get_errors_occurred(&self) -> bool {
        return self.errors_occurred;
    }

    pub fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

#[derive(Clone)]
pub struct Model {
    pub home_model: HomeModel,
    pub second_model: SecondModel,
    pub indice_model: IndiceModel,
    pub fileinput_model: FileInputModel,
    pub infoaggiuntive_model: InfoAggiuntiveModel,
    pub output_model: OutputModel,
    pub console_model: ConsoleModel,
    pub data_model: DataModel,
}

impl Model {
    pub fn new() -> Self {
        Self {
            home_model: HomeModel {
                value: 0,
                name: "Initial".to_string(),
            },
            second_model: SecondModel {
                value: 1,
                name: "Initial".to_string(),
            },
            indice_model: IndiceModel {
                frame_counter: 0,
                selected_index: None
            },
            fileinput_model: FileInputModel {
                frame_counter: 0,
                riferimento_path: None,
                riferimento_path_valid: false,
                campionamento_path: None,
                campionamento_path_valid: false,
                errors_occurred: false,
            },
            infoaggiuntive_model: InfoAggiuntiveModel {
                frame_counter: 0,
                done_editing: false,
                valid: false,
                errors_occurred: false,
            },
            output_model: OutputModel {
                frame_counter: 0,
            },
            console_model: ConsoleModel {
                console: Console::new(
                    80, // Columns - chars per line
                    1000, // Max messages
                    17, // Max messages shown at a time
                    HashMap::<String,String>::from([
                        ("version".to_string(), SHORT_PROJECT_VERSION.to_string()),
                        ("riferimento_niseci".to_string(), "Vuoto".to_string()),
                        ("campionamento_niseci".to_string(), "Vuoto".to_string()),
                        ("anagrafica_niseci".to_string(), "Vuoto".to_string()),
                    ]),
                ),
                name: "Initial".to_string(),
            },
            data_model: DataModel {
                errors_occurred: false,
                niseci: DataModelNISECI {
                    riferimento: None,
                    campionamento: None,
                    anagrafica: None,
                    risultato: None,
                },
            }
        }
    }
}
