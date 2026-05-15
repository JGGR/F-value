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

use crate::console::Console;
use crate::core::SHORT_PROJECT_VERSION;
use esox::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, RisultatoHFBI, StatoEcologicoHFBI};
use esox::domain::index::Indice;
use esox::domain::niseci::{AnagraficaNISECI, RiferimentoNISECI, StatoEcologicoNISECI};
#[cfg(not(feature = "lessclone"))]
use esox::{
    domain::niseci::{CampionamentoNISECI, RisultatoNISECI},
    engines::niseci::full::calculate_stato_ecologico_niseci,
};

#[cfg(feature = "lessclone")]
use esox::{
    domain::niseci::lessclone::{CampionamentoNISECI, RisultatoNISECI},
    engines::niseci::full::lessclone::calculate_stato_ecologico_niseci,
};

use esox::engines::hfbi::full::calculate_stato_ecologico_hfbi;
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) trait SubModel {
    fn _get_frame_counter(&self) -> u32;
    fn increment_frame_counter(&mut self);
    fn reset(&mut self);
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct HomeModel {
    frame_counter: u32,
    user_continued: bool,
    user_wants_info: bool,
}

impl SubModel for HomeModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_user_continued(false);
        self.set_user_wants_info(false);
    }
}

impl HomeModel {
    pub(crate) fn get_user_continued(&self) -> bool {
        self.user_continued
    }
    pub(crate) fn set_user_continued(&mut self, val: bool) {
        self.user_continued = val;
    }
    pub(crate) fn get_user_wants_info(&self) -> bool {
        self.user_wants_info
    }
    pub(crate) fn set_user_wants_info(&mut self, val: bool) {
        self.user_wants_info = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct HelpModel {
    frame_counter: u32,
    user_continued: bool,
}

impl SubModel for HelpModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_user_continued(false);
    }
}

impl HelpModel {
    pub(crate) fn get_user_continued(&self) -> bool {
        self.user_continued
    }
    pub(crate) fn set_user_continued(&mut self, val: bool) {
        self.user_continued = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct IndiceModel {
    frame_counter: u32,
    selected_index: Option<Indice>,
}

impl SubModel for IndiceModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_selected_index(None);
    }
}

impl IndiceModel {
    pub(crate) fn set_selected_index(&mut self, index: Option<Indice>) {
        self.selected_index = index;
    }

    pub(crate) fn get_selected_index(&self) -> Option<Indice> {
        self.selected_index
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct FileInputModel {
    frame_counter: u32,
    riferimento_path: Option<PathBuf>,
    riferimento_path_valid: bool,
    campionamento_path: Option<PathBuf>,
    campionamento_path_valid: bool,
    errors_occurred: bool,
}

impl SubModel for FileInputModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_riferimento_path(None);
        self.set_campionamento_path(None);
        self.set_riferimento_path_valid(false);
        self.set_campionamento_path_valid(false);
        self.set_errors_occurred(false);
    }
}

impl FileInputModel {
    pub(crate) fn set_riferimento_path(&mut self, path: Option<PathBuf>) {
        self.riferimento_path = path;
    }

    pub(crate) fn get_riferimento_path(&self) -> Option<PathBuf> {
        self.riferimento_path.clone()
    }

    pub(crate) fn set_riferimento_path_valid(&mut self, valid: bool) {
        self.riferimento_path_valid = valid;
    }

    pub(crate) fn get_riferimento_path_valid(&self) -> bool {
        self.riferimento_path_valid
    }

    pub(crate) fn set_campionamento_path(&mut self, path: Option<PathBuf>) {
        self.campionamento_path = path;
    }

    pub(crate) fn get_campionamento_path(&self) -> Option<PathBuf> {
        self.campionamento_path.clone()
    }

    pub(crate) fn set_campionamento_path_valid(&mut self, valid: bool) {
        self.campionamento_path_valid = valid;
    }

    pub(crate) fn get_campionamento_path_valid(&self) -> bool {
        self.campionamento_path_valid
    }

    pub(crate) fn get_errors_occurred(&self) -> bool {
        self.errors_occurred
    }

    pub(crate) fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct InfoAggiuntiveModel {
    frame_counter: u32,
    done_editing: bool,
    valid: bool,
    errors_occurred: bool,
}

impl SubModel for InfoAggiuntiveModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }

    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_done_editing(false);
        self.set_valid(false);
        self.set_errors_occurred(false);
    }
}

impl InfoAggiuntiveModel {
    pub(crate) fn is_done_editing(&self) -> bool {
        self.done_editing
    }

    pub(crate) fn set_done_editing(&mut self, val: bool) {
        self.done_editing = val;
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.valid
    }

    pub(crate) fn set_valid(&mut self, val: bool) {
        self.valid = val;
    }

    pub(crate) fn get_errors_occurred(&self) -> bool {
        self.errors_occurred
    }

