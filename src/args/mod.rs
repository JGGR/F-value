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

use crate::app::core::SUPPORT_HEADLESS;
use crate::core::cli::{esox_usage, print_copyright_splash, print_warranty_info, run_headless};
use crate::core::csv::{
    ANAGRAFICA_HFBI_HEADER, ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_HFBI_HEADER_FIELD_TYPES,
    ANAGRAFICA_NISECI_HEADER, ANAGRAFICA_NISECI_HEADER_FIELDS,
    ANAGRAFICA_NISECI_HEADER_FIELD_TYPES, CAMPIONAMENTO_HFBI_HEADER,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES,
    CAMPIONAMENTO_NISECI_HEADER, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES, RIFERIMENTO_NISECI_HEADER,
    RIFERIMENTO_NISECI_HEADER_FIELDS, RIFERIMENTO_NISECI_HEADER_FIELD_TYPES,
};
use crate::core::{
    COMMIT_HASH_PLUS, PROJECT_BRANCH, PROJECT_BUILD_TYPE, PROJECT_NAME, PROJECT_VERSION,
};
use std::env;

pub(crate) fn handle_args() {
    let args: Vec<String> = env::args().collect(); // Using this panics on receiving invalid Unicode

    let mut mutargs = args.clone();

    let mut headless = false;

    let mut indice_niseci = true;

    let mut has_headers = true;

    match args.len() {
        1 => {}
        _ => {
            for arg in &args[1..] {
                match arg.as_str() {
                    "-v" | "--version" | "-version" => {
                        println!("{PROJECT_NAME} v{PROJECT_VERSION}-{COMMIT_HASH_PLUS} ({PROJECT_BUILD_TYPE})");
                        return;
                    }
                    "--info" => {
                        println!("Info: {{");
                        println!("  Versione: {PROJECT_VERSION}");
                        println!("  Build: {PROJECT_BUILD_TYPE}");
                        println!("  Branch: {PROJECT_BRANCH}");
                        println!("  Commit: {COMMIT_HASH_PLUS}");
                        println!("}}");
                        println!("Header riferimento NISECI: {{");
                        println!("  {RIFERIMENTO_NISECI_HEADER}");
                        println!("}}");
                        println!("Header campionamento NISECI: {{");
                        println!("  {CAMPIONAMENTO_NISECI_HEADER}");
                        println!("}}");
                        println!("Header anagrafica NISECI: {{");
                        println!("  {ANAGRAFICA_NISECI_HEADER}");
                        println!("}}");
                        println!("Tipi header riferimento NISECI: {{");
                        for (i, field) in RIFERIMENTO_NISECI_HEADER_FIELDS.iter().enumerate() {
                            println!(
                                "    {}: {};",
                                field, RIFERIMENTO_NISECI_HEADER_FIELD_TYPES[i]
                            );
                        }
                        println!("}}");
                        println!("Tipi header campionamento NISECI: {{");
                        for (i, field) in CAMPIONAMENTO_NISECI_HEADER_FIELDS.iter().enumerate() {
                            println!(
                                "    {}: {};",
                                field, CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES[i]
                            );
                        }
                        println!("}}");
                        println!("Tipi header anagrafica NISECI: {{");
                        for (i, field) in ANAGRAFICA_NISECI_HEADER_FIELDS.iter().enumerate() {
                            println!(
                                "    {}: {};",
                                field, ANAGRAFICA_NISECI_HEADER_FIELD_TYPES[i]
                            );
                        }
                        println!("}}");
                        println!("Header campionamento HFBI: {{");
                        println!("  {CAMPIONAMENTO_HFBI_HEADER}");
                        println!("}}");
                        println!("Header anagrafica HFBI: {{");
                        println!("  {ANAGRAFICA_HFBI_HEADER}");
                        println!("}}");
                        println!("Tipi header campionamento HFBI: {{");
                        for (i, field) in CAMPIONAMENTO_HFBI_HEADER_FIELDS.iter().enumerate() {
                            println!(
                                "    {}: {};",
                                field, CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES[i]
                            );
                        }
                        println!("}}");
                        println!("Tipi header anagrafica HFBI: {{");
                        for (i, field) in ANAGRAFICA_HFBI_HEADER_FIELDS.iter().enumerate() {
                            println!("    {}: {};", field, ANAGRAFICA_HFBI_HEADER_FIELD_TYPES[i]);
                        }
                        println!("}}");
                        return;
                    }
                    "-h" | "-help" | "--help" => {
                        return esox_usage();
                    }
                    "--headless" => {
                        if !SUPPORT_HEADLESS {
                            eprintln!("Headless run is not supported.");
                            return;
                        }
                        headless = true;
                        mutargs.remove(1);
                    }
                    "--hfbi" => {
                        indice_niseci = false;
                        mutargs.remove(1);
                    }
                    "--no-headers" => {
                        has_headers = false;
                        mutargs.remove(1);
                    }
                    "-W" | "--warranty" | "-warranty" => {
                        return print_warranty_info();
                    }
                    _ => {
                        if arg.starts_with("--") {
                            eprintln!("Unknown flag: {arg}");
                            return esox_usage();
                        }
                    }
                }
            }
        }
    }

    print_copyright_splash();

    if headless {
        let res = run_headless(indice_niseci, has_headers, &mutargs);

        if !res {
            eprintln!("Headless run failed");
        }
    }
}
