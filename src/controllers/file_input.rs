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
pub(crate) struct FileInputController;
use crate::app::core::{Action, Localize, MainState};
use crate::app::model::{Model, SubModel};
use crate::controllers::{Controller, CurrentView, FileInputModel};
use esox::csv::deser::process_csv_errors;
use esox::csv::load::hfbi::{load_campionamento_hfbi_from_path, CampionamentoHFBIError};
use esox::csv::load::niseci::{
    load_campionamento_niseci_from_path, load_riferimento_niseci_from_path,
    CampionamentoNISECIError, RiferimentoNISECIError,
};
use esox::csv::load::InputFormat;
use esox::deser::TipoRecord;
use esox::domain::hfbi::CampionamentoHFBI;
use esox::domain::index::Indice;
use esox::domain::niseci::{CampionamentoNISECI, RiferimentoNISECI};
use raylib::RaylibHandle;
use std::path::PathBuf;

impl Controller for FileInputController {
    type SubModel = FileInputModel;

    fn update(
        &self,
        _rl: &mut RaylibHandle,
        state: &mut Model,
        actions: &mut Vec<Action>,
        main_state: &mut MainState,
    ) {
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

        for a in actions.drain(..) {
            match a {
                Action::PickRiferimentoPath(path) => {
                    self.set_riferimento_path(state, path);
                }
                Action::PickCampionamentoPath(path) => {
                    self.set_campionamento_path(state, path);
                }
                Action::ValidaRiferimentoPath(has_headers) => match main_state.locale {
                    Localize::Italian => {
                        self.valida_riferimento_niseci_path(
                            state,
                            has_headers,
                            InputFormat::Alternative,
                        );
                    }
                    Localize::International => {
                        self.valida_riferimento_niseci_path(
                            state,
                            has_headers,
                            InputFormat::Standard,
                        );
                    }
                },
                Action::ValidaCampionamentoPath(has_headers) => {
                    if let Some(idx) = state.indice_model.get_selected_index() {
                        match idx {
                            Indice::Niseci => match main_state.locale {
                                Localize::Italian => {
                                    self.valida_campionamento_niseci_path(
                                        state,
                                        has_headers,
                                        InputFormat::Alternative,
                                    );
                                }
                                Localize::International => {
                                    self.valida_campionamento_niseci_path(
                                        state,
                                        has_headers,
                                        InputFormat::Standard,
                                    );
                                }
                            },
                            Indice::Hfbi => match main_state.locale {
                                Localize::Italian => {
                                    self.valida_campionamento_hfbi_path(
                                        state,
                                        has_headers,
                                        InputFormat::Alternative,
                                    );
                                }
                                Localize::International => {
                                    self.valida_campionamento_hfbi_path(
                                        state,
                                        has_headers,
                                        InputFormat::Standard,
                                    );
                                }
                            },
                        }
                    } else {
                        eprintln!(
                            "FileInputController:  Can't handle action {} without a selected index",
                            a
                        );
                    }
                }
                _ => {
                    println!("FileInputController:  Got action {}", a);
                }
            }
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
}

impl FileInputController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_riferimento_path(&self, state: &Model) -> Option<PathBuf> {
        state.fileinput_model.get_riferimento_path()
    }

    pub(crate) fn set_riferimento_path(
        &self,
        state: &mut Model,
        riferimento_path: Option<PathBuf>,
    ) {
        if let Some(ref rif_path) = riferimento_path {
            self.add_console_message(
                state,
                format!(
                    "FileInputController:  Selezione percorso riferimento: {{{}}}",
                    rif_path.display()
                ),
            );
        } else {
            self.add_console_message(
                state,
                "FileInputController:  Deselezione percorso riferimento".to_string(),
            );
        }
        state.fileinput_model.set_riferimento_path(riferimento_path);
        state.fileinput_model.set_riferimento_path_valid(false); // Refresh the validity
    }

    fn set_data_riferimento_niseci(&self, state: &mut Model, riferimento: RiferimentoNISECI) {
        self.set_console_env(
            state,
            ("riferimento_niseci".to_string(), format!("{riferimento}")),
        );
        state.data_model.set_riferimento_niseci(Some(riferimento));
        state.fileinput_model.set_riferimento_path_valid(true);
    }

    pub(crate) fn get_data_riferimento_niseci(&self, state: &Model) -> Option<RiferimentoNISECI> {
        state.data_model.get_riferimento_niseci()
    }

    pub(crate) fn valida_riferimento_niseci_path(
        &self,
        state: &mut Model,
        has_headers: bool,
        format: InputFormat,
    ) {
        if let Some(path) = self.get_riferimento_path(state) {
            let load = load_riferimento_niseci_from_path(path, has_headers, format);

            match load {
                Ok(species) => {
                    self.add_console_message(
                        state,
                        "FileInputController:  Validazione RiferimentoNISECI completata!"
                            .to_string(),
                    );
                    self.set_data_riferimento_niseci(state, species);
                }
                Err(ev) => {
                    match ev {
                        RiferimentoNISECIError::Csv(errors) => {
                            // Csv errors
                            /*
                            for err in errors {
                                eprintln!("FileInputController:  {err}");
                            }
                            */
                            let processed_errors =
                                process_csv_errors(&errors, TipoRecord::RiferimentoNISECI);
                            for e in processed_errors {
                                self.add_console_message(
                                    state,
                                    format!("FileInputController:  {e}"),
                                );
                            }
                            state.fileinput_model.set_errors_occurred(true);
                        }
                        RiferimentoNISECIError::Value(errors) => {
                            // Value errors
                            for e in errors {
                                self.add_console_message(
                                    state,
                                    format!("FileInputController:  {e}"),
                                );
                            }
                            state.fileinput_model.set_errors_occurred(true);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn get_campionamento_path(&self, state: &Model) -> Option<PathBuf> {
        state.fileinput_model.get_campionamento_path()
    }

    pub(crate) fn set_campionamento_path(
        &self,
        state: &mut Model,
        campionamento_path: Option<PathBuf>,
    ) {
        if let Some(ref camp_path) = campionamento_path {
            self.add_console_message(
                state,
                format!(
                    "FileInputController:  Selezione percorso campionamento: {{{}}}",
                    camp_path.display()
                ),
            );
        } else {
            self.add_console_message(
                state,
                "FileInputController:  Deselezione percorso campionamento".to_string(),
            );
        }
        state
            .fileinput_model
            .set_campionamento_path(campionamento_path);
        state.fileinput_model.set_campionamento_path_valid(false); // Refresh the validity
    }

    pub(crate) fn _get_campionamento_path_valid(&self, state: &Model) -> bool {
        state.fileinput_model.get_campionamento_path_valid()
    }

    fn set_data_campionamento_niseci(&self, state: &mut Model, campionamento: CampionamentoNISECI) {
        self.set_console_env(
            state,
            (
                "campionamento_niseci".to_string(),
                format!("{campionamento}"),
            ),
        );
        state
            .data_model
            .set_campionamento_niseci(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub(crate) fn _get_data_campionamento_niseci(
        &self,
        state: &Model,
    ) -> Option<CampionamentoNISECI> {
        state.data_model.get_campionamento_niseci()
    }

    pub(crate) fn valida_campionamento_niseci_path(
        &self,
        state: &mut Model,
        has_headers: bool,
        format: InputFormat,
    ) {
        if let Some(path) = self.get_campionamento_path(state) {
            let opt_riferimento_niseci = self.get_data_riferimento_niseci(state);
            if let Some(riferimento_niseci) = opt_riferimento_niseci {
                let load = load_campionamento_niseci_from_path(
                    path,
                    has_headers,
                    &riferimento_niseci,
                    format,
                );
                match load {
                    Ok(campioni) => {
                        self.add_console_message(
                            state,
                            "FileInputController:  Validazione CampionamentoNISECI completata!"
                                .to_string(),
                        );
                        self.set_data_campionamento_niseci(state, campioni);
                    }
                    Err(ev) => {
                        match ev {
                            CampionamentoNISECIError::Csv(errors) => {
                                // Csv errors
                                /*
                                for err in errors {
                                    eprintln!("FileInputController:  {err}");
                                }
                                */
                                let processed_errors =
                                    process_csv_errors(&errors, TipoRecord::CampionamentoNISECI);
                                for e in processed_errors {
                                    self.add_console_message(
                                        state,
                                        format!("FileInputController:  {e}"),
                                    );
                                }
                            }
                            CampionamentoNISECIError::Value(errors) => {
                                // Value errors
                                for e in errors {
                                    self.add_console_message(
                                        state,
                                        format!("FileInputController:  {e}"),
                                    );
                                }
                            }
                        }
                        state.fileinput_model.set_errors_occurred(true);
                    }
                }
            } else {
                let error_msg = "Impossibile validare campionamento_niseci senza avere riferimento";
                eprintln!("{error_msg}");
                self.add_console_message(state, format!("FileInputController:  {error_msg}"));
                state.fileinput_model.set_errors_occurred(true);
            }
        }
    }
    pub(crate) fn valida_campionamento_hfbi_path(
        &self,
        state: &mut Model,
        has_headers: bool,
        format: InputFormat,
    ) {
        if let Some(path) = self.get_campionamento_path(state) {
            let load = load_campionamento_hfbi_from_path(path, has_headers, format);

            match load {
                Ok(campioni) => {
                    self.add_console_message(
                        state,
                        "FileInputController:  Validazione CampionamentoHFBI completata!"
                            .to_string(),
                    );
                    self.set_data_campionamento_hfbi(state, campioni);
                }
                Err(ev) => {
                    match ev {
                        CampionamentoHFBIError::Csv(errors) => {
                            // Csv errors
                            /*
                            for err in errors {
                                eprintln!("FileInputController:  {err}");
                            }
                            */
                            let processed_errors =
                                process_csv_errors(&errors, TipoRecord::CampionamentoNISECI);
                            for e in processed_errors {
                                self.add_console_message(
                                    state,
                                    format!("FileInputController:  {e}"),
                                );
                            }
                        }
                        CampionamentoHFBIError::Value(errors) => {
                            // Value errors
                            for e in errors {
                                self.add_console_message(
                                    state,
                                    format!("FileInputController:  {e}"),
                                );
                            }
                        }
                    }
                    state.fileinput_model.set_errors_occurred(true);
                }
            }
        }
    }

    fn set_data_campionamento_hfbi(&self, state: &mut Model, campionamento: CampionamentoHFBI) {
        self.set_console_env(
            state,
            ("campionamento_hfbi".to_string(), format!("{campionamento}")),
        );
        state.data_model.set_campionamento_hfbi(Some(campionamento));
        state.fileinput_model.set_campionamento_path_valid(true);
    }

    pub(crate) fn set_console_env(&self, state: &mut Model, (key, val): (String, String)) {
        state.console_model.console.set_env((key, val));
    }
}
