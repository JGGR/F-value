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
pub(crate) struct OutputController;
use crate::app::model::SubModel;
use crate::controllers::{Controller, CurrentView, OutputModel};
use crate::core::pdf::{esporta_pdf_hfbi, esporta_pdf_niseci};
use crate::domain::hfbi::{AnagraficaHFBI, RisultatoHFBI};
use crate::domain::niseci::{
    AnagraficaNISECI, RiferimentoNISECI, RisultatoNISECI, StatoEcologicoNISECI,
};
use crate::engines::hfbi::full::calculate_hfbi;
use crate::engines::niseci::full::{
    calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico,
};
use crate::state::GLOBAL_STATE;
use crate::MainState;
#[cfg(feature = "logged")]
use log::info;
use raylib::RaylibHandle;
use std::path::PathBuf;

#[cfg(feature = "logged")]
use dirs::document_dir;

#[cfg(feature = "logged")]
use std::fs::OpenOptions;
#[cfg(feature = "logged")]
use std::io::Write;

impl Controller for OutputController {
    type SubModel = OutputModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.output_model.increment_frame_counter();

        if state.output_model.get_should_reset() {
            main_state.showing_reset_win = true;
            state.output_model.set_should_reset(false);
            return;
        }
        if main_state.should_reset {
            eprintln!("OutputController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.fileinput_model.reset();
            state.infoaggiuntive_model.reset();
            state.data_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }

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
                    eprintln!(
                        "OutputController:  Let's update current view and go to ProduzionePDF."
                    );
                    main_state.set_current_view(CurrentView::ProduzionePDF);
                }
            }
            CurrentView::ProduzionePDF => {}
            _ => {}
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.output_model.clone()
    }
}