    pub(crate) fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub(crate) struct OutputModel {
    frame_counter: u32,
    done_calc: bool,
    done_user_confirm: bool,
    done_export: bool,
    should_reset: bool,
}

impl SubModel for OutputModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_done_calc(false);
        self.set_done_user_confirm(false);
        self.set_done_export(false);
    }
}

impl OutputModel {
    pub(crate) fn is_done_calc(&self) -> bool {
        self.done_calc
    }

    pub(crate) fn set_done_calc(&mut self, val: bool) {
        self.done_calc = val;
    }

    pub(crate) fn is_done_user_confirm(&self) -> bool {
        self.done_user_confirm
    }

    pub(crate) fn set_done_user_confirm(&mut self, val: bool) {
        self.done_user_confirm = val;
    }

    pub(crate) fn is_done_export(&self) -> bool {
        self.done_export
    }

    pub(crate) fn set_done_export(&mut self, val: bool) {
        self.done_export = val;
    }

    pub(crate) fn get_should_reset(&self) -> bool {
        self.should_reset
    }

    pub(crate) fn set_should_reset(&mut self, val: bool) {
        self.should_reset = val;
    }
}

#[derive(Clone)]
pub(crate) struct ConsoleModel {
    frame_counter: u32,
    pub(crate) console: Console,
    name: String,
    should_backout: bool,
}

impl SubModel for ConsoleModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_name("".to_string());
        self.set_should_backout(false);
        self.console.reset();
    }
}

impl ConsoleModel {
    pub(crate) fn _get_name(&self) -> String {
        self.name.clone()
    }

    pub(crate) fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub(crate) fn should_backout(&self) -> bool {
        self.should_backout
    }
    pub(crate) fn set_should_backout(&mut self, val: bool) {
        self.should_backout = val;
    }
}

#[derive(Clone)]
pub(crate) struct DataModelNISECI {
    riferimento: Option<RiferimentoNISECI>,
    campionamento: Option<CampionamentoNISECI>,
    anagrafica: Option<AnagraficaNISECI>,
    risultato: Option<RisultatoNISECI>,
}

#[derive(Clone)]
pub(crate) struct DataModelHFBI {
    campionamento: Option<CampionamentoHFBI>,
    anagrafica: Option<AnagraficaHFBI>,
    risultato: Option<RisultatoHFBI>,
}

#[derive(Clone)]
pub(crate) struct DataModel {
    frame_counter: u32,
    pub(crate) errors_occurred: bool,
    pub(crate) niseci: DataModelNISECI,
    pub(crate) hfbi: DataModelHFBI,
}

impl SubModel for DataModel {
    fn _get_frame_counter(&self) -> u32 {
        self.frame_counter
    }
    fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    fn reset(&mut self) {
        self.frame_counter = 0;
        self.set_riferimento_niseci(None);
        self.set_campionamento_niseci(None);
        self.set_anagrafica_niseci(None);
        self.set_risultato_niseci(None);
        self.set_campionamento_hfbi(None);
        self.set_anagrafica_hfbi(None);
        self.set_risultato_hfbi(None);
        self.set_errors_occurred(false);
    }
}

