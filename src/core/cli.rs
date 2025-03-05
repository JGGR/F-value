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

use crate::core::*;
use crate::engines::niseci::full::{calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico};
use crate::model::niseci::{ CampionamentoNISECI, RiferimentoNISECI };

pub fn esox_usage() {
    println!("{PROJECT_NAME} v{SHORT_PROJECT_VERSION}");
    println!("Usage: {PROJECT_NAME} [--headless] <campionamento.csv> <riferimento.csv> <anagrafica.csv>");
    println!("       {PROJECT_NAME} [--headless] --hfbi <campionamento.csv> <anagrafica.csv>");
    println!(
        "Flags:
  --headless               Run without GUI
  --hfbi                   Run with HFBI
  --version, -v            Print version and quit
  --info                   Print debug info and quit
  --help, -h               Print this message and quit");
}

pub fn run_headless(do_niseci: bool, args: &Vec<String>) -> bool {
    let mut arg_i = 0;
    let mut campionamento_path_str = "";
    let mut riferimento_path_str = "";
    let mut anagrafica_path_str = "";
    for arg in &args[1..] {
        arg_i += 1;
        match arg_i {
            1 => {
                campionamento_path_str = arg;
            }
            2 => {
                if do_niseci {
                    riferimento_path_str = arg;
                } else {
                    // Doing HFBI so we don't have a riferimento
                    anagrafica_path_str = arg;
                }
            }
            3 => {
                if do_niseci {
                    anagrafica_path_str = arg;
                }
            }
            _ => {
                eprintln!("Error: Unexpected arg: {arg}");
                esox_usage();
                return false;
            }
        }
    }

    let campionamento_path = PathBuf::from(campionamento_path_str);
    let riferimento_path = PathBuf::from(riferimento_path_str);
    let anagrafica_path = PathBuf::from(anagrafica_path_str);

    if !check_path_is_file_ends_with_csv(&campionamento_path) {
        eprintln!("Fallito controllo path campionamento");
        return false;
    }

    if do_niseci {

        if !check_path_is_file_ends_with_csv(&riferimento_path) {
            eprintln!("Fallito controllo path riferimento");
            return false;
        }
        let mut riferimento_csv_failed = false;
        let mut riferimento_valueparse_failed = false;
        let riferimento_csv_check_res = check_riferimento_niseci_path(riferimento_path);
        let mut riferimento_specie = Vec::new(); // Holds parsed SpecieNISECI
        match riferimento_csv_check_res {
            Ok(csv_recs) => {
                println!("Riferimento csv result: {{");
                for r in &csv_recs {
                    println!("  {r}");
                }
                println!("}}");
                let riferimento_records_check_res = check_records_riferimento_niseci(csv_recs);
                match riferimento_records_check_res {
                    Ok(recs_specie) => {
                        riferimento_specie = recs_specie;
                    }
                    Err(_value_errors) => {
                        /* Assuming they were printed before this point
                        eprintln!("Riferimento value errors in run_headless(): {{");
                        for e in value_errors {
                            let error_txt;
                            match e {
                                RecordCsvRiferimentoNISECIError::ValoreInvalido{ msg } => {
                                    error_txt = msg;
                                }
                            }
                            eprintln!("  {}", error_txt);
                        }
                        eprintln!("}}");
                        */
                        riferimento_valueparse_failed = true;
                        //return; We keep running to check the other file
                    }
                }
            }
            Err(_csv_errs) => {
                /* Assuming they were printed before this point
                eprintln!("Riferimento errors in run_headless(): {{");
                for e in errs {
                    eprintln!("  {e}");
                }
                eprintln!("}}");
                */
                riferimento_csv_failed = true;
                //return; We keep running to check the other file
            }
        }

        if !check_path_is_file_ends_with_csv(&campionamento_path) {
            eprintln!("Fallito controllo path campionamento");
            return false;
        }

        let mut campionamento_csv_failed = false;
        let mut campionamento_valueparse_failed = false;
        let campionamento_csv_check_res = check_campionamento_niseci_path(campionamento_path);
        let mut campionamento_specie = Vec::new(); // Holds parsed RecordNISECI
        match campionamento_csv_check_res {
            Ok(csv_recs) => {
                println!("Campionamento result: {{");
                for r in &csv_recs {
                    println!("  {r}");
                }
                println!("}}");
                let campionamento_records_check_res = check_records_campionamento_niseci(csv_recs, riferimento_specie.clone());
                match campionamento_records_check_res {
                    Ok(campioni) => {
                        campionamento_specie = campioni;
                    }
                    Err(_value_errors) => {
                        /* Assuming they were printed before this point
                        eprintln!("Campionamento value errors in run_headless(): {{");
                        for e in value_errors {
                            let error_txt;
                            match e {
                                RecordCsvCampionamentoNISECIError::ValoreInvalido{ msg } => {
                                    error_txt = msg;
                                }
                            }
                            eprintln!("  {}", error_txt);
                        }
                        eprintln!("}}");
                        */
                        campionamento_valueparse_failed = true;
                        //return; We keep running and return later
                    }
                }
            }
            Err(_errs) => {
                /* Assuming they were printed before this point
                eprintln!("Campionamento errors in run_headless(): {{");
                for e in errs {
                    eprintln!("  {e}");
                }
                eprintln!("}}");
                */
                campionamento_csv_failed = true;
                //return; We keep running and return later
            }
        }

        eprintln!("Check CSV riferimento:  {}", if riferimento_csv_failed { "FAIL" } else { "SUCCESS" });
        if !riferimento_csv_failed {
            eprintln!("Check valori riferimento:  {}", if riferimento_valueparse_failed { "FAIL" } else { "SUCCESS" });
        } else {
            eprintln!("Check valori riferimento:  SKIPPED (CSV check failed)");
        }
        eprintln!("Check CSV campionamento:  {}", if campionamento_csv_failed { "FAIL" } else { "SUCCESS" });
        if !campionamento_csv_failed {
            eprintln!("Check valori campionamento:  {}", if campionamento_valueparse_failed { "FAIL" } else { "SUCCESS" });
        } else {
            eprintln!("Check valori campionamento:  SKIPPED (CSV check failed)");
        }

        let had_failures = ( riferimento_csv_failed ||
            campionamento_csv_failed ) || (
            riferimento_valueparse_failed ||
            campionamento_valueparse_failed );


        let mut anagrafica_csv_failed = false;
        let mut anagrafica_valueparse_failed = false;
        let mut anagrafica = AnagraficaNISECI {
            comunita: ComunitaNISECI {
                tipo: TipoComunitaNISECI::Redatta,
                fonte: None,
                numero_protocollo: None,
            },
            codice_stazione: "foo".to_string(),
            date_string: "foo".to_string(),
            area: AreaNISECI::Alpina,
            corpo_idrico: "foo".to_string(),
            bacino_appartenenza: "foo".to_string(),
            idro_eco_regione: IdroEcoRegioneNISECI::Toscana,
            posizione: Location {
                regione: "foo".to_string(),
                provincia: "foo".to_string(),
            },
            lunghezza_media_stazione: 0.0,
            larghezza_media_stazione: 0.0,
        };
        if !had_failures {
            for s in &riferimento_specie {
                println!("Specie:  {:?}", s);
            }
            for c in &campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            let anagrafica_csv_check_res = check_anagrafica_niseci_path(anagrafica_path);
            match anagrafica_csv_check_res {
                Ok(csv_recs) => {
                    println!("Anagrafica result: {{");
                    for r in &csv_recs {
                        println!("  {r}");
                    }
                    println!("}}");
                    let anagrafica_records_check_res = check_records_anagrafica_niseci(csv_recs);
                    match anagrafica_records_check_res {
                        Ok(a) => {
                            anagrafica = a;
                        }
                        Err(_value_errs) => {
                            /* Assuming they were printed before this point
                            eprintln!("Anagrafica value errors in run_headless(): {{");
                            for e in value_errors {
                                let error_txt;
                                match e {
                                    RecordCsvAnagraficaNISECIError::ValoreInvalido{ msg } => {
                                        error_txt = msg;
                                    }
                                }
                                eprintln!("  {}", error_txt);
                            }
                            eprintln!("}}");
                            */
                            anagrafica_valueparse_failed = true;
                            //return; We keep running and return later
                    }
                        }
                    }
                    Err(_errs) => {
                        /* Assuming they were printed before this point
                        eprintln!("Anagrafica errors in run_headless(): {{");
                        for e in errs {
                            eprintln!("  {e}");
                        }
                        eprintln!("}}");
                        */
                        anagrafica_csv_failed = true;
                        //return; We keep running and return later
                    }
            }
        }

        let had_failures = had_failures ||
            ( anagrafica_csv_failed || anagrafica_valueparse_failed );

        let mut niseci_calc_failed = false;
        if !had_failures {
            let campionamento = CampionamentoNISECI {
                campionamento: campionamento_specie
            };
            let riferimento = RiferimentoNISECI {
                elenco_specie: riferimento_specie
            };
            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, _intermediates)) => {
                    let rqe_niseci = calculate_rqe_niseci(niseci);
                    let stato_eco_niseci = calculate_stato_ecologico(niseci, &anagrafica.area);
                    println!("NISECI: {niseci}");
                    println!("RQE NISECI: {rqe_niseci}");
                    println!("STATO ECOLOGICO NISECI: {stato_eco_niseci}");
                }
                Err(_errors) => {
                    /* Assuming they were printed before this point
                    for e in errs {
                        eprintln!("  {e}");
                    }
                    */
                    niseci_calc_failed = true;
                }
            }

        }
        let final_res = !had_failures && !niseci_calc_failed;
        return final_res;
    } else {
        let campionamento_check_res = check_campionamento_hfbi_path(campionamento_path);
        println!("Result: {campionamento_check_res}");
        return true;
    }
}
