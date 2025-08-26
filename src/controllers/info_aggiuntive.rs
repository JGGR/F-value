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
pub(crate) struct InfoAggiuntiveController;
use crate::app::model::SubModel;
use crate::controllers::{Controller, CurrentView, InfoAggiuntiveModel};
use crate::core::csv::parser::parse_date;
use crate::domain::hfbi::AnagraficaHFBI;
use crate::domain::index::Indice;
use crate::domain::niseci::{AnagraficaNISECI, TipoComunitaNISECI};
use crate::state::GLOBAL_STATE;
use crate::MainState;
use chrono::format::ParseErrorKind;
use raylib::RaylibHandle;

impl Controller for InfoAggiuntiveController {
    type SubModel = InfoAggiuntiveModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.increment_frame_counter();

        if main_state.should_reset {
            eprintln!("InfoAggiuntiveController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.fileinput_model.reset();
            state.infoaggiuntive_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }

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
            CurrentView::SelezioneInfoAggiuntive => match current_indice {
                Indice::Niseci | Indice::Hfbi => {
                    if state.infoaggiuntive_model.is_done_editing() {
                        eprintln!("InfoAggiuntiveController:  Let's update current view and go to ValidaInfoAggiuntive");
                        main_state.set_current_view(CurrentView::ValidazioneInfoAggiuntive);
                    }
                }
            },
            CurrentView::ValidazioneInfoAggiuntive => match current_indice {
                Indice::Niseci | Indice::Hfbi => {
                    if !state.infoaggiuntive_model.is_done_editing() {
                        eprintln!("InfoAggiuntiveController:  Let's update current view and go back to SelezionaInfoAggiuntive");
                        main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                    }
                    if state.infoaggiuntive_model.is_valid() {
                        eprintln!("InfoAggiuntiveController:  Let's update current view and go to ProduzioneOutput");
                        main_state.set_current_view(CurrentView::ProduzioneOutput);
                    }
                }
            },
            _ => {}
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.clone()
    }
}

