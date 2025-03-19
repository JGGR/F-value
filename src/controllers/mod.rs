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

use crate::model::core::*;
use crate::core::{MainState, parse_date, TipoRecordCsv, VeryItalianRecordCsvRiferimentoNISECI, VeryItalianRecordCsvCampionamentoNISECI, check_campionamento_niseci_path, check_riferimento_niseci_path, check_records_riferimento_niseci, check_records_campionamento_niseci};
use crate::model::index::Indice;
use crate::model::niseci::{RiferimentoNISECI, CampionamentoNISECI, AnagraficaNISECI, TipoComunitaNISECI, RisultatoNISECI, StatoEcologicoNISECI};
use crate::state::GLOBAL_STATE;
use crate::CurrentView;
use crate::process_csv_errors;
use crate::engines::niseci::full::{calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico};
use raylib::RaylibHandle;
use std::path::PathBuf;
use raylib::consts::KeyboardKey::*;
use chrono::format::ParseErrorKind;

pub struct Controllers {
    pub(crate) home_controller: HomeController,
    pub(crate) second_controller: SecondController,
    pub(crate) indice_controller: IndiceController,
    pub(crate) fileinput_controller: FileInputController,
    pub(crate) infoaggiuntive_controller: InfoAggiuntiveController,
    pub(crate) output_controller: OutputController,
    pub(crate) console_controller: ConsoleController
}

impl Controllers {
    pub fn new() -> Self {
        Self {
            home_controller: HomeController::new(),
            second_controller: SecondController::new(),
            indice_controller: IndiceController::new(),
            fileinput_controller: FileInputController::new(),
            infoaggiuntive_controller: InfoAggiuntiveController::new(),
            output_controller: OutputController::new(),
            console_controller: ConsoleController::new()
        }
    }
    pub fn update(&self, rl: &mut RaylibHandle, main_state: &mut MainState) {
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
            CurrentView::ProduzioneOutput | CurrentView::ProduzionePDF=> {
                self.output_controller.update(rl, main_state);
            }
            CurrentView::Console => {
                self.console_controller.update(rl, main_state);
            }
        }
    }
}

pub trait Controller {
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
pub struct HomeController;

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
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.home_model.clone()
    }
}

impl HomeController {
    pub fn new() -> Self {
        Self
    }

    pub fn set_user_continued(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_user_continued(val);
    }
}

pub struct SecondController;

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
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.second_model.clone()
    }
}

impl SecondController {

    pub fn new() -> Self {
        Self
    }

    pub fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name(name);
    }

    pub fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_value(val);
    }
    pub fn set_user_continued(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_user_continued(val);
    }
}

pub struct IndiceController;

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
    }
    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.clone()
    }
}

impl IndiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn set_indice_corrente(&self, index: Indice) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.set_selected_index(index);
    }
}

pub struct FileInputController;

impl Controller for FileInputController {
    type SubModel = FileInputModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.increment_frame_counter();

        if state.fileinput_model.get_errors_occurred() {
            eprintln!("FileInputController:  Errors occurred");
            eprintln!("FileInputController:  Let's update current view and go to CONSOLE.");
            main_state.set_current_view(CurrentView::Console);
            eprintln!("FileInputController:  Clearing error state");
            state.fileinput_model.set_errors_occurred(false);
        }

