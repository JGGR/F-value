use crate::core::*;

pub fn run_headless(do_niseci: bool, args: &Vec<String>) {
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
                //TODO: usage, might as well get clap then
                return;
            }
        }
    }

    let campionamento_path = PathBuf::from(campionamento_path_str);
    let riferimento_path = PathBuf::from(riferimento_path_str);

    if !campionamento_path.exists() {
        eprintln!("Error: Passed campionamento does not exist");
        return;
    } else if !campionamento_path.is_file() {
        eprintln!("Error: Passed campionamento is not a regular file");
        return;
    } else {
        let ext = campionamento_path.extension();
        match ext {
            Some(ex) => {
                if ! (ex == "csv") {
                    eprintln!("Error: Passed campionamento does not end with .csv");
                    return;
                }
            }
            None => {
                eprintln!("Error: Passed campionamento does not end with .csv");
                return;
            }
        }
    }
    if do_niseci {
        if !riferimento_path.exists() {
            eprintln!("Error: Passed riferimento does not exist");
            return;
        } else if !riferimento_path.is_file() {
            eprintln!("Error: Passed riferimento is not a regular file");
            return;
        } else {
            let ext = riferimento_path.extension();
            match ext {
                Some(ex) => {
                    if ! (ex == "csv") {
                        eprintln!("Error: Passed riferimento does not end with .csv");
                        return;
                    }
                }
                None => {
                    eprintln!("Error: Passed riferimento does not end with .csv");
                    return;
                }
            }
        }
        let campionamento_check_res = check_campionamento_niseci_path(campionamento_path);
        let riferimento_check_res = check_riferimento_niseci_path(riferimento_path);
        println!("Result: {campionamento_check_res}, {riferimento_check_res}");
    } else {
        let campionamento_check_res = check_campionamento_hfbi_path(campionamento_path);
        println!("Result: {campionamento_check_res}");
    }
}
