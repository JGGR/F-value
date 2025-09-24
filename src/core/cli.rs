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

use crate::core::csv::deser::{
    check_path_is_file_ends_with_csv,
    hfbi::{
        check_anagrafica_hfbi_path, check_campionamento_hfbi_path,
        VeryItalianRecordCsvAnagraficaHFBI, VeryItalianRecordCsvCampionamentoHFBI,
    },
    niseci::{
        check_anagrafica_niseci_path, check_campionamento_niseci_path,
        check_riferimento_niseci_path, VeryItalianRecordCsvAnagraficaNISECI,
        VeryItalianRecordCsvCampionamentoNISECI, VeryItalianRecordCsvRiferimentoNISECI,
    },
};
use crate::core::csv::parser::{
    check_records_anagrafica_hfbi, check_records_anagrafica_niseci,
    check_records_campionamento_hfbi, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use crate::core::pdf::{esporta_pdf_hfbi, esporta_pdf_niseci};
use crate::core::{COPYRIGHT_INFO, PROJECT_NAME, PROJECT_VERSION_FULL, SHORT_PROJECT_VERSION};
use crate::domain::hfbi::{
    AnagraficaHFBI, CampionamentoHFBI, HabitatHFBI, RisultatoHFBI, StagioneHFBI,
    TipoLagunaCostieraHFBI,
};
use crate::domain::location::Location;
use crate::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, ComunitaNISECI, IdroEcoRegioneNISECI,
    RiferimentoNISECI, RisultatoNISECI, TipoComunitaNISECI,
};
use crate::engines::hfbi::full::calculate_hfbi;
use crate::engines::niseci::full::{
    calculate_niseci, calculate_rqe_niseci, calculate_stato_ecologico,
};
use std::path::PathBuf;

pub(crate) fn esox_usage() {
    println!("{PROJECT_NAME} v{SHORT_PROJECT_VERSION}");
    println!("Usage: {PROJECT_NAME} [--headless] <campionamento.csv> <riferimento.csv> <anagrafica.csv> [pdf_export_path]");
    println!("       {PROJECT_NAME} [--headless] --hfbi <campionamento.csv> <anagrafica.csv> [pdf_export_path]");
    println!(
        "Flags:
  --headless               Run without GUI
  --hfbi                   Run with HFBI
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

pub(crate) fn run_headless(do_niseci: bool, args: &[String]) -> bool {
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
                    esox_usage();
                    return false;
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
        let riferimento_csv_check_res = check_riferimento_niseci_path::<
            VeryItalianRecordCsvRiferimentoNISECI,
        >(riferimento_path);
        let mut riferimento_specie = Vec::new(); // Holds parsed SpecieNISECI
        match riferimento_csv_check_res {
            Ok(csv_recs) => {
                /* TODO: handle verbosity
                println!("Riferimento csv result: {{");
                for r in &csv_recs {
                    println!("  {r}");
                }
                println!("}}");
                */
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
        // Using italian deser for now
        let campionamento_csv_check_res = check_campionamento_niseci_path::<
            VeryItalianRecordCsvCampionamentoNISECI,
        >(campionamento_path);
        let mut campionamento_specie = Vec::new(); // Holds parsed RecordNISECI
        match campionamento_csv_check_res {
            Ok(csv_recs) => {
                /* TODO: handle verbosity
                println!("Campionamento result: {{");
                for r in &csv_recs {
                    println!("  {r}");
                }
                println!("}}");
                */
                let campionamento_records_check_res =
                    check_records_campionamento_niseci(csv_recs, riferimento_specie.clone());
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
            /* TODO: handle verbosity
            for s in &riferimento_specie {
                println!("Specie:  {:?}", s);
            }
            for c in &campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            */
            // Using italian deser for now
            let anagrafica_csv_check_res = check_anagrafica_niseci_path::<
                VeryItalianRecordCsvAnagraficaNISECI,
            >(anagrafica_path);
            match anagrafica_csv_check_res {
                Ok(csv_recs) => {
                    /* TODO: handle verbosity
                    println!("Anagrafica result: {{");
                    for r in &csv_recs {
                        println!("  {r}");
                    }
                    println!("}}");
                    */
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

        let had_failures = had_failures || (anagrafica_csv_failed || anagrafica_valueparse_failed);

        let mut niseci_calc_failed = false;
        if !had_failures {
            let campionamento = CampionamentoNISECI {
                campionamento: campionamento_specie,
            };
            let riferimento = RiferimentoNISECI {
                elenco_specie: riferimento_specie,
            };
            match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
                Ok((niseci, intermediates)) => {
                    let rqe_niseci = calculate_rqe_niseci(niseci);
                    let stato_eco_niseci = calculate_stato_ecologico(niseci, &anagrafica.area);

                    intermediates.log();
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
        let campionamento_check_res = check_campionamento_hfbi_path::<
            VeryItalianRecordCsvCampionamentoHFBI,
        >(campionamento_path);
        let mut campionamento_specie = Vec::new(); // Holds parsed RecordHFBI
        match campionamento_check_res {
            Ok(csv_recs) => {
                /* TODO: handle verbosity
                println!("Campionamento result: {{");
                for r in &csv_recs {
                    println!("  {r}");
                }
                println!("}}");
                */
                let campionamento_records_check_res = check_records_campionamento_hfbi(csv_recs);
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
                                RecordCsvCampionamentoHFBIError::ValoreInvalido{ msg } => {
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
        let mut anagrafica = AnagraficaHFBI {
            codice_stazione: "foo".to_string(),
            date_string: "foo".to_string(),
            corpo_idrico: "foo".to_string(),
            posizione: Location {
                regione: "foo".to_string(),
                provincia: "foo".to_string(),
            },
            lunghezza_media_transetto: 0.0,
            larghezza_media_transetto: 0.0,
            stagione: StagioneHFBI::Primavera,
            habitat_vegetato: HabitatHFBI::Vegetato,
            tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
        };
        if !had_failures {
            /* TODO: handle verbosity
            for c in &campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            */
            // Using italian deser for now
            let anagrafica_csv_check_res =
                check_anagrafica_hfbi_path::<VeryItalianRecordCsvAnagraficaHFBI>(anagrafica_path);
            match anagrafica_csv_check_res {
                Ok(csv_recs) => {
                    /* TODO: handle verbosity
                    println!("Anagrafica result: {{");
                    for r in &csv_recs {
                        println!("  {r}");
                    }
                    println!("}}");
                    */
                    let anagrafica_records_check_res = check_records_anagrafica_hfbi(csv_recs);
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
                                    RecordCsvAnagraficaHFBIError::ValoreInvalido{ msg } => {
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

        let had_failures = had_failures || (anagrafica_csv_failed || anagrafica_valueparse_failed);

        let mut hfbi_calc_failed = false;
        if !had_failures {
            let campionamento = CampionamentoHFBI {
                campionamento: campionamento_specie,
            };
            match calculate_hfbi(&campionamento, &anagrafica) {
                Ok((hfbi, intermediates)) => {
                    intermediates.log();
                    let risultato_hfbi = RisultatoHFBI::new(Some(hfbi), intermediates);

                    println!("HFBI: {hfbi}");

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
