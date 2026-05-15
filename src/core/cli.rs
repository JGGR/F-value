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

use crate::core::pdf::{esporta_pdf_hfbi, esporta_pdf_niseci};
use crate::core::{COPYRIGHT_INFO, PROJECT_NAME, PROJECT_VERSION_FULL, SHORT_PROJECT_VERSION};
use esox::csv::deser::check_path_is_file_ends_with_csv;
use esox::csv::load::hfbi::{
    load_anagrafica_hfbi_from_path, load_campionamento_hfbi_from_path, AnagraficaHFBIError,
    CampionamentoHFBIError,
};
use esox::csv::load::niseci::{
    load_anagrafica_niseci_from_path, load_campionamento_niseci_from_path,
    load_riferimento_niseci_from_path, AnagraficaNISECIError, CampionamentoNISECIError,
    RiferimentoNISECIError,
};
use esox::csv::load::InputFormat;
use esox::domain::hfbi::{
    AnagraficaHFBI, CampionamentoHFBI, HabitatHFBI, RisultatoHFBI, StagioneHFBI,
    TipoLagunaCostieraHFBI,
};
use esox::domain::location::Location;
use esox::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI, RiferimentoNISECI,
    TipoComunitaNISECI,
};

#[cfg(not(feature = "lessclone"))]
use esox::{
    domain::niseci::{CampionamentoNISECI, RisultatoNISECI},
    engines::niseci::full::{
        calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico_niseci,
    },
};

#[cfg(feature = "lessclone")]
use esox::{
    domain::niseci::lessclone::{CampionamentoNISECI, RisultatoNISECI},
    engines::niseci::full::lessclone::{
        calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico_niseci,
    },
};

use esox::domain::posf32::PositiveF32;
use esox::engines::hfbi::full::{calculate_hfbi, calculate_stato_ecologico_hfbi};
use std::path::PathBuf;

pub(crate) fn f_value_usage() {
    println!("{PROJECT_NAME} v{SHORT_PROJECT_VERSION}");
    println!("Usage: {PROJECT_NAME} [--headless] <campionamento.csv> <riferimento.csv> <anagrafica.csv> [pdf_export_path]");
    println!("       {PROJECT_NAME} [--headless] --hfbi <campionamento.csv> <anagrafica.csv> [pdf_export_path]");
    println!(
        "Flags:
  --headless               Run without GUI
  --hfbi                   Run with HFBI
  --no-headers             Expect no headers in input files
  --version, -v            Print version and quit
  --info                   Print debug info and quit
  --help, -h               Print this message and quit"
    );
}

pub(crate) fn print_warranty_info() {
    println!(
        "  THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY
  APPLICABLE LAW.  EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT
  HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM \"AS IS\" WITHOUT WARRANTY
  OF ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO,
  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
  PURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM
  IS WITH YOU.  SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF
  ALL NECESSARY SERVICING, REPAIR OR CORRECTION.\n"
    );
}

pub(crate) fn print_copyright_splash() {
    let splash: String = format!("{PROJECT_VERSION_FULL}\n\n{COPYRIGHT_INFO}");
    println!("{splash}\n");
}

