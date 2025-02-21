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
use crate::core::{MainState, TipoRecordCsv, check_campionamento_niseci_path, check_riferimento_niseci_path, check_records_riferimento_niseci, check_records_campionamento_niseci};
use crate::model::index::Indice;
use crate::model::niseci::{RiferimentoNISECI, CampionamentoNISECI, AnagraficaNISECI, TipoComunitaNISECI};
use crate::state::GLOBAL_STATE;
use crate::CurrentView;
use crate::process_csv_errors;
use raylib::RaylibHandle;
use std::path::PathBuf;
use raylib::consts::KeyboardKey::*;

// Controller to update and access the state
pub struct HomeController;

impl HomeController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> HomeModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.home_model.clone();
    }

    pub fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name(name);
    }

    pub fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_value(val);
    }
}

pub struct SecondController;

impl SecondController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> SecondModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.second_model.clone();
    }

    pub fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name(name);
    }

    pub fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_value(val);
    }
}

pub struct IndiceController;

impl IndiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.increment_frame_counter();

        match state.indice_model.get_selected_index() {
            Some(index) => {
                eprintln!("IndiceController:  L'utente ha selezionato indice {index}");
                eprintln!("IndiceController:  Let's update current view and go to SelezioneFileInput.");
                main_state.set_current_view(CurrentView::SelezioneFileInput)
            },
            None => ()
        }
    }

    pub fn set_indice_corrente(&self, index: Indice) -> () {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.set_selected_index(index);
    }
    pub fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.add_message(msg);
    }
}

pub struct FileInputController;

impl FileInputController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.increment_frame_counter();

        if state.fileinput_model.get_errors_occurred() {
            eprintln!("FileInputController:  Errors occurred");
            eprintln!("FileInputController:  Let's update current view and go to CONSOLE.");
            main_state.set_current_view(CurrentView::CONSOLE);
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
                    Indice::NISECI => {
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
                    Indice::HFBI => {
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
                    Indice::NISECI => {
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
                    Indice::HFBI => {
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
                            self.add_console_message(format!("FileInputController:  HFBI - L'utente ha validato campionamento"));
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                }
            },
            _ => {},
        }
    }

    pub fn get_state(&self) -> FileInputModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.clone();
    }

    pub fn get_current_index(&self) -> Option<Indice> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.indice_model.get_selected_index();
    }

    pub fn get_riferimento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_riferimento_path();
    }

    pub fn set_riferimento_path(&self, riferimento_path: Option<PathBuf>) {
        if let Some(ref rif_path) = riferimento_path {
            self.add_console_message(format!("FileInputController:  Selezione percorso riferimento: {{{}}}", rif_path.display()));
        } else {
            self.add_console_message(format!("FileInputController:  Deselezione percorso riferimento"));
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_riferimento_path(riferimento_path);
        state.fileinput_model.set_riferimento_path_valid(false); // Refresh the validity
    }

    pub fn get_riferimento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_riferimento_path_valid();
    }

    fn set_data_riferimento_niseci(&self, riferimento: RiferimentoNISECI) {
        self.set_console_env(("riferimento_niseci".to_string(), format!("{riferimento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_riferimento_niseci(Some(riferimento));
        state.fileinput_model.set_riferimento_path_valid(true);
    }

    pub fn get_data_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.data_model.get_riferimento_niseci();
    }

    pub fn valida_riferimento_niseci_path(&self) {
        if let Some(path) = self.get_riferimento_path() {
            let csv_check = check_riferimento_niseci_path(path);

            match csv_check {
                Ok(records) => {
                    let records_check = check_records_riferimento_niseci(records);

                    match records_check {
                        Ok(species) => {
                            self.add_console_message(format!("FileInputController:  Validazione RiferimentoNISECI completata!"));
                            let riferimento = RiferimentoNISECI::new(species);
                            self.set_data_riferimento_niseci(riferimento);
                        }
                        Err(errors) => { // Value errors
                            for e in errors {
                                self.add_console_message(format!("FileInputController:  {e}"));
                            }
                            let mut state = GLOBAL_STATE.lock().unwrap();
                            state.fileinput_model.set_errors_occurred(true);
                            return;
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
                    return;
                }
            }
        }
    }

    pub fn get_campionamento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_campionamento_path();
    }

    pub fn set_campionamento_path(&self, campionamento_path: Option<PathBuf>) {
        if let Some(ref camp_path) = campionamento_path {
            self.add_console_message(format!("FileInputController:  Selezione percorso campionamento: {{{}}}", camp_path.display()));
        } else {
            self.add_console_message(format!("FileInputController:  Deselezione percorso campionamento"));
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_campionamento_path(campionamento_path);
        state.fileinput_model.set_campionamento_path_valid(false); // Refresh the validity
    }

    pub fn get_campionamento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_campionamento_path_valid();
    }

    fn set_data_campionamento_niseci(&self, campionamento: CampionamentoNISECI) {
        self.set_console_env(("campionamento_niseci".to_string(), format!("{campionamento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_campionamento_niseci(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub fn get_data_campionamento_niseci(&self) -> Option<CampionamentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.data_model.get_campionamento_niseci();
    }

    pub fn valida_campionamento_niseci_path(&self) {
        if let Some(path) = self.get_campionamento_path() {
            let csv_check = check_campionamento_niseci_path(path);

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
                                self.add_console_message(format!("FileInputController:  Validazione CampionamentoNISECI completata!"));
                                let campionamento = CampionamentoNISECI::new(campioni);
                                self.set_data_campionamento_niseci(campionamento);
                            }
                            Err(errors) => { // Value errors
                                for e in errors {
                                    self.add_console_message(format!("FileInputController:  {e}"));
                                }
                                let mut state = GLOBAL_STATE.lock().unwrap();
                                state.fileinput_model.set_errors_occurred(true);
                                return;
                            }
                        }
                    } else {
                        let error_msg = "Impossibile validare campionamento_niseci senza avere riferimento";
                        eprintln!("{error_msg}");
                        self.add_console_message(format!("FileInputController:  {error_msg}"));
                        let mut state = GLOBAL_STATE.lock().unwrap();
                        state.fileinput_model.set_errors_occurred(true);
                        return;
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
                    return;
                }
            }
        }
    }
    pub fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.add_message(msg);
    }
    pub fn set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }
}