        let current_indice;
        if let Some(idx) = state.indice_model.get_selected_index() {
            current_indice = idx;
        } else {
            eprintln!("FileInputController:  User did not select an index");
            eprintln!("FileInputController:  Let's update current view and go back to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice);
            return;
        }
        match main_state.current_view {
            CurrentView::SelezioneFileInput => {
                match current_indice {
                    Indice::Niseci => {
                        let mut riferimento_ready = false;

                        if let Some(_rif_path) = state.fileinput_model.get_riferimento_path() {
                            riferimento_ready = true;
                        }
                        let mut campionamento_ready = false;
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            // Assumes the path is ready to be used.
                            // The current selection used by the only forces .csv extension
                            campionamento_ready = true;
                        }

                        if riferimento_ready && campionamento_ready {
                            eprintln!("FileInputController:  NISECI - L'utente ha fornito riferimento e campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to ValidazioneFileInput.");
                            main_state.set_current_view(CurrentView::ValidazioneFileInput);
                        }
                    }
                    Indice::Hfbi => {
                        let mut campionamento_ready = false;
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            // Assumes the path is ready to be used.
                            // The current selection used by the only forces .csv extension
                            campionamento_ready = true;
                        }
                        if campionamento_ready {
                            eprintln!("FileInputController:  HFBI - L'utente ha fornito campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to ValidazioneFileInput.");
                            main_state.set_current_view(CurrentView::ValidazioneFileInput);
                        }
                    }
                }
            },
            CurrentView::ValidazioneFileInput => {
                match current_indice {
                    Indice::Niseci => {
                        if let Some(_rif_path) = state.fileinput_model.get_riferimento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a riferimento niseci path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }

                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento niseci path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }

                        let riferimento_valid = state.fileinput_model.get_riferimento_path_valid();
                        let campionamento_valid = state.fileinput_model.get_campionamento_path_valid();

                        if riferimento_valid && campionamento_valid {
                            eprintln!("FileInputController:  NISECI - L'utente ha validato riferimento e campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            //self.add_console_message(format!("FileInputController:  NISECI - L'utente ha validato riferimento e campionamento"));
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                    Indice::Hfbi => {
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento hfbi path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }
                        let campionamento_valid = state.fileinput_model.get_campionamento_path_valid();

                        if campionamento_valid {
                            eprintln!("FileInputController:  HFBI - L'utente ha validato campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            self.add_console_message("FileInputController:  HFBI - L'utente ha validato campionamento".to_string());
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                }
            },
            _ => {},
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.clone()
    }

}

impl FileInputController {
    pub fn new() -> Self {
        Self
    }

    pub fn get_riferimento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_riferimento_path()
    }

    pub fn set_riferimento_path(&self, riferimento_path: Option<PathBuf>) {
        if let Some(ref rif_path) = riferimento_path {
            self.add_console_message(format!("FileInputController:  Selezione percorso riferimento: {{{}}}", rif_path.display()));
        } else {
            self.add_console_message("FileInputController:  Deselezione percorso riferimento".to_string());
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_riferimento_path(riferimento_path);
        state.fileinput_model.set_riferimento_path_valid(false); // Refresh the validity
    }

    pub fn get_riferimento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_riferimento_path_valid()
    }

    fn set_data_riferimento_niseci(&self, riferimento: RiferimentoNISECI) {
        self.set_console_env(("riferimento_niseci".to_string(), format!("{riferimento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_riferimento_niseci(Some(riferimento));
        state.fileinput_model.set_riferimento_path_valid(true);
    }

    pub fn get_data_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_riferimento_niseci()
    }

    pub fn valida_riferimento_niseci_path(&self) {
        if let Some(path) = self.get_riferimento_path() {
            // Using italian deser for now
            let csv_check = check_riferimento_niseci_path::<VeryItalianRecordCsvRiferimentoNISECI>(path);

            match csv_check {
                Ok(records) => {
                    let records_check = check_records_riferimento_niseci(records);

                    match records_check {
                        Ok(species) => {
                            self.add_console_message("FileInputController:  Validazione RiferimentoNISECI completata!".to_string());
                            let riferimento = RiferimentoNISECI::new(species);
                            self.set_data_riferimento_niseci(riferimento);
                        }
                        Err(errors) => { // Value errors
                            for e in errors {
                                self.add_console_message(format!("FileInputController:  {e}"));
                            }
                            let mut state = GLOBAL_STATE.lock().unwrap();
                            state.fileinput_model.set_errors_occurred(true);
                        }
                    }
                }
                Err(errors) => { // Csv errors
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    let processed_errors = process_csv_errors(&errors, TipoRecordCsv::RiferimentoNISECI);
                    for e in processed_errors {
                        self.add_console_message(format!("FileInputController:  {e}"));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }

    pub fn get_campionamento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_campionamento_path()
    }

    pub fn set_campionamento_path(&self, campionamento_path: Option<PathBuf>) {
        if let Some(ref camp_path) = campionamento_path {
            self.add_console_message(format!("FileInputController:  Selezione percorso campionamento: {{{}}}", camp_path.display()));
        } else {
            self.add_console_message("FileInputController:  Deselezione percorso campionamento".to_string());
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_campionamento_path(campionamento_path);
        state.fileinput_model.set_campionamento_path_valid(false); // Refresh the validity
    }

    pub fn _get_campionamento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_campionamento_path_valid()
    }

    fn set_data_campionamento_niseci(&self, campionamento: CampionamentoNISECI) {
        self.set_console_env(("campionamento_niseci".to_string(), format!("{campionamento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_campionamento_niseci(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub fn _get_data_campionamento_niseci(&self) -> Option<CampionamentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_campionamento_niseci()
    }

    pub fn valida_campionamento_niseci_path(&self) {
        if let Some(path) = self.get_campionamento_path() {
            // Using italian deser for now
            let csv_check = check_campionamento_niseci_path::<VeryItalianRecordCsvCampionamentoNISECI>(path);

            match csv_check {
                Ok(records) => {
                    let opt_riferimento_niseci = self.get_data_riferimento_niseci();
                    //NOTE: no double locking is not allowed! If state is
                    // still in scope, its lock has not been dropped yet.
                    //A scope is mandatory to ensure the lock is dropped before calling any
                    //method on self which would try to acquire a lock itself.
                    //This is a valid example:
                    //
                    //{
                    //    let mut state = GLOBAL_STATE.lock().unwrap();
                    //    opt_riferimento_niseci = state.data_model.get_riferimento_niseci();
                    //}
                    //But instead we tuck the lock acquisition inside the
                    //self.get_data_riferimento_niseci() and we chill.
                    if let Some(riferimento_niseci) = opt_riferimento_niseci {
                        let records_check = check_records_campionamento_niseci(records, riferimento_niseci.elenco_specie);
                        match records_check {
                            Ok(campioni) => {
                                self.add_console_message("FileInputController:  Validazione CampionamentoNISECI completata!".to_string());
                                let campionamento = CampionamentoNISECI::new(campioni);
                                self.set_data_campionamento_niseci(campionamento);
                            }
                            Err(errors) => { // Value errors
                                for e in errors {
                                    self.add_console_message(format!("FileInputController:  {e}"));
                                }
                                let mut state = GLOBAL_STATE.lock().unwrap();
                                state.fileinput_model.set_errors_occurred(true);
                            }
                        }
                    } else {
                        let error_msg = "Impossibile validare campionamento_niseci senza avere riferimento";
                        eprintln!("{error_msg}");
                        self.add_console_message(format!("FileInputController:  {error_msg}"));
                        let mut state = GLOBAL_STATE.lock().unwrap();
                        state.fileinput_model.set_errors_occurred(true);
                    }
                }
                Err(errors) => { // Csv errors
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    let processed_errors = process_csv_errors(&errors, TipoRecordCsv::CampionamentoNISECI);
                    for e in processed_errors {
                        self.add_console_message(format!("FileInputController:  {e}"));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }
    pub fn set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }
}

pub struct InfoAggiuntiveController;

impl Controller for InfoAggiuntiveController {
    type SubModel = InfoAggiuntiveModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.increment_frame_counter();

        if state.infoaggiuntive_model.get_errors_occurred() {
            eprintln!("InfoAggiuntiveController:  Errors occurred");
            eprintln!("InfoAggiuntiveController:  Let's update current view and go to CONSOLE.");
            main_state.set_current_view(CurrentView::Console);
            eprintln!("InfoAggiuntiveController:  Clearing error state");
            state.infoaggiuntive_model.set_errors_occurred(false);
        }

        let current_indice;
        if let Some(idx) = state.indice_model.get_selected_index() {
            current_indice = idx;
        } else {
            eprintln!("InfoAggiuntiveController:  User did not select an index");
            eprintln!("InfoAggiuntiveController:  Let's update current view and go back to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice);
            return;
        }
        match main_state.current_view {
            CurrentView::SelezioneInfoAggiuntive => {
                match current_indice {
                    Indice::Niseci => {
                        if state.infoaggiuntive_model.is_done_editing() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go to ValidaInfoAggiuntive");
                            main_state.set_current_view(CurrentView::ValidazioneInfoAggiuntive);
                        }
                    }
                    Indice::Hfbi => {
                    }
                }
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                match current_indice {
                    Indice::Niseci => {
                        if !state.infoaggiuntive_model.is_done_editing() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go back to SelezionaInfoAggiuntive");
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                        if state.infoaggiuntive_model.is_valid() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go to ProduzioneOutput");
                            main_state.set_current_view(CurrentView::ProduzioneOutput);
                        }
                    }
                    Indice::Hfbi => {
                    }
                }
            }
            _ => {}
        }

    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.clone()
    }
}

impl InfoAggiuntiveController {
    pub fn new() -> Self {
        Self
    }

    pub fn get_data_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_anagrafica_niseci()
    }

    pub fn submit_anagrafica_niseci(&self, anagrafica: AnagraficaNISECI) {
        self.set_console_env(("anagrafica_niseci".to_string(), format!("{anagrafica}")));
        self.add_console_message("InfoAggiuntiveController: L'utente ha completato l'inserimento info aggiuntive.".to_string());
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_niseci().is_none());
        state.data_model.set_anagrafica_niseci(Some(anagrafica));
        state.infoaggiuntive_model.set_done_editing(true);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub fn check_larghezza_stazione_string(&self, larghezza: &str) -> Result<f32,String> {
        let s = larghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => {
                Ok(value)
            }
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione larghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg.replace("invalid float literal", "tipo non valido: atteso decimale");
                }
                self.add_console_message(format!("InfoAggiuntiveController:  {err_msg}"));
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.data_model.set_anagrafica_niseci(None);
                state.infoaggiuntive_model.set_done_editing(false);
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                Err(err_msg)
            }
        }
    }

    pub fn check_lunghezza_stazione_string(&self, lunghezza: &str) -> Result<f32,String> {
        let s = lunghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => {
                Ok(value)
            }
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione lunghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg.replace("invalid float literal", "tipo non valido: atteso decimale");
                }
                self.add_console_message(format!("InfoAggiuntiveController:  {err_msg}"));
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.data_model.set_anagrafica_niseci(None);
                state.infoaggiuntive_model.set_done_editing(false);
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                Err(err_msg)
            }
        }
    }

    pub fn valida_anagrafica_niseci(&self) {

        //We grab the state in a scope to ensure we don't get lock problems
        {
            let state = GLOBAL_STATE.lock().unwrap();
            assert!(state.infoaggiuntive_model.is_done_editing());
        }

        if let Some(anagrafica) = self.get_data_anagrafica_niseci() {
            let mut errors: Vec<String> = Vec::new();

            if anagrafica.codice_stazione.is_empty() {
                errors.push("Codice stazione troppo corto".to_string());
            }

            if anagrafica.corpo_idrico.is_empty() {
                errors.push("Nome fiume troppo corto".to_string());
            }

            if anagrafica.posizione.regione.is_empty() {
                errors.push("Nome regione troppo corto".to_string());
            }

            if anagrafica.posizione.provincia.is_empty() {
                errors.push("Nome provincia troppo corto".to_string());
            }

            match parse_date(&anagrafica.date_string) {
                Ok(_) => {},
                Err(e) => {
                    match e.kind() {
                        ParseErrorKind::OutOfRange => {
                            errors.push("Data fornita non valida: fuori range".to_string());
                        },
                        ParseErrorKind::Impossible => {
                            errors.push("Data fornita non valida: valori non possibili".to_string());
                        },
                        ParseErrorKind::NotEnough => {
                            errors.push("Data fornita non valida: specifica insufficiente".to_string());
                        },
                        ParseErrorKind::Invalid => {
                            errors.push("Data fornita non valida: presenza di caratteri non attesi".to_string());
                        },
                        ParseErrorKind::TooShort => {
                            errors.push("Data fornita non valida: terminazione prematura dell'input".to_string());
                        },
                        ParseErrorKind::TooLong => {
                            errors.push("Data fornita non valida: input in eccesso".to_string());
                        },
                        ParseErrorKind::BadFormat => {
                            errors.push("Data fornita non valida: errore nella specifica di formattazione".to_string());
                        },
                        _ => {
                            errors.push("Data fornita non valida: errore sconosciuto".to_string());
                        }
                    }
                }
            }

            if (anagrafica.get_lunghezza_media() - 0.0) < 1e-6 {
                errors.push(format!("Lunghezza media troppo bassa: {}", anagrafica.get_lunghezza_media()));
            }

            if (anagrafica.get_larghezza_media() - 0.0) < 1e-6 {
                errors.push(format!("Larghezza media troppo bassa: {}", anagrafica.get_larghezza_media()));
            }

            match anagrafica.comunita.tipo {
                TipoComunitaNISECI::Recuperata => {
                    if let Some(fonte) = anagrafica.comunita.fonte {
                        if fonte.is_empty() {
                            errors.push("Fonte troppo corta".to_string());
                        }
                    } else {
                        errors.push("Fonte mancante".to_string());
                    }
                }
                TipoComunitaNISECI::AffinataDalMase => {
                    if let Some(num_proto) = anagrafica.comunita.numero_protocollo {
                        if num_proto.is_empty() {
                            errors.push("Numero protocollo troppo corto".to_string());
                        }
                    } else {
                        errors.push("Numero protocollo mancante".to_string());
                    }
                }
                _ => {}
            }

            if anagrafica.bacino_appartenenza.is_empty() {
                errors.push("Nome bacino di appartenenza troppo corto".to_string());
            }

            for e in &errors {
                self.add_console_message(format!("InfoAggiuntiveController:  {e}"));
            }

            let mut state = GLOBAL_STATE.lock().unwrap();

            if errors.is_empty() {
                state.infoaggiuntive_model.set_valid(true);
            } else {
                //TODO: handle validation errors
                //Will probably switch to ConsoleView using an errors_occurred flag like ValidazioneFileInput
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
            }
        } else {
            let err_msg = "InfoAggiuntiveController: valida_anagrafica_niseci() ha ricevuto uno stato spurio.";
            eprintln!("{}", err_msg);
            self.add_console_message(format!("InfoAggiuntiveController:  {err_msg}"));
        };
    }

    pub fn backout_anagrafica_niseci(&self) {
        self.unset_console_env("anagrafica_niseci".to_string());
        self.add_console_message("InfoAggiuntiveController: L'utente ha annullato l'inserimento info aggiuntive.".to_string());
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_niseci().is_some());
        state.data_model.set_anagrafica_niseci(None);
        state.infoaggiuntive_model.set_done_editing(false);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub fn set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }

    pub fn unset_console_env(&self, key: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.console_model.console.remove_env(key);
    }
}

pub struct OutputController;

impl Controller for OutputController {
    type SubModel = OutputModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.output_model.increment_frame_counter();
        if state.data_model.get_errors_occurred() {
            eprintln!("OutputController:  Errors occurred");
            eprintln!("OutputController:  Let's update current view and go to CONSOLE.");
            main_state.set_current_view(CurrentView::Console);
            eprintln!("OutputController:  Clearing error state");
            state.data_model.set_errors_occurred(false);
        }
        match main_state.current_view {
            CurrentView::ProduzioneOutput => {
                if state.output_model.is_done_user_confirm() {
                    eprintln!("OutputController:  User confirmed");
                    eprintln!("OutputController:  Let's update current view and go to ProduzionePDF.");
                    main_state.set_current_view(CurrentView::ProduzionePDF);
                }
            }
            CurrentView::ProduzionePDF => {

            }
            _ => {}
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.output_model.clone()
    }
}

impl OutputController {
    pub fn new() -> Self {
        Self
    }

    pub fn get_is_done_calc(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.output_model.is_done_calc()
    }

    pub fn get_niseci_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => {
                    r.get_valore()
                }
                None => {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_rqe_niseci_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => {
                    r.get_rqe()
                }
                None => {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_stato_eco_niseci_value(&self) -> Option<StatoEcologicoNISECI> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => {
                    let state = GLOBAL_STATE.lock().unwrap();
                    let opt_anagrafica = state.data_model.get_anagrafica_niseci();
                    match opt_anagrafica {
                        Some(anagr) => {
                            let niseci_val = r.get_valore();
                            calculate_stato_ecologico(niseci_val, &anagr.area)
                        }
                        None => {
                            None
                        }
                    }
                }
                None => {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_x1_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            opt_res.map(|r| r.get_x1())
        } else {
            None
        }
    }

    pub fn get_x2_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => {
                    r.get_x2()
                }
                None => {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_x3_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            opt_res.map(|r| r.get_x3())
        } else {
            None
        }
    }


    fn set_data_risultato_niseci(&self, risultato: RisultatoNISECI) {
        self.set_console_env(("risultato_niseci".to_string(), format!("{risultato}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_risultato_niseci(Some(risultato));
        state.output_model.set_done_calc(true);
    }

    pub fn get_data_risultato_niseci(&self) -> Option<RisultatoNISECI> {
        if self.get_is_done_calc() {
            let state = GLOBAL_STATE.lock().unwrap();
            state.data_model.get_risultato_niseci()
        } else {
            None
        }
    }

    pub fn calc_niseci(&self) {
        let riferimento;
        let campionamento;
        let anagrafica;
        {
            let state = GLOBAL_STATE.lock().unwrap();
            riferimento = state.data_model.get_riferimento_niseci();
            campionamento = state.data_model.get_campionamento_niseci();
            anagrafica = state.data_model.get_anagrafica_niseci();
        }

        let mut valid = true;

        if riferimento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message("IMPLEMENTATION ERROR: riferimento niseci was None in calc_niseci()".to_string());
        }
        if campionamento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message("IMPLEMENTATION ERROR: campionamento niseci was None in calc_niseci()".to_string());
        }
        if anagrafica.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message("IMPLEMENTATION ERROR: anagrafica niseci was None in calc_niseci()".to_string());
        }
        if valid {
            let riferimento = riferimento.expect("calc_niseci() checked is_none() before");
            let campionamento = campionamento.expect("calc_niseci() checked is_none() before");
            let anagrafica = anagrafica.expect("calc_niseci() checked is_none() before");

            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, intermediates)) => {
                    match niseci {
                        Some(val) => {
                            self.add_console_message(format!("NISECI: {val}"));
                        }
                        None => {
                            self.add_console_message("NISECI: NC".to_string());
                        }
                    }

                    let rqe_niseci = calculate_rqe_niseci(niseci);

                    match rqe_niseci {
                        Some(val) => {
                            self.add_console_message(format!("RQE NISECI: {val}"));
                        }
                        None => {
                            self.add_console_message("RQE NISECI: NC".to_string());
                        }
                    }

                    let stato_ecologico = calculate_stato_ecologico(niseci, &anagrafica.area);

                    match stato_ecologico {
                        Some(val) => {
                            self.add_console_message(format!("Stato ecologico: {val}"));
                        }
                        None => {
                            self.add_console_message("Stato ecologico: NC".to_string());
                        }
                    }

                    intermediates.log();

                    self.add_console_message(format!("{intermediates}"));

                    let risultato_niseci = RisultatoNISECI::new(
                        niseci,
                        rqe_niseci,
                        intermediates
                    );

                    self.set_data_risultato_niseci(risultato_niseci);
                    println!("OutputController: Finished NISECI calc");
                },
                Err(niseci_errors) => {
                    for e in niseci_errors {
                        self.add_console_message(format!("Errore durante il calcolo NISECI: {}", e));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.data_model.set_errors_occurred(true);
                    state.output_model.set_done_calc(false);
                    state.data_model.set_risultato_niseci(None);
                }
            }
        } else {
            self.add_console_message("IMPLEMENTATION ERROR: spurious state in calc_niseci()".to_string());
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.data_model.set_errors_occurred(true);
        }
    }

