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

use crate::app::core::SUPPORT_HEADLESS;
use crate::core::cli::{f_value_usage, print_copyright_splash, print_warranty_info, run_headless};
use crate::core::{
    COMMIT_HASH_PLUS, PROJECT_BRANCH, PROJECT_BUILD_TYPE, PROJECT_NAME, PROJECT_VERSION,
};
use clap::{CommandFactory, Parser};
use esox::csv::{
    ANAGRAFICA_HFBI_HEADER, ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_HFBI_HEADER_FIELD_TYPES,
    ANAGRAFICA_NISECI_HEADER, ANAGRAFICA_NISECI_HEADER_FIELDS,
    ANAGRAFICA_NISECI_HEADER_FIELD_TYPES, CAMPIONAMENTO_HFBI_HEADER,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES,
    CAMPIONAMENTO_NISECI_HEADER, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES, RIFERIMENTO_NISECI_HEADER,
    RIFERIMENTO_NISECI_HEADER_FIELDS, RIFERIMENTO_NISECI_HEADER_FIELD_TYPES,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = PROJECT_NAME,
    disable_help_flag = true,
)]
struct Args {
    /// Print version information.
    #[arg(short = 'v', long = "version", visible_alias = "version")]
    version: bool,
    /// Print usage help.
    #[arg(short = 'h', long = "help")]
    help: bool,
    /// Print detailed build and header information.
    #[arg(long)]
    info: bool,
    /// Run without the graphical interface.
    #[arg(long)]
    headless: bool,
    /// Use HFBI instead of NISECI.
    #[arg(long)]
    hfbi: bool,
    /// Input does not contain headers.
    #[arg(long = "no-headers")]
    no_headers: bool,
    /// Print warranty information.
    #[arg(short = 'W', long = "warranty", visible_alias = "warranty")]
    warranty: bool,

    /// Campionamento CSV input.
    #[arg(
        long,
        value_name = "PATH",
        required_unless_present = "info",
        required_unless_present = "help",
        required_unless_present = "version",
        conflicts_with = "info",
        conflicts_with = "help",
        conflicts_with = "version"
    )]
    campionamento: Option<PathBuf>,

    /// Riferimento CSV input (NISECI only).
    #[arg(
        long,
        value_name = "PATH",
        required_unless_present = "hfbi",
        required_unless_present = "info",
        required_unless_present = "help",
        required_unless_present = "version",
        conflicts_with = "hfbi",
        conflicts_with = "info",
        conflicts_with = "help",
        conflicts_with = "version"
    )]
    riferimento: Option<PathBuf>,

    /// Anagrafica CSV input.
    #[arg(
        long,
        value_name = "PATH",
        required_unless_present = "info",
        required_unless_present = "help",
        required_unless_present = "version",
        conflicts_with = "info",
        conflicts_with = "help",
        conflicts_with = "version",
    )]
    anagrafica: Option<PathBuf>,

    /// Optional PDF export path.
    #[arg(long, value_name = "PATH")]
    pdf_export_path: Option<PathBuf>,
}

pub(crate) fn print_info() {
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
}

pub(crate) fn handle_args() {
    let args = Args::parse();

    if args.version {
        println!("{PROJECT_NAME} v{PROJECT_VERSION}-{COMMIT_HASH_PLUS} ({PROJECT_BUILD_TYPE})");
        return;
    }

    if args.help {
        f_value_usage();
        Args::command().print_help().unwrap();
        return;
    }

    if args.info {
        print_info();
        return;
    }

    if args.warranty {
        print_warranty_info();
        return;
    }

    if args.headless && !SUPPORT_HEADLESS {
        eprintln!("Headless run is not supported.");
        return;
    }

    print_copyright_splash();

    if args.headless {
        let res = run_headless(
            !args.hfbi,
            !args.no_headers,
            args.campionamento
                .expect("Campionamento path is required outside of info mode"),
            args.riferimento,
            args.anagrafica
                .expect("Anagrafica path is required outside of info mode"),
            args.pdf_export_path,
        );

        if !res {
            eprintln!("Headless run failed");
        }
    }
}