pub struct InfoAggiuntiveController;

impl InfoAggiuntiveController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.increment_frame_counter();

        if state.infoaggiuntive_model.get_errors_occurred() {
            eprintln!("InfoAggiuntiveController:  Errors occurred");
            eprintln!("InfoAggiuntiveController:  Let's update current view and go to CONSOLE.");
            main_state.set_current_view(CurrentView::CONSOLE);
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
                    Indice::NISECI => {
                        if state.infoaggiuntive_model.is_done_editing() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go to ValidaInfoAggiuntive");
                            main_state.set_current_view(CurrentView::ValidazioneInfoAggiuntive);
                            return;
                        }
                    }
                    Indice::HFBI => {
                    }
                }
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                match current_indice {
                    Indice::NISECI => {
                        if !state.infoaggiuntive_model.is_done_editing() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go back to SelezionaInfoAggiuntive");
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                            return;
                        }
                        if state.infoaggiuntive_model.is_valid() {
                            eprintln!("InfoAggiuntiveController:  Let's update current view and go to ProduzioneOutput");
                            main_state.set_current_view(CurrentView::ProduzioneOutput);
                            return;
                        }
                    }
                    Indice::HFBI => {
                    }
                }
            }
            _ => {}
        }

    }

    pub fn get_state(&self) -> InfoAggiuntiveModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.infoaggiuntive_model.clone();
    }

    pub fn get_current_index(&self) -> Option<Indice> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.indice_model.get_selected_index();
    }

    pub fn get_data_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.data_model.get_anagrafica_niseci();
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

    pub fn valida_anagrafica_niseci(&self) {

        {
            let mut state = GLOBAL_STATE.lock().unwrap();
            assert!(state.infoaggiuntive_model.is_done_editing());
        }

        if let Some(anagrafica) = self.get_data_anagrafica_niseci() {
            let mut errors: Vec<String> = Vec::new();

            //TODO: check codice stazione after refactor

            if anagrafica.nome_fiume.len() < 1 {
                errors.push(format!("Nome fiume troppo corto"));
            }

            if anagrafica.posizione.regione.len() < 1 {
                errors.push(format!("Nome regione troppo corto"));
            }

            if anagrafica.posizione.provincia.len() < 1 {
                errors.push(format!("Nome provincia troppo corto"));
            }

            //TODO: check date format

            if (anagrafica.get_lunghezza_media() - 0.0) < 1e-6 {
                errors.push(format!("Lunghezza media troppo bassa: {}", anagrafica.get_lunghezza_media()));
            }

            if (anagrafica.get_larghezza_media() - 0.0) < 1e-6 {
                errors.push(format!("Larghezza media troppo bassa: {}", anagrafica.get_larghezza_media()));
            }

            match anagrafica.comunita.tipo {
                TipoComunitaNISECI::Recuperata => {
                    if let Some(fonte) = anagrafica.comunita.fonte {
                        if fonte.len() < 1 {
                            errors.push(format!("Fonte troppo corta"));
                        }
                    } else {
                        errors.push(format!("Fonte mancante"));
                    }
                }
                TipoComunitaNISECI::AffinataDalMase => {
                    if let Some(num_proto) = anagrafica.comunita.numero_protocollo {
                        if num_proto.len() < 1 {
                            errors.push(format!("Numero protocollo troppo corto"));
                        }
                    } else {
                        errors.push(format!("Numero protocollo mancante"));
                    }
                }
                _ => {}
            }

            if anagrafica.bacino_appartenenza.len() < 1 {
                errors.push(format!("Nome bacino di appartenenza troppo corto"));
            }

            for e in &errors {
                self.add_console_message(format!("InfoAggiuntiveController:  {e}"));
            }

            let mut state = GLOBAL_STATE.lock().unwrap();

            if errors.len() == 0 {
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

    pub fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.add_message(msg);
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

impl OutputController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, _main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.output_model.increment_frame_counter();
    }

    pub fn get_state(&self) -> OutputModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.output_model.clone();
    }
    pub fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.add_message(msg);
    }
}

pub struct LogController;

impl LogController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        //let mut state = GLOBAL_STATE.lock().unwrap();
        //state.second_model.set_name("Updated".to_string());
    }
}

pub struct ConsoleController;

impl ConsoleController {

    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, rl : &mut RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
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

    pub fn get_state(&self) -> ConsoleModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.console_model.clone();
    }

    pub fn add_console_message(&self, msg: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.add_message(msg);
    }

    pub fn set_console_env(&self, (key, val): (String,String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key,val));
    }
}
