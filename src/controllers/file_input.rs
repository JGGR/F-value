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
pub(crate) struct FileInputController;
use crate::app::model::SubModel;
use crate::controllers::{Controller, CurrentView, FileInputModel};
use crate::core::csv::deser::{
    hfbi::{check_campionamento_hfbi_path, VeryItalianRecordCsvCampionamentoHFBI},
    niseci::{
        check_campionamento_niseci_path, check_riferimento_niseci_path,
        VeryItalianRecordCsvCampionamentoNISECI, VeryItalianRecordCsvRiferimentoNISECI,
        PlainRecordCsvRiferimentoNISECI,
    },
    process_csv_errors,
};
use crate::core::csv::parser::{
    hfbi::check_records_campionamento_hfbi,
    niseci::{check_records_campionamento_niseci, check_records_riferimento_niseci},
};
use crate::core::csv::{TipoRecordCsv, RecordCsvRiferimentoNISECI, RecordCsvCampionamentoNISECI, RecordCsvCampionamentoHFBI};
use crate::domain::hfbi::CampionamentoHFBI;
use crate::domain::index::Indice;
use crate::domain::niseci::{CampionamentoNISECI, RiferimentoNISECI};
use crate::state::GLOBAL_STATE;
use crate::app::core::{MainState, Localize};
use raylib::RaylibHandle;
use std::path::PathBuf;

impl Controller for FileInputController {
    type SubModel = FileInputModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.increment_frame_counter();

        if main_state.should_reset {
            eprintln!("FileInputController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.fileinput_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }

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
            eprintln!(
                "FileInputController:  Let's update current view and go back to SelezioneIndice."
            );
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
                        if let Some(_campionamento_path) =
                            state.fileinput_model.get_campionamento_path()
                        {
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
                        if let Some(_campionamento_path) =
                            state.fileinput_model.get_campionamento_path()
                        {
                            // Assumes the path is ready to be used.
                            // The current selection used by the only forces .csv extension
                            campionamento_ready = true;
                        }
                        if campionamento_ready {
                            eprintln!(
                                "FileInputController:  HFBI - L'utente ha fornito campionamento"
                            );
                            eprintln!("FileInputController:  Let's update current view and go to ValidazioneFileInput.");
                            main_state.set_current_view(CurrentView::ValidazioneFileInput);
                        }
                    }
                }
            }
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

                        if let Some(_campionamento_path) =
                            state.fileinput_model.get_campionamento_path()
                        {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento niseci path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }

                        let riferimento_valid = state.fileinput_model.get_riferimento_path_valid();
                        let campionamento_valid =
                            state.fileinput_model.get_campionamento_path_valid();

