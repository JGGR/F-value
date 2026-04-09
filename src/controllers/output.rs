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
pub(crate) struct OutputController;
use crate::app::core::{Action, Localize};
use crate::app::model::{Model, SubModel};
use crate::controllers::{Controller, CurrentView, OutputModel};
use crate::core::pdf::{esporta_pdf_hfbi, esporta_pdf_niseci};
use crate::core::{gen_logfile_name, CommaFormat};
use crate::MainState;
use dirs::document_dir;
use esox::domain::hfbi::{AnagraficaHFBI, RisultatoHFBI, ValoriIntermediHFBI};
use esox::domain::index::Indice;
use esox::domain::niseci::{AnagraficaNISECI, RisultatoNISECI, ValoriIntermediNISECI};
use esox::engines::hfbi::full::{calculate_hfbi, calculate_stato_ecologico_hfbi};
use esox::engines::niseci::full::{
    calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico_niseci,
};
use raylib::RaylibHandle;
use std::fs::{create_dir, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

impl Controller for OutputController {
    type SubModel = OutputModel;

    fn update(
        &self,
        _rl: &mut RaylibHandle,
        state: &mut Model,
        actions: &mut Vec<Action>,
        main_state: &mut MainState,
    ) {
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
            state.output_model.reset();
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

        for a in actions.drain(..) {
            match a {
                Action::RunCalc => {
                    if let Some(idx) = state.indice_model.get_selected_index() {
                        match idx {
                            Indice::Niseci => {
                                self.calc_niseci(state, main_state.locale);
                            }
                            Indice::Hfbi => {
                                self.calc_hfbi(state, main_state.locale);
                            }
                        }
                    } else {
                        eprintln!(
                            "OutputController:  Can't handle action {} without a selected index",
                            a
                        );
                    }
                }
                Action::ConfirmCalc => {
                    self.user_confirm_calc(state);
                }
                Action::ExportPdf(path) => {
                    if let Some(idx) = state.indice_model.get_selected_index() {
                        match idx {
                            Indice::Niseci => {
                                self.esporta_pdf_niseci(state, path);
                            }
                            Indice::Hfbi => {
                                self.esporta_pdf_hfbi(state, path);
                            }
                        }
                    } else {
                        eprintln!(
                            "OutputController:  Can't handle action pdf export without a selected index"

                        );
                    }
                }
                Action::Reset => {
                    self.prompt_reset(state);
                }
                _ => {
                    println!("OutputController:  Got action {}", a);
                }
            }
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
}

impl OutputController {
    pub(crate) fn new() -> Self {
        Self
    }

    fn prep_logfile_dir(&self) -> Option<PathBuf> {
        if let Some(documents_dir) = document_dir() {
            let dir = documents_dir.join("f_value");
            if let Err(e) = create_dir(&dir) {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    eprintln!("Failed to create dir: {}, {}", dir.display(), e);
                    return None;
                }
            };
            Some(dir)
        } else {
            let dir = PathBuf::from("./f_value");
            if let Err(e) = create_dir(&dir) {
                if e.kind() != std::io::ErrorKind::AlreadyExists {
                    eprintln!("Failed to create dir: {}, {}", dir.display(), e);
                    return None;
                }
            };
            Some(dir)
        }
    }

    pub(crate) fn get_is_done_calc(&self, state: &Model) -> bool {
        state.output_model.is_done_calc()
    }

    fn set_data_risultato_niseci(&self, state: &mut Model, risultato: RisultatoNISECI) {
        self.set_console_env(
            state,
            ("risultato_niseci".to_string(), format!("{risultato}")),
        );
        state.data_model.set_risultato_niseci(Some(risultato));
        state.output_model.set_done_calc(true);
    }

    pub(crate) fn calc_niseci(&self, state: &mut Model, locale: Localize) {
        let riferimento = state.data_model.get_riferimento_niseci();
        let campionamento = state.data_model.get_campionamento_niseci();
        let anagrafica = state.data_model.get_anagrafica_niseci();

        let mut valid = true;

        if riferimento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: riferimento niseci was None in calc_niseci()".to_string(),
            );
        }
        if campionamento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: campionamento niseci was None in calc_niseci()".to_string(),
            );
        }
        if anagrafica.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: anagrafica niseci was None in calc_niseci()".to_string(),
            );
        }
        if valid {
            let riferimento = riferimento.expect("calc_niseci() checked is_none() before");
            let campionamento = campionamento.expect("calc_niseci() checked is_none() before");
            let anagrafica = anagrafica.expect("calc_niseci() checked is_none() before");

            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, intermediates)) => {
                    let niseci_str = match niseci {
                        Some(val) => match locale {
                            Localize::Italian => val.comma().to_string(),
                            Localize::International => {
                                format!("{val}")
                            }
                        },
                        None => "NC".to_string(),
                    };
                    self.add_console_message(state, format!("NISECI: {niseci_str}"));

                    let rqe_niseci = calculate_rqe_niseci(niseci);
                    let rqe_niseci_str = match rqe_niseci {
                        Some(val) => match locale {
                            Localize::Italian => val.comma().to_string(),
                            Localize::International => {
                                format!("{val}")
                            }
                        },
                        None => "NC".to_string(),
                    };
                    self.add_console_message(state, format!("RQE NISECI: {rqe_niseci_str}"));

                    let stato_ecologico =
                        calculate_stato_ecologico_niseci(niseci, &anagrafica.area);
                    let stato_ecologico_str = match stato_ecologico {
                        Some(val) => {
                            format!("{val}")
                        }
                        None => "NC".to_string(),
                    };
                    self.add_console_message(
                        state,
                        format!("Stato ecologico: {stato_ecologico_str}"),
                    );

                    let risultato_niseci =
                        RisultatoNISECI::new(niseci, rqe_niseci, intermediates.clone());
                    self.log_niseci_values(
                        locale,
                        &anagrafica,
                        &risultato_niseci,
                        &intermediates,
                        &state
                            .fileinput_model
                            .get_riferimento_path()
                            .expect("Failed initialising riferimento niseci path"),
                    );

                    //This logs to stdout
                    intermediates.log();

                    self.add_console_message(state, format!("{intermediates}"));

                    self.log_niseci_intermediates(
                        locale,
                        &intermediates,
                        &state
                            .fileinput_model
                            .get_riferimento_path()
                            .expect("Failed initialising riferimento niseci path"),
                        &anagrafica.codice_stazione,
                    );

                    self.set_data_risultato_niseci(state, risultato_niseci);
                    println!("OutputController: Finished NISECI calc");
                }
                Err(niseci_errors) => {
                    for e in niseci_errors {
                        self.add_console_message(
                            state,
                            format!("Errore durante il calcolo NISECI: {}", e),
                        );
                    }
                    state.data_model.set_errors_occurred(true);
                    state.output_model.set_done_calc(false);
                    state.data_model.set_risultato_niseci(None);
                }
            }
        } else {
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: spurious state in calc_niseci()".to_string(),
            );
            state.data_model.set_errors_occurred(true);
        }
    }

    /// TODO: Once RisultatoNISECI lets us get x3_a and x3_b, we can drop the intermediates arg
    pub(crate) fn log_niseci_values(
        &self,
        locale: Localize,
        anagrafica: &AnagraficaNISECI,
        risultato: &RisultatoNISECI,
        intermediates: &ValoriIntermediNISECI,
        ref_filename: &Path,
    ) {
        let niseci = risultato.get_valore();
        let niseci_str = match niseci {
            Some(val) => match locale {
                Localize::Italian => val.comma().to_string(),
                Localize::International => {
                    format!("{val}")
                }
            },
            None => "NC".to_string(),
        };

        let rqe_niseci = calculate_rqe_niseci(niseci);
        let rqe_niseci_str = match rqe_niseci {
            Some(val) => match locale {
                Localize::Italian => val.comma().to_string(),
                Localize::International => {
                    format!("{val}")
                }
            },
            None => "NC".to_string(),
        };

        let stato_ecologico = calculate_stato_ecologico_niseci(niseci, &anagrafica.area);
        let stato_ecologico_str = match stato_ecologico {
            Some(val) => {
                format!("{val}")
            }
            None => "NC".to_string(),
        };
        let name = gen_logfile_name(ref_filename, &anagrafica.codice_stazione, true);
        let log_file_path;
        if let Some(dir) = self.prep_logfile_dir() {
            log_file_path = dir.join(name);
        } else {
            return;
        }
        let file_result = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(log_file_path);

        match file_result {
            Ok(mut file) => {
                let string_representation = match locale {
                    Localize::Italian => {
                        format!("Codice stazione; Data; Regione; Idroecoregione; Area pertinenza; Bacino; NISECI; RQE NISECI; Stato ecologico; x1; x2; x3; x3_a; x3_b\n{}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}",
                            anagrafica.codice_stazione,
                            anagrafica.date_string,
                            anagrafica.posizione.regione,
                            anagrafica.idro_eco_regione,
                            anagrafica.area,
                            anagrafica.corpo_idrico,
                            niseci_str,
                            rqe_niseci_str,
                            stato_ecologico_str,
                            intermediates.x1.comma(),
                            match intermediates.x2 {
                                Some(v) => v.comma().to_string(),
                                None => "NC".to_string(),
                            },
                            intermediates.x3.comma(),
                            match intermediates.x3_a {
                                Some(v) => v.comma().to_string(),
                                None => "NC".to_string(),
                            },
                            match intermediates.x3_b {
                                Some(v) => v.comma().to_string(),
                                None => "NC".to_string(),
                            }
                        )
                    }
                    Localize::International => {
                        format!("Codice stazione, Data, Regione, Idroecoregione, Area pertinenza, Bacino, NISECI, RQE NISECI, Stato ecologico, x1, x2, x3, x3_a, x3_b\n{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
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
                        )
                    }
                };
                let write_result = writeln!(file, "{string_representation}");
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

    pub(crate) fn log_niseci_intermediates(
        &self,
        locale: Localize,
        intermediates: &ValoriIntermediNISECI,
        ref_filename: &Path,
        station_code: &str,
    ) {
        let name = gen_logfile_name(ref_filename, station_code, false);
        let log_file_path;
        if let Some(dir) = self.prep_logfile_dir() {
            log_file_path = dir.join(name);
        } else {
            return;
        }

        let file_result = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(log_file_path);

        match file_result {
            Ok(mut file) => {
                let mut string_representation = match locale {
                    Localize::Italian => {
                        "specie; nome latino; tipo autoctono; tipo alloctono; specie attesa; cl1; cl2; cl3; cl4; cl5; densita stimata; quantita stimata; x2b; rapporto ad/juv; x2a_a; x2a_b".to_string()
                    }
                    Localize::International => {
                        "specie, nome latino, tipo autoctono, tipo alloctono, specie attesa, cl1, cl2, cl3, cl4, cl5, densita stimata, quantita stimata, x2b, rapporto ad/juv, x2a_a, x2a_b".to_string()
                    }
                };
                for (_k, v) in intermediates.specie_specifici.iter() {
                    let rapporto_ad_juv_str = match v.rapporto_ad_juv {
                        Some(v) => match locale {
                            Localize::Italian => v.comma().to_string(),
                            Localize::International => format!("{v}"),
                        },
                        None => "NC".to_string(),
                    };
                    let specie_attesa_str = if v.classi_eta.specie.specie_attesa {
                        "SI".to_string()
                    } else {
                        "NO".to_string()
                    };
                    string_representation = match locale {
                        Localize::Italian => {
                            format!(
                                "{}\n{}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}",
                                string_representation,
                                v.classi_eta.specie.id,
                                v.classi_eta.specie.nome,
                                v.classi_eta.specie.tipo_autoctono,
                                v.classi_eta.specie.tipo_alloctono,
                                specie_attesa_str,
                                v.classi_eta.cl1,
                                v.classi_eta.cl2,
                                v.classi_eta.cl3,
                                v.classi_eta.cl4,
                                v.classi_eta.cl5,
                                v.densita_stimata.comma(),
                                v.quantita_stimata,
                                v.x2_b,
                                rapporto_ad_juv_str,
                                v.x2_a_a,
                                v.x2_a_b
                            )
                        }
                        Localize::International => {
                            format!(
                                "{}\n{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                                string_representation,
                                v.classi_eta.specie.id,
                                v.classi_eta.specie.nome,
                                v.classi_eta.specie.tipo_autoctono,
                                v.classi_eta.specie.tipo_alloctono,
                                specie_attesa_str,
                                v.classi_eta.cl1,
                                v.classi_eta.cl2,
                                v.classi_eta.cl3,
                                v.classi_eta.cl4,
                                v.classi_eta.cl5,
                                v.densita_stimata,
                                v.quantita_stimata,
                                v.x2_b,
                                rapporto_ad_juv_str,
                                v.x2_a_a,
                                v.x2_a_b
                            )
                        }
                    }
                }
                let write_result = writeln!(file, "{string_representation}");
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

    pub(crate) fn esporta_pdf_niseci(&self, state: &mut Model, export_path: PathBuf) {
        self.add_console_message(
            state,
            format!("Esportazione pdf in {}", export_path.display()),
        );

        let risultato_niseci = state
            .data_model
            .get_risultato_niseci()
            .expect("Failed calculating NISECI before requesting export");

        let anagrafica_niseci = state
            .data_model
            .get_anagrafica_niseci()
            .expect("Failed getting AnagraficaNISECI before requesting export");

        let riferimento_niseci = state
            .data_model
            .get_riferimento_niseci()
            .expect("Failed getting RiferimentoNISECI before requesting export");

        esporta_pdf_niseci(
            export_path,
            riferimento_niseci,
            anagrafica_niseci,
            risultato_niseci,
        );
        self.set_done_export(state, true);
    }

    pub(crate) fn calc_hfbi(&self, state: &mut Model, locale: Localize) {
        let campionamento = state.data_model.get_campionamento_hfbi();
        let anagrafica = state.data_model.get_anagrafica_hfbi();

        let mut valid = true;

        if campionamento.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: campionamento hfbi was None in calc_hfbi()".to_string(),
            );
        }
        if anagrafica.is_none() {
            // Implementation error, this should never happen
            valid = false;
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: anagrafica hfbi was None in calc_hfbi()".to_string(),
            );
        }

        if valid {
            let campionamento = campionamento.expect("calc_hfbi() checked is_none() before");
            let anagrafica = anagrafica.expect("calc_hfbi() checked is_none() before");

            match calculate_hfbi(&campionamento, &anagrafica) {
                Ok((hfbi, intermediates)) => {
                    self.add_console_message(state, format!("HFBI: {hfbi}"));

                    let stato_ecologico = calculate_stato_ecologico_hfbi(Some(hfbi));
                    let stato_ecologico_str = match stato_ecologico {
                        Some(val) => {
                            format!("{val}")
                        }
                        None => "NC".to_string(),
                    };
                    self.add_console_message(
                        state,
                        format!("Stato ecologico: {stato_ecologico_str}"),
                    );

                    self.log_hfbi_values(
                        locale,
                        &anagrafica,
                        hfbi,
                        stato_ecologico_str,
                        &intermediates,
                        &state
                            .fileinput_model
                            .get_campionamento_path()
                            .expect("Failed initialising campionamento hfbi path"),
                    );

                    //This logs to stdout
                    intermediates.log();
                    println!("HFBI: {hfbi}");

                    self.add_console_message(state, format!("{intermediates}"));

                    self.log_hfbi_intermediates(
                        locale,
                        &intermediates,
                        &state
                            .fileinput_model
                            .get_campionamento_path()
                            .expect("Failed initialising campionamento hfbi path"),
                        &anagrafica.codice_stazione,
                    );

                    let risultato_hfbi = RisultatoHFBI::new(Some(hfbi), intermediates);

                    self.set_data_risultato_hfbi(state, risultato_hfbi);
                    println!("OutputController: Finished HFBI calc");
                }
                Err(hfbi_errors) => {
                    self.add_console_message(
                        state,
                        format!("Errore durante il calcolo HFBI: {}", hfbi_errors),
                    );
                    state.data_model.set_errors_occurred(true);
                    state.output_model.set_done_calc(false);
                    state.data_model.set_risultato_hfbi(None);
                }
            }
        } else {
            self.add_console_message(
                state,
                "IMPLEMENTATION ERROR: spurious state in calc_hfbi()".to_string(),
            );
            state.data_model.set_errors_occurred(true);
        }
    }

    pub(crate) fn log_hfbi_values(
        &self,
        locale: Localize,
        anagrafica: &AnagraficaHFBI,
        hfbi: f32,
        stato_ecologico_str: String,
        intermediates: &ValoriIntermediHFBI,
        samp_filename: &Path,
    ) {
        let name = gen_logfile_name(samp_filename, &anagrafica.codice_stazione, true);
        let log_file_path;
        if let Some(dir) = self.prep_logfile_dir() {
            log_file_path = dir.join(name);
        } else {
            return;
        }
        let file_result = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(log_file_path);

        match file_result {
            Ok(mut file) => {
                let string_representation = match locale {
                    Localize::Italian => {
                        format!("Codice stazione; stagione; habitat vegetato; tipo laguna; MMI; HFBI; Stato ecologico\n{}; {}; {}; {}; {}; {}; {}",
                                anagrafica.codice_stazione,
                                anagrafica.stagione,
                                anagrafica.habitat_vegetato,
                                anagrafica.tipo_laguna,
                                intermediates.mmi.comma(),
                                hfbi.comma(),
                                stato_ecologico_str
                            )
                    }
                    Localize::International => {
                        format!("Codice stazione, stagione, habitat vegetato, tipo laguna, MMI, HFBI, Stato ecologico\n{}, {}, {}, {}, {}, {}, {}",
                                anagrafica.codice_stazione,
                                anagrafica.stagione,
                                anagrafica.habitat_vegetato,
                                anagrafica.tipo_laguna,
                                intermediates.mmi,
                                hfbi,
                                stato_ecologico_str
                            )
                    }
                };
                let write_result = writeln!(file, "{string_representation}");
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

    pub(crate) fn log_hfbi_intermediates(
        &self,
        locale: Localize,
        intermediates: &ValoriIntermediHFBI,
        samp_filename: &Path,
        station_code: &str,
    ) {
        let name = gen_logfile_name(samp_filename, station_code, false);
        let log_file_path;
        if let Some(dir) = self.prep_logfile_dir() {
            log_file_path = dir.join(name);
        } else {
            return;
        }
        let file_result = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(log_file_path);

        match file_result {
            Ok(mut file) => {
                let string_representation = match locale {
                    Localize::Italian => {
                        format!(
                            "bbent; bn; dbent; ddom; dhzp; dmig\n{}; {}; {}; {}; {}; {}",
                            intermediates.bbent.comma(),
                            intermediates.bn.comma(),
                            intermediates.dbent.comma(),
                            intermediates.ddom.comma(),
                            intermediates.dhzp.comma(),
                            intermediates.dmig.comma()
                        )
                    }
                    Localize::International => {
                        format!(
                            "bbent, bn, dbent, ddom, dhzp, dmig\n{}, {}, {}, {}, {}, {}",
                            intermediates.bbent,
                            intermediates.bn,
                            intermediates.dbent,
                            intermediates.ddom,
                            intermediates.dhzp,
                            intermediates.dmig
                        )
                    }
                };
                let write_result = writeln!(file, "{string_representation}");
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

    pub(crate) fn esporta_pdf_hfbi(&self, state: &mut Model, export_path: PathBuf) {
        self.add_console_message(
            state,
            format!("Esportazione pdf in {}", export_path.display()),
        );

        let risultato_hfbi = self
            .get_data_risultato_hfbi(state)
            .expect("Failed calculating HFBI before requesting export");

        let anagrafica_hfbi = self
            .get_data_anagrafica_hfbi(state)
            .expect("Failed getting AnagraficaHFBI before requesting export");

        esporta_pdf_hfbi(export_path, anagrafica_hfbi, risultato_hfbi);
        self.set_done_export(state, true);
    }

    pub(crate) fn user_confirm_calc(&self, state: &mut Model) {
        state.output_model.set_done_user_confirm(true);
    }

    pub(crate) fn set_done_export(&self, state: &mut Model, val: bool) {
        state.output_model.set_done_export(val);
    }

    pub(crate) fn get_data_risultato_hfbi(&self, state: &Model) -> Option<RisultatoHFBI> {
        if self.get_is_done_calc(state) {
            state.data_model.get_risultato_hfbi()
        } else {
            None
        }
    }

    pub(crate) fn get_data_anagrafica_hfbi(&self, state: &Model) -> Option<AnagraficaHFBI> {
        state.data_model.get_anagrafica_hfbi()
    }

    fn set_data_risultato_hfbi(&self, state: &mut Model, risultato: RisultatoHFBI) {
        self.set_console_env(
            state,
            ("risultato_hfbi".to_string(), format!("{risultato}")),
        );
        state.data_model.set_risultato_hfbi(Some(risultato));
        state.output_model.set_done_calc(true);
    }

    pub(crate) fn prompt_reset(&self, state: &mut Model) {
        state.output_model.set_should_reset(true);
    }

    pub(crate) fn set_console_env(&self, state: &mut Model, (key, val): (String, String)) {
        state.console_model.console.set_env((key, val));
    }
}
