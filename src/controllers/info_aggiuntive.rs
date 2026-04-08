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
pub(crate) struct InfoAggiuntiveController;
use crate::app::core::Action;
use crate::app::model::{Model, SubModel};
use crate::controllers::{Controller, CurrentView, InfoAggiuntiveModel};
use crate::MainState;
use chrono::format::ParseErrorKind;
use esox::csv::parser::parse_date;
use esox::domain::hfbi::AnagraficaHFBI;
use esox::domain::index::Indice;
use esox::domain::niseci::{AnagraficaNISECI, TipoComunitaNISECI};
use raylib::RaylibHandle;

impl Controller for InfoAggiuntiveController {
    type SubModel = InfoAggiuntiveModel;

    fn update(
        &self,
        _rl: &mut RaylibHandle,
        state: &mut Model,
        actions: &mut Vec<Action>,
        main_state: &mut MainState,
    ) {
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

        for a in actions.drain(..) {
            match a {
                Action::SubmitAnagraficaNISECI(anagrafica_draft) => {
                    let larghezza_stazione = match self.check_larghezza_stazione_string(
                        state,
                        &anagrafica_draft.larghezza_media_stazione,
                    ) {
                        Ok(v) => v,
                        Err(_e) => {
                            return; // We change view in the failed check maybe?
                        }
                    };
                    let lunghezza_stazione = match self.check_lunghezza_stazione_string(
                        state,
                        &anagrafica_draft.lunghezza_media_stazione,
                    ) {
                        Ok(v) => v,
                        Err(_e) => {
                            return; // We change view in the failed check maybe?
                        }
                    };
                    let anagrafica = AnagraficaNISECI {
                        comunita: anagrafica_draft.comunita,
                        codice_stazione: anagrafica_draft.codice_stazione,
                        date_string: anagrafica_draft.date_string,
                        area: anagrafica_draft.area,
                        corpo_idrico: anagrafica_draft.corpo_idrico,
                        bacino_appartenenza: anagrafica_draft.bacino_appartenenza,
                        idro_eco_regione: anagrafica_draft.idro_eco_regione,
                        posizione: anagrafica_draft.posizione,
                        lunghezza_media_stazione: lunghezza_stazione,
                        larghezza_media_stazione: larghezza_stazione,
                    };
                    self.submit_anagrafica_niseci(state, anagrafica);
                }
                Action::SubmitAnagraficaHFBI(anagrafica_draft) => {
                    let larghezza_transetto = match self.check_larghezza_stazione_string(
                        state,
                        &anagrafica_draft.larghezza_media_transetto,
                    ) {
                        Ok(v) => v,
                        Err(_e) => {
                            return; // We change view in the failed check maybe?
                        }
                    };
                    let lunghezza_transetto = match self.check_lunghezza_stazione_string(
                        state,
                        &anagrafica_draft.lunghezza_media_transetto,
                    ) {
                        Ok(v) => v,
                        Err(_e) => {
                            return; // We change view in the failed check maybe?
                        }
                    };
                    let anagrafica = AnagraficaHFBI {
                        codice_stazione: anagrafica_draft.codice_stazione,
                        corpo_idrico: anagrafica_draft.corpo_idrico,
                        posizione: anagrafica_draft.posizione,
                        date_string: anagrafica_draft.date_string,
                        tipo_laguna: anagrafica_draft.tipo_laguna,
                        stagione: anagrafica_draft.stagione,
                        habitat_vegetato: anagrafica_draft.habitat_vegetato,
                        lunghezza_media_transetto: lunghezza_transetto,
                        larghezza_media_transetto: larghezza_transetto,
                    };
                    self.submit_anagrafica_hfbi(state, anagrafica);
                }
                Action::CheckAnagrafica => {
                    if let Some(idx) = state.indice_model.get_selected_index() {
                        match idx {
                            Indice::Niseci => {
                                self.valida_anagrafica_niseci(state);
                            }
                            Indice::Hfbi => {
                                self.valida_anagrafica_hfbi(state);
                            }
                        }
                    } else {
                        eprintln!("InfoAggiuntiveController:  Can't handle action {} without a selected index", a);
                    }
                }
                Action::BackoutAnagrafica => {
                    if let Some(idx) = state.indice_model.get_selected_index() {
                        match idx {
                            Indice::Niseci => {
                                self.backout_anagrafica_niseci(state);
                            }
                            Indice::Hfbi => {
                                self.backout_anagrafica_hfbi(state);
                            }
                        }
                    } else {
                        eprintln!("InfoAggiuntiveController:  Can't process action {} without a selected index", a);
                    }
                }
                _ => {
                    println!("InfoAggiuntiveController:  Got action {}", a);
                }
            }
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
}

impl InfoAggiuntiveController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_data_anagrafica_niseci(&self, state: &Model) -> Option<AnagraficaNISECI> {
        state.data_model.get_anagrafica_niseci()
    }

    pub(crate) fn submit_anagrafica_niseci(&self, state: &mut Model, anagrafica: AnagraficaNISECI) {
        self.set_console_env(
            state,
            ("anagrafica_niseci".to_string(), format!("{anagrafica}")),
        );
        self.add_console_message(
            state,
            "InfoAggiuntiveController: L'utente ha completato l'inserimento info aggiuntive."
                .to_string(),
        );
        assert!(state.data_model.get_anagrafica_niseci().is_none());
        state.data_model.set_anagrafica_niseci(Some(anagrafica));
        state.infoaggiuntive_model.set_done_editing(true);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn check_larghezza_stazione_string(
        &self,
        state: &mut Model,
        larghezza: &str,
    ) -> Result<f32, String> {
        let s = larghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione larghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg
                        .replace("invalid float literal", "tipo non valido: atteso decimale");
                }
                self.add_console_message(state, format!("InfoAggiuntiveController:  {err_msg}"));
                state.data_model.set_anagrafica_niseci(None);
                state.infoaggiuntive_model.set_done_editing(false);
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                Err(err_msg)
            }
        }
    }

    pub(crate) fn check_lunghezza_stazione_string(
        &self,
        state: &mut Model,
        lunghezza: &str,
    ) -> Result<f32, String> {
        let s = lunghezza.replace(',', "."); // Replace comma with dot
        match s.parse::<f32>() {
            Ok(value) => Ok(value),
            Err(e) => {
                let mut err_msg = format!("Errore nella conversione lunghezza stazione: {}", e);
                if err_msg.contains("invalid float literal") {
                    err_msg = err_msg
                        .replace("invalid float literal", "tipo non valido: atteso decimale");
                }
                self.add_console_message(state, format!("InfoAggiuntiveController:  {err_msg}"));
                state.data_model.set_anagrafica_niseci(None);
                state.infoaggiuntive_model.set_done_editing(false);
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                Err(err_msg)
            }
        }
    }

    pub(crate) fn valida_anagrafica_niseci(&self, state: &mut Model) {
        assert!(state.infoaggiuntive_model.is_done_editing());

        if let Some(anagrafica) = self.get_data_anagrafica_niseci(state) {
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
                self.add_console_message(state, format!("InfoAggiuntiveController:  {e}"));
            }

            if !errors.is_empty() {
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                return;
            }
            state.infoaggiuntive_model.set_valid(true);
        } else {
            let err_msg = "InfoAggiuntiveController: valida_anagrafica_niseci() ha ricevuto uno stato spurio.";
            eprintln!("{}", err_msg);
            self.add_console_message(state, format!("InfoAggiuntiveController:  {err_msg}"));
        };
    }

    pub(crate) fn backout_anagrafica_niseci(&self, state: &mut Model) {
        self.unset_console_env(state, "anagrafica_niseci".to_string());
        self.add_console_message(
            state,
            "InfoAggiuntiveController: L'utente ha annullato l'inserimento info aggiuntive."
                .to_string(),
        );
        assert!(state.data_model.get_anagrafica_niseci().is_some());
        state.data_model.set_anagrafica_niseci(None);
        state.infoaggiuntive_model.set_done_editing(false);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn get_data_anagrafica_hfbi(&self, state: &Model) -> Option<AnagraficaHFBI> {
        state.data_model.get_anagrafica_hfbi()
    }

    pub(crate) fn submit_anagrafica_hfbi(&self, state: &mut Model, anagrafica: AnagraficaHFBI) {
        self.set_console_env(
            state,
            ("anagrafica_hfbi".to_string(), format!("{anagrafica}")),
        );
        self.add_console_message(
            state,
            "InfoAggiuntiveController: L'utente ha completato l'inserimento info aggiuntive."
                .to_string(),
        );
        assert!(state.data_model.get_anagrafica_hfbi().is_none());
        state.data_model.set_anagrafica_hfbi(Some(anagrafica));
        state.infoaggiuntive_model.set_done_editing(true);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn valida_anagrafica_hfbi(&self, state: &mut Model) {
        assert!(state.infoaggiuntive_model.is_done_editing());

        if let Some(anagrafica) = self.get_data_anagrafica_hfbi(state) {
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
                self.add_console_message(state, format!("InfoAggiuntiveController:  {e}"));
            }

            if !errors.is_empty() {
                state.infoaggiuntive_model.set_valid(false);
                state.infoaggiuntive_model.set_errors_occurred(true);
                return;
            }
            state.infoaggiuntive_model.set_valid(true);
        } else {
            let err_msg =
                "InfoAggiuntiveController: valida_anagrafica_hfbi() ha ricevuto uno stato spurio.";
            eprintln!("{}", err_msg);
            self.add_console_message(state, format!("InfoAggiuntiveController:  {err_msg}"));
        };
    }

    pub(crate) fn backout_anagrafica_hfbi(&self, state: &mut Model) {
        self.unset_console_env(state, "anagrafica_hfbi".to_string());
        self.add_console_message(
            state,
            "InfoAggiuntiveController: L'utente ha annullato l'inserimento info aggiuntive."
                .to_string(),
        );
        assert!(state.data_model.get_anagrafica_hfbi().is_some());
        state.data_model.set_anagrafica_hfbi(None);
        state.infoaggiuntive_model.set_done_editing(false);
        state.infoaggiuntive_model.set_valid(false);
    }

    pub(crate) fn set_console_env(&self, state: &mut Model, (key, val): (String, String)) {
        state.console_model.console.set_env((key, val));
    }

    pub(crate) fn unset_console_env(&self, state: &mut Model, key: String) {
        state.console_model.console.remove_env(key);
    }
}