                        if riferimento_valid && campionamento_valid {
                            eprintln!("FileInputController:  NISECI - L'utente ha validato riferimento e campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            //self.add_console_message(format!("FileInputController:  NISECI - L'utente ha validato riferimento e campionamento"));
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                    Indice::Hfbi => {
                        if let Some(_campionamento_path) =
                            state.fileinput_model.get_campionamento_path()
                        {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento hfbi path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }
                        let campionamento_valid =
                            state.fileinput_model.get_campionamento_path_valid();

                        if campionamento_valid {
                            eprintln!(
                                "FileInputController:  HFBI - L'utente ha validato campionamento"
                            );
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            //self.add_console_message("FileInputController:  HFBI - L'utente ha validato campionamento".to_string());
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.clone()
    }
}

impl FileInputController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_riferimento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_riferimento_path()
    }

    pub(crate) fn set_riferimento_path(&self, riferimento_path: Option<PathBuf>) {
        if let Some(ref rif_path) = riferimento_path {
            self.add_console_message(format!(
                "FileInputController:  Selezione percorso riferimento: {{{}}}",
                rif_path.display()
            ));
        } else {
            self.add_console_message(
                "FileInputController:  Deselezione percorso riferimento".to_string(),
            );
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_riferimento_path(riferimento_path);
        state.fileinput_model.set_riferimento_path_valid(false); // Refresh the validity
    }

    pub(crate) fn get_riferimento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_riferimento_path_valid()
    }

    fn set_data_riferimento_niseci(&self, riferimento: RiferimentoNISECI) {
        self.set_console_env(("riferimento_niseci".to_string(), format!("{riferimento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_riferimento_niseci(Some(riferimento));
        state.fileinput_model.set_riferimento_path_valid(true);
    }

    pub(crate) fn get_data_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_riferimento_niseci()
    }

    pub(crate) fn valida_riferimento_niseci_path<T: RecordCsvRiferimentoNISECI + 'static>(&self) {
        if let Some(path) = self.get_riferimento_path() {
            let csv_check = check_riferimento_niseci_path::<T>(path);

            match csv_check {
                Ok(records) => {
                    let records_check = check_records_riferimento_niseci(records);

                    match records_check {
                        Ok(species) => {
                            self.add_console_message(
                                "FileInputController:  Validazione RiferimentoNISECI completata!"
                                    .to_string(),
                            );
                            let riferimento = RiferimentoNISECI::new(species);
                            self.set_data_riferimento_niseci(riferimento);
                        }
                        Err(errors) => {
                            // Value errors
                            for e in errors {
                                self.add_console_message(format!("FileInputController:  {e}"));
                            }
                            let mut state = GLOBAL_STATE.lock().unwrap();
                            state.fileinput_model.set_errors_occurred(true);
                        }
                    }
                }
                Err(errors) => {
                    // Csv errors
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    let processed_errors =
                        process_csv_errors(&errors, TipoRecordCsv::RiferimentoNISECI);
                    for e in processed_errors {
                        self.add_console_message(format!("FileInputController:  {e}"));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }

    pub(crate) fn get_campionamento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_campionamento_path()
    }

    pub(crate) fn set_campionamento_path(&self, campionamento_path: Option<PathBuf>) {
        if let Some(ref camp_path) = campionamento_path {
            self.add_console_message(format!(
                "FileInputController:  Selezione percorso campionamento: {{{}}}",
                camp_path.display()
            ));
        } else {
            self.add_console_message(
                "FileInputController:  Deselezione percorso campionamento".to_string(),
            );
        }
        let mut state = GLOBAL_STATE.lock().unwrap();
        state
            .fileinput_model
            .set_campionamento_path(campionamento_path);
        state.fileinput_model.set_campionamento_path_valid(false); // Refresh the validity
    }

    pub(crate) fn _get_campionamento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.get_campionamento_path_valid()
    }

    fn set_data_campionamento_niseci(&self, campionamento: CampionamentoNISECI) {
        self.set_console_env((
            "campionamento_niseci".to_string(),
            format!("{campionamento}"),
        ));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state
            .data_model
            .set_campionamento_niseci(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub(crate) fn _get_data_campionamento_niseci(&self) -> Option<CampionamentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_campionamento_niseci()
    }

    pub(crate) fn valida_campionamento_niseci_path<T: RecordCsvCampionamentoNISECI + 'static>(&self) {
        if let Some(path) = self.get_campionamento_path() {
            let csv_check =
                check_campionamento_niseci_path::<T>(path);

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
                        let records_check = check_records_campionamento_niseci(
                            records,
                            riferimento_niseci.elenco_specie,
                        );
                        match records_check {
                            Ok(campioni) => {
                                self.add_console_message("FileInputController:  Validazione CampionamentoNISECI completata!".to_string());
                                let campionamento = CampionamentoNISECI::new(campioni);
                                self.set_data_campionamento_niseci(campionamento);
                            }
                            Err(errors) => {
                                // Value errors
                                for e in errors {
                                    self.add_console_message(format!("FileInputController:  {e}"));
                                }
                                let mut state = GLOBAL_STATE.lock().unwrap();
                                state.fileinput_model.set_errors_occurred(true);
                            }
                        }
                    } else {
                        let error_msg =
                            "Impossibile validare campionamento_niseci senza avere riferimento";
                        eprintln!("{error_msg}");
                        self.add_console_message(format!("FileInputController:  {error_msg}"));
                        let mut state = GLOBAL_STATE.lock().unwrap();
                        state.fileinput_model.set_errors_occurred(true);
                    }
                }
                Err(errors) => {
                    // Csv errors
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    let processed_errors =
                        process_csv_errors(&errors, TipoRecordCsv::CampionamentoNISECI);
                    for e in processed_errors {
                        self.add_console_message(format!("FileInputController:  {e}"));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }
    pub(crate) fn valida_campionamento_hfbi_path<T: RecordCsvCampionamentoHFBI + 'static>(&self) {
        if let Some(path) = self.get_campionamento_path() {
            let csv_check =
                check_campionamento_hfbi_path::<T>(path);

            match csv_check {
                Ok(records) => {
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
                    let records_check = check_records_campionamento_hfbi(records);
                    match records_check {
                        Ok(mut campioni) => {
                            self.add_console_message(
                                "FileInputController:  Validazione CampionamentoHFBI completata!"
                                    .to_string(),
                            );
                            campioni.sort_by(|a, b| {
                                b.peso
                                    .partial_cmp(&a.peso)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            });
                            let campionamento = CampionamentoHFBI::new(campioni);
                            self.set_data_campionamento_hfbi(campionamento);
                        }
                        Err(errors) => {
                            // Value errors
                            for e in errors {
                                self.add_console_message(format!("FileInputController:  {e}"));
                            }
                            let mut state = GLOBAL_STATE.lock().unwrap();
                            state.fileinput_model.set_errors_occurred(true);
                        }
                    }
                }
                Err(errors) => {
                    // Csv errors
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    let processed_errors =
                        process_csv_errors(&errors, TipoRecordCsv::CampionamentoNISECI);
                    for e in processed_errors {
                        self.add_console_message(format!("FileInputController:  {e}"));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }

    fn set_data_campionamento_hfbi(&self, campionamento: CampionamentoHFBI) {
        self.set_console_env(("campionamento_hfbi".to_string(), format!("{campionamento}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_campionamento_hfbi(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub(crate) fn set_console_env(&self, (key, val): (String, String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key, val));
    }
}