pub(crate) fn run_headless(do_niseci: bool, has_headers: bool, args: &[String]) -> bool {
    let mut arg_i = 0;
    let mut campionamento_path_str = "";
    let mut riferimento_path_str = "";
    let mut anagrafica_path_str = "";
    let mut pdf_export_path_str = ".";
    let mut passed_pdf_export_path = false;
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
                } else {
                    // Doing HFBI so we don't have a riferimento
                    pdf_export_path_str = arg;
                    passed_pdf_export_path = true;
                }
            }
            4 => {
                if do_niseci {
                    pdf_export_path_str = arg;
                    passed_pdf_export_path = true;
                } else {
                    eprintln!("Error: Unexpected arg: {arg}");
                    f_value_usage();
                    return false;
                }
            }
            _ => {
                eprintln!("Error: Unexpected arg: {arg}");
                f_value_usage();
                return false;
            }
        }
    }

    let campionamento_path = PathBuf::from(campionamento_path_str);
    let riferimento_path = PathBuf::from(riferimento_path_str);
    let anagrafica_path = PathBuf::from(anagrafica_path_str);
    let pdf_export_path = PathBuf::from(pdf_export_path_str);

    if !check_path_is_file_ends_with_csv(&campionamento_path) {
        eprintln!(
            "Fallito controllo path campionamento: {}",
            campionamento_path.display()
        );
        return false;
    }

    if do_niseci {
        if !check_path_is_file_ends_with_csv(&riferimento_path) {
            eprintln!(
                "Fallito controllo path riferimento: {}",
                riferimento_path.display()
            );
            return false;
        }
        let mut riferimento_csv_failed = false;
        let mut riferimento_valueparse_failed = false;
        // Using italian deser for now
        let riferimento_load_res = load_riferimento_niseci_from_path(
            riferimento_path,
            has_headers,
            InputFormat::Alternative,
        );
        let mut riferimento = RiferimentoNISECI::new(vec![]);
        match riferimento_load_res {
            Ok(recs_specie) => {
                riferimento = recs_specie;
            }
            Err(ev) => {
                // Assuming they were printed before this point
                match ev {
                    RiferimentoNISECIError::Csv(_errors) => {
                        riferimento_csv_failed = true;
                    }
                    RiferimentoNISECIError::Value(_value_errors) => {
                        /*
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
                    }
                }
                //return; We keep running to check the other file
            }
        }

        if !check_path_is_file_ends_with_csv(&campionamento_path) {
            eprintln!("Fallito controllo path campionamento");
            return false;
        }

        let mut campionamento_csv_failed = false;
        let mut campionamento_valueparse_failed = false;
        // Using italian deser for now
        let campionamento_load_res = load_campionamento_niseci_from_path(
            campionamento_path,
            has_headers,
            &riferimento,
            InputFormat::Alternative,
        );
        let mut campionamento = CampionamentoNISECI::new(vec![]);
        match campionamento_load_res {
            Ok(campioni) => {
                campionamento = campioni;
            }
            Err(ev) => {
                // Assuming they were printed before this point
                match ev {
                    CampionamentoNISECIError::Csv(_errors) => {
                        campionamento_csv_failed = true;
                    }
                    CampionamentoNISECIError::Value(_errors) => {
                        /*
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
                    }
                }
                //return; We keep running and return later
            }
        }

        eprintln!(
            "Check CSV riferimento:  {}",
            if riferimento_csv_failed {
                "FAIL"
            } else {
                "SUCCESS"
            }
        );
        if !riferimento_csv_failed {
            eprintln!(
                "Check valori riferimento:  {}",
                if riferimento_valueparse_failed {
                    "FAIL"
                } else {
                    "SUCCESS"
                }
            );
        } else {
            eprintln!("Check valori riferimento:  SKIPPED (CSV check failed)");
        }
        eprintln!(
            "Check CSV campionamento:  {}",
            if campionamento_csv_failed {
                "FAIL"
            } else {
                "SUCCESS"
            }
        );
        if !campionamento_csv_failed {
            eprintln!(
                "Check valori campionamento:  {}",
                if campionamento_valueparse_failed {
                    "FAIL"
                } else {
                    "SUCCESS"
                }
            );
        } else {
            eprintln!("Check valori campionamento:  SKIPPED (CSV check failed)");
        }

        let had_failures = (riferimento_csv_failed || campionamento_csv_failed)
            || (riferimento_valueparse_failed || campionamento_valueparse_failed);

        let mut anagrafica_failed = false;
        let mut anagrafica = AnagraficaNISECI::new(
            ComunitaNISECI::new(TipoComunitaNISECI::Redatta, None, None),
            "foo".to_string(),
            "foo".to_string(),
            AreaNISECI::Alpina,
            "foo".to_string(),
            "foo".to_string(),
            IdroEcoRegioneNISECI::Toscana,
            Location {
                regione: "foo".to_string(),
                provincia: "foo".to_string(),
            },
            PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32"),
            PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32"),
        );
        if !had_failures {
            /* TODO: handle verbosity
            for s in &riferimento_specie {
                println!("Specie:  {:?}", s);
            }
            for c in &campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            */
            // Using italian deser for now
            let anagrafica_load_res = load_anagrafica_niseci_from_path(
                anagrafica_path,
                has_headers,
                InputFormat::Alternative,
            );
            match anagrafica_load_res {
                Ok(a) => {
                    anagrafica = a;
                }
                Err(ev) => {
                    // Assuming they were printed before this point
                    match ev {
                        AnagraficaNISECIError::Csv(_errors) => {}
                        AnagraficaNISECIError::Value(_value_errors) => {
                            /*
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
                        }
                    }
                    anagrafica_failed = true;
                }
            }
        }

        let had_failures = had_failures || anagrafica_failed;

        let mut niseci_calc_failed = false;
        if !had_failures {
            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, intermediates)) => {
                    let rqe_niseci = calculate_rqe_niseci(niseci);
                    let stato_eco_niseci =
                        calculate_stato_ecologico_niseci(niseci, &anagrafica.area);

                    println!("{}", intermediates);
                    match niseci {
                        Some(val) => {
                            println!("NISECI: {val}");
                        }
                        None => {
                            println!("NISECI: NC");
                        }
                    }
                    match rqe_niseci {
                        Some(val) => {
                            println!("RQE NISECI: {val}");
                        }
                        None => {
                            println!("RQE NISECI: NC");
                        }
                    }
                    match stato_eco_niseci {
                        Some(val) => {
                            println!("STATO ECOLOGICO NISECI: {val}");
                        }
                        None => {
                            println!("STATO ECOLOGICO NISECI: NC");
                        }
                    }

                    let risultato_niseci = RisultatoNISECI::new(niseci, rqe_niseci, intermediates);

                    if passed_pdf_export_path {
                        println!("Esportato pdf in {}", pdf_export_path.display());
                        esporta_pdf_niseci(
                            pdf_export_path,
                            riferimento,
                            anagrafica,
                            risultato_niseci,
                        );
                    }
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
        !had_failures && !niseci_calc_failed
    } else {
        let mut campionamento_csv_failed = false;
        let mut campionamento_valueparse_failed = false;
        let campionamento_load_res = load_campionamento_hfbi_from_path(
            campionamento_path,
            has_headers,
            InputFormat::Alternative,
        );
        let mut campionamento = CampionamentoHFBI::new(vec![]);
        match campionamento_load_res {
            Ok(campioni) => {
                campionamento = campioni;
            }
            Err(ev) => {
                match ev {
                    CampionamentoHFBIError::Csv(_errors) => {
                        campionamento_csv_failed = true;
                    }
                    CampionamentoHFBIError::Value(_errors) => {
                        /*
                        eprintln!("Campionamento value errors in run_headless(): {{");
                        for e in value_errors {
                            let error_txt;
                            match e {
                                RecordCsvCampionamentoHFBIError::ValoreInvalido{ msg } => {
                                    error_txt = msg;
                                }
                            }
                            eprintln!("  {}", error_txt);
                        }
                        eprintln!("}}");
                        */
                        campionamento_valueparse_failed = true;
                    }
                }
                //return; We keep running and return later
            }
        }

        eprintln!(
            "Check CSV campionamento:  {}",
            if campionamento_csv_failed {
                "FAIL"
            } else {
                "SUCCESS"
            }
        );
        if !campionamento_csv_failed {
            eprintln!(
                "Check valori campionamento:  {}",
                if campionamento_valueparse_failed {
                    "FAIL"
                } else {
                    "SUCCESS"
                }
            );
        } else {
            eprintln!("Check valori campionamento:  SKIPPED (CSV check failed)");
        }

        let had_failures = (campionamento_csv_failed) || (campionamento_valueparse_failed);

        let mut anagrafica_csv_failed = false;
        let mut anagrafica_valueparse_failed = false;
        let mut anagrafica = AnagraficaHFBI::new(
            "foo".to_string(),
            "foo".to_string(),
            Location {
                regione: "foo".to_string(),
                provincia: "foo".to_string(),
            },
            "foo".to_string(),
            TipoLagunaCostieraHFBI::MAt1,
            StagioneHFBI::Primavera,
            HabitatHFBI::Vegetato,
            PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32"),
            PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32"),
        );
        if !had_failures {
            /* TODO: handle verbosity
            for c in &campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            */
            // Using italian deser for now
            let anagrafica_load_res = load_anagrafica_hfbi_from_path(
                anagrafica_path,
                has_headers,
                InputFormat::Alternative,
            );
            match anagrafica_load_res {
                Ok(a) => {
                    anagrafica = a;
                }
                Err(ev) => {
                    match ev {
                        AnagraficaHFBIError::Csv(_errors) => {
                            anagrafica_csv_failed = true;
                        }
                        AnagraficaHFBIError::Value(_errors) => {
                            /*
                            eprintln!("Anagrafica value errors in run_headless(): {{");
                            for e in value_errors {
                                let error_txt;
                                match e {
                                    RecordCsvAnagraficaHFBIError::ValoreInvalido{ msg } => {
                                        error_txt = msg;
                                    }
                                }
                                eprintln!("  {}", error_txt);
                            }
                            eprintln!("}}");
                            */
                            anagrafica_valueparse_failed = true;
                        }
                    }
                    //return; We keep running and return later
                }
            }
        }

        let had_failures = had_failures || (anagrafica_csv_failed || anagrafica_valueparse_failed);

        let mut hfbi_calc_failed = false;
        if !had_failures {
            match calculate_hfbi(&campionamento, &anagrafica) {
                Ok((hfbi, intermediates)) => {
                    intermediates.log();
                    let risultato_hfbi = RisultatoHFBI::new(Some(hfbi), intermediates);

                    println!("HFBI: {hfbi}");

                    let stato_ecologico = calculate_stato_ecologico_hfbi(Some(hfbi));
                    let stato_ecologico_str = match stato_ecologico {
                        Some(val) => {
                            format!("{val}")
                        }
                        None => "NC".to_string(),
                    };
                    println!("Stato ecologico: {stato_ecologico_str}");

                    if passed_pdf_export_path {
                        println!("Esportato pdf in {}", pdf_export_path.display());
                        esporta_pdf_hfbi(pdf_export_path, anagrafica, risultato_hfbi);
                    }
                }
                Err(_errors) => {
                    /* Assuming they were printed before this point
                    for e in errs {
                        eprintln!("  {e}");
                    }
                    */
                    hfbi_calc_failed = true;
                }
            }
        }
        !had_failures && !hfbi_calc_failed
    }
}