impl InfoAggiuntiveController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_data_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_anagrafica_niseci()
    }

    pub(crate) fn submit_anagrafica_niseci(&self, anagrafica: AnagraficaNISECI) {
        self.set_console_env(("anagrafica_niseci".to_string(), format!("{anagrafica}")));
        self.add_console_message(
            "InfoAggiuntiveController: L'utente ha completato l'inserimento info aggiuntive."
                .to_string(),
        );
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_niseci().is_none());
        state.data_model.set_anagrafica_niseci(Some(anagrafica));
        state.infoaggiuntive_model.set_done_editing(true);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn check_larghezza_stazione_string(&self, larghezza: &str) -> Result<f32, String> {
        let s = larghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione larghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg
                        .replace("invalid float literal", "tipo non valido: atteso decimale");
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

    pub(crate) fn check_lunghezza_stazione_string(&self, lunghezza: &str) -> Result<f32, String> {
        let s = lunghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione lunghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg
                        .replace("invalid float literal", "tipo non valido: atteso decimale");
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

    pub(crate) fn valida_anagrafica_niseci(&self) {
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
                Ok(_) => {}
                Err(e) => match e.kind() {
                    ParseErrorKind::OutOfRange => {
                        errors.push("Data fornita non valida: fuori range".to_string());
                    }
                    ParseErrorKind::Impossible => {
                        errors.push("Data fornita non valida: valori non possibili".to_string());
                    }
                    ParseErrorKind::NotEnough => {
                        errors.push("Data fornita non valida: specifica insufficiente".to_string());
                    }
                    ParseErrorKind::Invalid => {
                        errors.push(
                            "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                        );
                    }
                    ParseErrorKind::TooShort => {
                        errors.push(
                            "Data fornita non valida: terminazione prematura dell'input"
                                .to_string(),
                        );
                    }
                    ParseErrorKind::TooLong => {
                        errors.push("Data fornita non valida: input in eccesso".to_string());
                    }
                    ParseErrorKind::BadFormat => {
                        errors.push(
                            "Data fornita non valida: errore nella specifica di formattazione"
                                .to_string(),
                        );
                    }
                    _ => {
                        errors.push("Data fornita non valida: errore sconosciuto".to_string());
                    }
                },
            }

            if (anagrafica.get_lunghezza_media() - 0.0) < 1e-6 {
                errors.push(format!(
                    "Lunghezza media troppo bassa: {}",
                    anagrafica.get_lunghezza_media()
                ));
            }

            if (anagrafica.get_larghezza_media() - 0.0) < 1e-6 {
                errors.push(format!(
                    "Larghezza media troppo bassa: {}",
                    anagrafica.get_larghezza_media()
                ));
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

    pub(crate) fn backout_anagrafica_niseci(&self) {
        self.unset_console_env("anagrafica_niseci".to_string());
        self.add_console_message(
            "InfoAggiuntiveController: L'utente ha annullato l'inserimento info aggiuntive."
                .to_string(),
        );
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_niseci().is_some());
        state.data_model.set_anagrafica_niseci(None);
        state.infoaggiuntive_model.set_done_editing(false);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn get_data_anagrafica_hfbi(&self) -> Option<AnagraficaHFBI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_anagrafica_hfbi()
    }

    pub(crate) fn submit_anagrafica_hfbi(&self, anagrafica: AnagraficaHFBI) {
        self.set_console_env(("anagrafica_hfbi".to_string(), format!("{anagrafica}")));
        self.add_console_message(
            "InfoAggiuntiveController: L'utente ha completato l'inserimento info aggiuntive."
                .to_string(),
        );
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_hfbi().is_none());
        state.data_model.set_anagrafica_hfbi(Some(anagrafica));
        state.infoaggiuntive_model.set_done_editing(true);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn valida_anagrafica_hfbi(&self) {
        //We grab the state in a scope to ensure we don't get lock problems
        {
            let state = GLOBAL_STATE.lock().unwrap();
            assert!(state.infoaggiuntive_model.is_done_editing());
        }

        if let Some(anagrafica) = self.get_data_anagrafica_hfbi() {
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
                Ok(_) => {}
                Err(e) => match e.kind() {
                    ParseErrorKind::OutOfRange => {
                        errors.push("Data fornita non valida: fuori range".to_string());
                    }
                    ParseErrorKind::Impossible => {
                        errors.push("Data fornita non valida: valori non possibili".to_string());
                    }
                    ParseErrorKind::NotEnough => {
                        errors.push("Data fornita non valida: specifica insufficiente".to_string());
                    }
                    ParseErrorKind::Invalid => {
                        errors.push(
                            "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                        );
                    }
                    ParseErrorKind::TooShort => {
                        errors.push(
                            "Data fornita non valida: terminazione prematura dell'input"
                                .to_string(),
                        );
                    }
                    ParseErrorKind::TooLong => {
                        errors.push("Data fornita non valida: input in eccesso".to_string());
                    }
                    ParseErrorKind::BadFormat => {
                        errors.push(
                            "Data fornita non valida: errore nella specifica di formattazione"
                                .to_string(),
                        );
                    }
                    _ => {
                        errors.push("Data fornita non valida: errore sconosciuto".to_string());
                    }
                },
            }

            if (anagrafica.get_lunghezza_media() - 0.0) < 1e-6 {
                errors.push(format!(
                    "Lunghezza media troppo bassa: {}",
                    anagrafica.get_lunghezza_media()
                ));
            }

            if (anagrafica.get_larghezza_media() - 0.0) < 1e-6 {
                errors.push(format!(
                    "Larghezza media troppo bassa: {}",
                    anagrafica.get_larghezza_media()
                ));
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
            let err_msg =
                "InfoAggiuntiveController: valida_anagrafica_hfbi() ha ricevuto uno stato spurio.";
            eprintln!("{}", err_msg);
            self.add_console_message(format!("InfoAggiuntiveController:  {err_msg}"));
        };
    }

    pub(crate) fn backout_anagrafica_hfbi(&self) {
        self.unset_console_env("anagrafica_hfbi".to_string());
        self.add_console_message(
            "InfoAggiuntiveController: L'utente ha annullato l'inserimento info aggiuntive."
                .to_string(),
        );
        let mut state = GLOBAL_STATE.lock().unwrap();
        assert!(state.data_model.get_anagrafica_hfbi().is_some());
        state.data_model.set_anagrafica_hfbi(None);
        state.infoaggiuntive_model.set_done_editing(false);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn set_console_env(&self, (key, val): (String, String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key, val));
    }

    pub(crate) fn unset_console_env(&self, key: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.console_model.console.remove_env(key);
    }
}