    pub fn user_confirm_calc(&self) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.output_model.set_done_user_confirm(true);
    }

    pub fn set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }
}

pub struct _LogController;

impl _LogController {
    pub fn _new() -> Self {
        Self
    }

    pub fn _update(&self, _rl: &RaylibHandle) {
        //let mut state = GLOBAL_STATE.lock().unwrap();
        //state.second_model.set_name("Updated".to_string());
    }
}

pub struct ConsoleController;

impl Controller for ConsoleController {
    type SubModel = ConsoleModel;

    fn update(&self, rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        if state.console_model.should_backout() {
            state.console_model.set_should_backout(false);
            let prev = main_state.previous_view;
            main_state.set_current_view(prev);
            return;
        }

        // Handle input

        let mwheel_move = rl.get_mouse_wheel_move() as i32;

        if mwheel_move != 0 {
            if mwheel_move > 0 { // Positive is to scroll up
                state.console_model.console.scroll_up(mwheel_move as usize);
            } else {
                state.console_model.console.scroll_down(-mwheel_move as usize);
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
            state.console_model.console.handle_input(rl, Some(c), false, false);
        }

        // Check for Enter key press
        if rl.is_key_pressed(KEY_ENTER) {
            state.console_model.console.handle_input(rl, None, true, false);
        }

        // Check for Backspace key press
        if rl.is_key_pressed(KEY_BACKSPACE) {
            state.console_model.console.handle_input(rl, None, false, true);
        }
        state.console_model.set_name("Updated".to_string());
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.console_model.clone()
    }
}

impl ConsoleController {

    pub fn new() -> Self {
        Self
    }

    pub fn backout(&self) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.console_model.set_should_backout(true);
    }

    pub fn _set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }
}
