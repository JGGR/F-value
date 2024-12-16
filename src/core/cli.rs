use crate::core::*;

pub fn esox_usage() {
    println!("{PROJECT_NAME} v{SHORT_PROJECT_VERSION}");
    println!("Usage: {PROJECT_NAME} [--headless [--hfbi]] <campionamento.csv> <riferimento.csv>");
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
                    eprintln!("Ignoring riferimento since we're doing HFBI: {riferimento_path_str}");
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
        let riferimento_check_res = check_riferimento_niseci_path(riferimento_path);
        let mut riferimento_specie = Vec::new(); // Holds parsed SpecieNISECI
        match riferimento_check_res {
            Ok(recs) => {
                println!("Riferimento result: {{");
                for r in &recs {
                    println!("  {r}");
                }
                println!("}}");
                let (specie, value_errors) = parse_recordcsv_riferimento_niseci(recs);
                if !value_errors.is_empty() {
                    eprintln!("Riferimento value errors in run_headless(): {{");
                    for e in value_errors {
                        let mut error_txt = "".to_string();
                        match e {
                            RecordCsvRiferimentoNISECIError::ValoreInvalido{ msg } => {
                                error_txt = msg;
                            }
                        }
                        eprintln!("  {}", error_txt);
                    }
                    eprintln!("}}");
                    riferimento_valueparse_failed = true;
                    //return; We keep running to check the other file
                }
                riferimento_specie = specie;
            }
            Err(_errs) => {
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
        let campionamento_check_res = check_campionamento_niseci_path(campionamento_path);
        let mut campionamento_specie = Vec::new(); // Holds parsed RecordNISECI
        match campionamento_check_res {
            Ok(recs) => {
                println!("Campionamento result: {{");
                for r in &recs {
                    println!("  {r}");
                }
                println!("}}");
                let (campioni, value_errors) = parse_recordcsv_campionamento_niseci(recs, riferimento_specie.clone());
                if !value_errors.is_empty() {
                    eprintln!("Campionamento value errors in run_headless(): {{");
                    for e in value_errors {
                        let mut error_txt = "".to_string();
                        match e {
                            RecordCsvCampionamentoNISECIError::ValoreInvalido{ msg } => {
                                error_txt = msg;
                            }
                        }
                        eprintln!("  {}", error_txt);
                    }
                    eprintln!("}}");
                    campionamento_valueparse_failed = true;
                    //return; We keep running and return later
                }
                campionamento_specie = campioni;
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

        if !had_failures {
            for s in riferimento_specie {
                println!("Specie:  {:?}", s);
            }
            for c in campionamento_specie {
                println!("Campione:  {:?}", c);
            }
            println!("TODO:  Implement anagrafica loading, full calc");
        }
        return !had_failures;
    } else {
        let campionamento_check_res = check_campionamento_hfbi_path(campionamento_path);
        println!("Result: {campionamento_check_res}");
        return true;
    }
}