impl OutputController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_is_done_calc(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.output_model.is_done_calc()
    }

    pub(crate) fn get_is_done_export(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        state.output_model.is_done_export()
    }

    pub(crate) fn get_niseci_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => r.get_valore(),
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_rqe_niseci_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => r.get_rqe(),
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_stato_eco_niseci_value(&self) -> Option<StatoEcologicoNISECI> {
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
                        None => None,
                    }
                }
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_x1_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            opt_res.map(|r| r.get_x1())
        } else {
            None
        }
    }

    pub(crate) fn get_x2_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_niseci();
            match opt_res {
                Some(r) => r.get_x2(),
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_x3_value(&self) -> Option<f32> {
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

    pub(crate) fn get_data_risultato_niseci(&self) -> Option<RisultatoNISECI> {
        if self.get_is_done_calc() {
            let state = GLOBAL_STATE.lock().unwrap();
            state.data_model.get_risultato_niseci()
        } else {
            None
        }
    }

    pub(crate) fn get_data_anagrafica_niseci(&self) -> Option<AnagraficaNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_anagrafica_niseci()
    }

    pub(crate) fn get_data_riferimento_niseci(&self) -> Option<RiferimentoNISECI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_riferimento_niseci()
    }

    pub(crate) fn calc_niseci(&self) {
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
            self.add_console_message(
                "IMPLEMENTATION ERROR: riferimento niseci was None in calc_niseci()".to_string(),
            );
        }
        if campionamento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                "IMPLEMENTATION ERROR: campionamento niseci was None in calc_niseci()".to_string(),
            );
        }
        if anagrafica.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                "IMPLEMENTATION ERROR: anagrafica niseci was None in calc_niseci()".to_string(),
            );
        }
        if valid {
            let riferimento = riferimento.expect("calc_niseci() checked is_none() before");
            let campionamento = campionamento.expect("calc_niseci() checked is_none() before");
            let anagrafica = anagrafica.expect("calc_niseci() checked is_none() before");

            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, intermediates)) => {
                    let niseci_str;
                    match niseci {
                        Some(val) => {
                            niseci_str = format!("{val}");
                        }
                        None => {
                            niseci_str = format!("NC");
                        }
                    }
                    self.add_console_message(format!("NISECI: {niseci_str}"));

                    let rqe_niseci = calculate_rqe_niseci(niseci);
                    let rqe_niseci_str;

                    match rqe_niseci {
                        Some(val) => {
                            rqe_niseci_str = format!("{val}");
                        }
                        None => {
                            rqe_niseci_str = format!("NC");
                        }
                    }
                    self.add_console_message(format!("RQE NISECI: {rqe_niseci_str}"));

                    let stato_ecologico = calculate_stato_ecologico(niseci, &anagrafica.area);
                    let stato_ecologico_str;

                    match stato_ecologico {
                        Some(val) => {
                            stato_ecologico_str = format!("{val}");
                        }
                        None => {
                            stato_ecologico_str = format!("NC");
                        }
                    }
                    self.add_console_message(format!("Stato ecologico: {stato_ecologico_str}"));

                    #[cfg(feature = "logged")]
                    {
                        info!("Codice stazione; Data; Regione; Idroecoregione; Area pertinenza; Bacino; NISECI; RQE NISECI; Stato ecologico; x1; x2; x3; x3_a; x3_b\n{}",
                            format!("{}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}",
                            anagrafica.codice_stazione,
                            anagrafica.date_string,
                            anagrafica.posizione.regione,
                            anagrafica.idro_eco_regione,
                            anagrafica.area,
                            anagrafica.corpo_idrico,
                            niseci_str,
                            rqe_niseci_str,
                            stato_ecologico_str,
                            intermediates.x1,
                            match intermediates.x2 {
                                Some(v) => format!("{v}"),
                                None => "NC".to_string(),
                            },
                            intermediates.x3,
                            match intermediates.x3_a {
                                Some(v) => format!("{v}"),
                                None => "NC".to_string(),
                            },
                            match intermediates.x3_b {
                                Some(v) => format!("{v}"),
                                None => "NC".to_string(),
                            }
                        ));
                    }

                    //This logs to stdout
                    intermediates.log();

                    self.add_console_message(format!("{intermediates}"));

                    //TODO: format intermediates properly
                    #[cfg(feature = "logged")]
                    {
                        let log_file_path;
                        if let Some(documents_dir) = document_dir() {
                            log_file_path =
                                documents_dir.join("f_value").join("log_intermediates.csv");
                        } else {
                            log_file_path = PathBuf::from("./f_value/log_intermediates.csv");
                        }

                        let file_result = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(log_file_path);

                        match file_result {
                            Ok(mut file) => {
                                let mut string_representation = format!("specie; nome latino; tipo autoctono; tipo alloctono; specie attesa; cl1; cl2; cl3; cl4; cl5; densita stimata; rapporto ad/juv; x2a_a; x2a_b");
                                for (_k, v) in intermediates.specie_specifici.iter() {
                                    string_representation =
                                        format!("{}\n{}", string_representation, v);
                                }
                                let write_result =
                                    writeln!(file, "{}", format!("{string_representation}"));
                                match write_result {
                                    Ok(_) => println!("Successfully wrote to file."),
                                    Err(e) => eprintln!("Failed to write to file: {}", e),
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to open file: {}", e);
                            }
                        }
                    }

                    let risultato_niseci = RisultatoNISECI::new(niseci, rqe_niseci, intermediates);

                    self.set_data_risultato_niseci(risultato_niseci);
                    println!("OutputController: Finished NISECI calc");
                }
                Err(niseci_errors) => {
                    for e in niseci_errors {
                        self.add_console_message(format!(
                            "Errore durante il calcolo NISECI: {}",
                            e
                        ));
                    }
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.data_model.set_errors_occurred(true);
                    state.output_model.set_done_calc(false);
                    state.data_model.set_risultato_niseci(None);
                }
            }
        } else {
            self.add_console_message(
                "IMPLEMENTATION ERROR: spurious state in calc_niseci()".to_string(),
            );
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.data_model.set_errors_occurred(true);
        }
    }

    pub(crate) fn esporta_pdf_niseci(&self, export_path: PathBuf) {
        self.add_console_message(format!("Esportazione pdf in {}", export_path.display()));

        let risultato_niseci = self
            .get_data_risultato_niseci()
            .expect("Failed calculating NISECI before requesting export");

        let anagrafica_niseci = self
            .get_data_anagrafica_niseci()
            .expect("Failed getting AnagraficaNISECI before requesting export");

        let riferimento_niseci = self
            .get_data_riferimento_niseci()
            .expect("Failed getting RiferimentoNISECI before requesting export");

        esporta_pdf_niseci(
            export_path,
            riferimento_niseci,
            anagrafica_niseci,
            risultato_niseci,
        );
        self.set_done_export(true);
    }

    pub(crate) fn calc_hfbi(&self) {
        let campionamento;
        let anagrafica;
        {
            let mut state = GLOBAL_STATE.lock().unwrap();
            campionamento = state.data_model.get_campionamento_hfbi();
            anagrafica = state.data_model.get_anagrafica_hfbi();
        }

        let mut valid = true;

        if campionamento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                "IMPLEMENTATION ERROR: campionamento hfbi was None in calc_hfbi()".to_string(),
            );
        }
        if anagrafica.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                "IMPLEMENTATION ERROR: anagrafica hfbi was None in calc_hfbi()".to_string(),
            );
        }

        if valid {
            let campionamento = campionamento.expect("calc_hfbi() checked is_none() before");
            let anagrafica = anagrafica.expect("calc_hfbi() checked is_none() before");

            match calculate_hfbi(&campionamento, &anagrafica) {
                Ok((hfbi, intermediates)) => {
                    self.add_console_message(format!("HFBI: {hfbi}"));

                    #[cfg(feature = "logged")]
                    {
                        info!("Codice stazione, stagione, habitat vegetato, tipo laguna, MMI, HFBI\n{}",
                            format!("{}, {}, {}, {}, {}, {}",
                            anagrafica.codice_stazione,
                            anagrafica.stagione,
                            anagrafica.habitat_vegetato,
                            anagrafica.tipo_laguna,
                            intermediates.mmi,
                            hfbi
                        ));
                    }

                    //This logs to stdout
                    intermediates.log();
                    println!("HFBI: {hfbi}");

                    self.add_console_message(format!("{intermediates}"));

                    //TODO: format intermediates properly
                    #[cfg(feature = "logged")]
                    {
                        let log_file_path;
                        if let Some(documents_dir) = document_dir() {
                            log_file_path =
                                documents_dir.join("f_value").join("log_intermediates.csv");
                        } else {
                            log_file_path = PathBuf::from("./f_value/log_intermediates.csv");
                        }

                        let file_result = OpenOptions::new()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(log_file_path);

                        match file_result {
                            Ok(mut file) => {
                                let string_representation = format!(
                                    "bbent, bn, dbent, ddom, dhzp, dmig\n{}, {}, {}, {}, {}, {}",
                                    intermediates.bbent,
                                    intermediates.bn,
                                    intermediates.dbent,
                                    intermediates.ddom,
                                    intermediates.dhzp,
                                    intermediates.dmig
                                );
                                let write_result =
                                    writeln!(file, "{}", format!("{string_representation}"));
                                match write_result {
                                    Ok(_) => println!("Successfully wrote to file."),
                                    Err(e) => eprintln!("Failed to write to file: {}", e),
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to open file: {}", e);
                            }
                        }
                    }
                    let risultato_hfbi = RisultatoHFBI::new(Some(hfbi), intermediates);

                    self.set_data_risultato_hfbi(risultato_hfbi);
                    println!("OutputController: Finished HFBI calc");
                }
                Err(hfbi_errors) => {
                    self.add_console_message(format!(
                        "Errore durante il calcolo HFBI: {}",
                        hfbi_errors
                    ));
                    let mut state = GLOBAL_STATE.lock().unwrap();
                    state.data_model.set_errors_occurred(true);
                    state.output_model.set_done_calc(false);
                    state.data_model.set_risultato_hfbi(None);
                }
            }
        } else {
            self.add_console_message(
                "IMPLEMENTATION ERROR: spurious state in calc_hfbi()".to_string(),
            );
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.data_model.set_errors_occurred(true);
        }
    }

    pub(crate) fn esporta_pdf_hfbi(&self, export_path: PathBuf) {
        self.add_console_message(format!("Esportazione pdf in {}", export_path.display()));

        let risultato_hfbi = self
            .get_data_risultato_hfbi()
            .expect("Failed calculating HFBI before requesting export");

        let anagrafica_hfbi = self
            .get_data_anagrafica_hfbi()
            .expect("Failed getting AnagraficaHFBI before requesting export");

        esporta_pdf_hfbi(export_path, anagrafica_hfbi, risultato_hfbi);
        self.set_done_export(true);
    }

    pub(crate) fn user_confirm_calc(&self) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.output_model.set_done_user_confirm(true);
    }

    pub(crate) fn set_done_export(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.output_model.set_done_export(val);
    }

    pub(crate) fn get_hfbi_value(&self) -> Option<f32> {
        if self.get_is_done_calc() {
            let opt_res = self.get_data_risultato_hfbi();
            match opt_res {
                Some(r) => r.get_valore(),
                None => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn get_data_risultato_hfbi(&self) -> Option<RisultatoHFBI> {
        if self.get_is_done_calc() {
            let state = GLOBAL_STATE.lock().unwrap();
            state.data_model.get_risultato_hfbi()
        } else {
            None
        }
    }

    pub(crate) fn get_data_anagrafica_hfbi(&self) -> Option<AnagraficaHFBI> {
        let state = GLOBAL_STATE.lock().unwrap();
        state.data_model.get_anagrafica_hfbi()
    }

    fn set_data_risultato_hfbi(&self, risultato: RisultatoHFBI) {
        self.set_console_env(("risultato_hfbi".to_string(), format!("{risultato}")));
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.data_model.set_risultato_hfbi(Some(risultato));
        state.output_model.set_done_calc(true);
    }

    pub(crate) fn prompt_reset(&self) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.output_model.set_should_reset(true);
    }

    pub(crate) fn set_console_env(&self, (key, val): (String, String)) {
        let mut state = GLOBAL_STATE.lock().unwrap();

        state.console_model.console.set_env((key, val));
    }
}
