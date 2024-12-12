use crate::core::*;

pub fn esox_usage() {
    println!("{PROJECT_NAME} v{SHORT_PROJECT_VERSION}");
    println!("Usage: {PROJECT_NAME} [--headless [--hfbi]] <campionamento.csv> <riferimento.csv>");
    println!(
        "Flags:
  --headless               Run without GUI
  --hfbi                   Run with HFBI
  --version, -v            Print version and quit
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

    if !campionamento_path.exists() {
        eprintln!("Error: Passed campionamento does not exist");
        return false;
    } else if !campionamento_path.is_file() {
        eprintln!("Error: Passed campionamento is not a regular file");
        return false;
    } else {
        let ext = campionamento_path.extension();
        match ext {
            Some(ex) => {
                if ! (ex == "csv") {
                    eprintln!("Error: Passed campionamento does not end with .csv");
                    return false;
                }
            }
            None => {
                eprintln!("Error: Passed campionamento does not end with .csv");
                return false;
            }
        }
    }
    if do_niseci {
        if !riferimento_path.exists() {
            eprintln!("Error: Passed riferimento does not exist");
            return false;
        } else if !riferimento_path.is_file() {
            eprintln!("Error: Passed riferimento is not a regular file");
            return false;
        } else {
            let ext = riferimento_path.extension();
            match ext {
                Some(ex) => {
                    if ! (ex == "csv") {
                        eprintln!("Error: Passed riferimento does not end with .csv");
                        return false;
                    }
                }
                None => {
                    eprintln!("Error: Passed riferimento does not end with .csv");
                    return false;
                }
            }
        }
        let mut campionamento_csv_failed = false;
        let campionamento_check_res = check_campionamento_niseci_path(campionamento_path);
        match campionamento_check_res {
            Ok(recs) => {
                println!("Campionamento result:");
                for r in recs {
                    println!("{r}");
                }
                println!("TODO:  Implement validation step after successful csv parsing");
                println!("TODO: We may skip the riferimento check also in that case");
            }
            Err(errs) => {
                eprintln!("Campionamento errors in run_headless(): {{");
                for e in errs {
                    eprintln!("  {e}");
                }
                eprintln!("}}");
                campionamento_csv_failed = true;
                //return; We keep running to check the other file
            }
        }
        let mut riferimento_csv_failed = false;
        let riferimento_check_res = check_riferimento_niseci_path(riferimento_path);
        match riferimento_check_res {
            Ok(recs) => {
                println!("Riferimento result:");
                for r in recs {
                    println!("{r}");
                }
                println!("TODO:  Implement validation step after successful csv parsing");
            }
            Err(errs) => {
                eprintln!("Riferimento errors in run_headless(): {{");
                for e in errs {
                    eprintln!("  {e}");
                }
                eprintln!("}}");
                riferimento_csv_failed = true;
                //return; We keep running and return later
            }
        }

        return !riferimento_csv_failed && !campionamento_csv_failed;
    } else {
        let campionamento_check_res = check_campionamento_hfbi_path(campionamento_path);
        println!("Result: {campionamento_check_res}");
        return true;
    }
}