impl DataModel {
    pub(crate) fn _new(niseci: DataModelNISECI, hfbi: DataModelHFBI) -> Self {
        Self {
            frame_counter: 0,
            errors_occurred: false,
            niseci,
            hfbi,
        }
    }
    pub(crate) fn get_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        self.niseci.riferimento.clone()
    }
    pub(crate) fn set_riferimento_niseci(&mut self, riferimento: Option<RiferimentoNISECI>) {
        self.niseci.riferimento = riferimento;
    }
    pub(crate) fn get_campionamento_niseci(&self) -> Option<CampionamentoNISECI> {
        self.niseci.campionamento.clone()
    }
    pub(crate) fn set_campionamento_niseci(&mut self, campionamento: Option<CampionamentoNISECI>) {
        self.niseci.campionamento = campionamento;
    }
    pub(crate) fn get_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        self.niseci.anagrafica.clone()
    }
    pub(crate) fn set_anagrafica_niseci(&mut self, anagrafica: Option<AnagraficaNISECI>) {
        self.niseci.anagrafica = anagrafica;
    }
    pub(crate) fn get_risultato_niseci(&self) -> Option<RisultatoNISECI> {
        self.niseci.risultato.clone()
    }
    pub(crate) fn set_risultato_niseci(&mut self, risultato: Option<RisultatoNISECI>) {
        self.niseci.risultato = risultato;
    }

    pub(crate) fn get_x1_value(&self) -> Option<f32> {
        let opt_res = self.get_risultato_niseci();
        opt_res.map(|r| r.get_x1())
    }

    pub(crate) fn get_x2_value(&self) -> Option<f32> {
        let opt_res = self.get_risultato_niseci();
        opt_res.map(|r| r.get_x2())?
    }

    pub(crate) fn get_x3_value(&self) -> Option<f32> {
        let opt_res = self.get_risultato_niseci();
        opt_res.map(|r| r.get_x3())
    }

    pub(crate) fn get_niseci_value(&self) -> Option<f32> {
        match &self.niseci.risultato {
            Some(v) => v.get_valore(),
            None => None,
        }
    }

    pub(crate) fn get_rqe_niseci_value(&self) -> Option<f32> {
        match &self.niseci.risultato {
            Some(v) => v.get_rqe(),
            None => None,
        }
    }

    pub(crate) fn get_stato_eco_niseci_value(&self, state: &Model) -> Option<StatoEcologicoNISECI> {
        if state.output_model.is_done_calc() {
            let opt_res = state.data_model.get_risultato_niseci();
            match opt_res {
                Some(r) => {
                    let opt_anagrafica = state.data_model.get_anagrafica_niseci();
                    match opt_anagrafica {
                        Some(anagr) => {
                            let niseci_val = r.get_valore();
                            calculate_stato_ecologico_niseci(niseci_val, &anagr.area)
                        }
                        None => None,
                    }
                }
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_campionamento_hfbi(&mut self) -> Option<CampionamentoHFBI> {
        self.hfbi.campionamento.clone()
    }
    pub(crate) fn set_campionamento_hfbi(&mut self, campionamento: Option<CampionamentoHFBI>) {
        self.hfbi.campionamento = campionamento;
    }
    pub(crate) fn get_anagrafica_hfbi(&self) -> Option<AnagraficaHFBI> {
        self.hfbi.anagrafica.clone()
    }
    pub(crate) fn set_anagrafica_hfbi(&mut self, anagrafica: Option<AnagraficaHFBI>) {
        self.hfbi.anagrafica = anagrafica;
    }
    pub(crate) fn get_risultato_hfbi(&self) -> Option<RisultatoHFBI> {
        self.hfbi.risultato.clone()
    }
    pub(crate) fn set_risultato_hfbi(&mut self, risultato: Option<RisultatoHFBI>) {
        self.hfbi.risultato = risultato;
    }

    pub(crate) fn get_hfbi_value(&self) -> Option<f32> {
        match &self.hfbi.risultato {
            Some(v) => v.get_valore(),
            None => None,
        }
    }

    pub(crate) fn get_stato_eco_hfbi_value(&self, state: &Model) -> Option<StatoEcologicoHFBI> {
        if state.output_model.is_done_calc() {
            let opt_res = state.data_model.get_risultato_hfbi();
            match opt_res {
                Some(r) => {
                    let hfbi_val = r.get_valore();
                    calculate_stato_ecologico_hfbi(hfbi_val)
                }
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_errors_occurred(&self) -> bool {
        self.errors_occurred
    }

    pub(crate) fn set_errors_occurred(&mut self, val: bool) {
        self.errors_occurred = val;
    }
}

#[derive(Clone)]
pub(crate) struct Model {
    pub(crate) home_model: HomeModel,
    pub(crate) second_model: HelpModel,
    pub(crate) indice_model: IndiceModel,
    pub(crate) fileinput_model: FileInputModel,
    pub(crate) infoaggiuntive_model: InfoAggiuntiveModel,
    pub(crate) output_model: OutputModel,
    pub(crate) console_model: ConsoleModel,
    pub(crate) data_model: DataModel,
}

impl Model {
    pub(crate) fn new() -> Self {
        Self {
            home_model: HomeModel {
                frame_counter: 0,
                user_continued: false,
                user_wants_info: false,
            },
            second_model: HelpModel {
                frame_counter: 0,
                user_continued: false,
            },
            indice_model: IndiceModel {
                frame_counter: 0,
                selected_index: None,
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
                done_calc: false,
                done_user_confirm: false,
                done_export: false,
                should_reset: false,
            },
            console_model: ConsoleModel {
                frame_counter: 0,
                console: Console::new(
                    65,   // Columns - chars per line
                    1000, // Max messages
                    17,   // Max messages shown at a time
                    HashMap::<String, String>::from([
                        ("version".to_string(), SHORT_PROJECT_VERSION.to_string()),
                        ("riferimento_niseci".to_string(), "Vuoto".to_string()),
                        ("campionamento_niseci".to_string(), "Vuoto".to_string()),
                        ("anagrafica_niseci".to_string(), "Vuoto".to_string()),
                        ("risultato_niseci".to_string(), "Vuoto".to_string()),
                        ("campionamento_hfbi".to_string(), "Vuoto".to_string()),
                        ("anagrafica_hfbi".to_string(), "Vuoto".to_string()),
                        ("risultato_hfbi".to_string(), "Vuoto".to_string()),
                    ]),
                ),
                name: "Initial".to_string(),
                should_backout: false,
            },
            data_model: DataModel {
                frame_counter: 0,
                errors_occurred: false,
                niseci: DataModelNISECI {
                    riferimento: None,
                    campionamento: None,
                    anagrafica: None,
                    risultato: None,
                },
                hfbi: DataModelHFBI {
                    campionamento: None,
                    anagrafica: None,
                    risultato: None,
                },
            },
        }
    }
}
