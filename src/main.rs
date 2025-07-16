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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod state;
mod views;
mod controllers;
mod console;
mod core;
mod engines;
mod app;
#[cfg(test)]
mod tests;

use std::env;
use raylib::consts::TraceLogLevel;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
use raylib::core::texture::Image;
use raylib::color::Color;
use crate::app::core::{SUPPORT_HEADLESS, PROJECT_LOGO_DATA, ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT, MainState};
use crate::core::{PROJECT_NAME, SHORT_PROJECT_VERSION, PROJECT_VERSION, COMMIT_HASH_PLUS, PROJECT_BUILD_TYPE, PROJECT_BRANCH};
use crate::core::csv::{RIFERIMENTO_NISECI_HEADER, RIFERIMENTO_NISECI_HEADER_FIELDS, RIFERIMENTO_NISECI_HEADER_FIELD_TYPES, CAMPIONAMENTO_NISECI_HEADER, CAMPIONAMENTO_NISECI_HEADER_FIELDS, CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES, ANAGRAFICA_NISECI_HEADER, ANAGRAFICA_NISECI_HEADER_FIELDS, ANAGRAFICA_NISECI_HEADER_FIELD_TYPES, CAMPIONAMENTO_HFBI_HEADER, CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES, ANAGRAFICA_HFBI_HEADER, ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_HFBI_HEADER_FIELD_TYPES};
use crate::core::cli::{esox_usage, print_warranty_info, print_copyright_splash, run_headless};
use crate::controllers::Controllers;
use crate::views::Views;

#[cfg(feature="logged")]
use crate::core::prep_logger;

fn main() {

    #[cfg(feature="logged")]
    let _ = prep_logger();

    let args: Vec<String> = env::args().collect(); // Using this panics on receiving invalid Unicode

    let mut mutargs = args.clone();
    let mut arg_i = 0;

    let mut headless = false;

    let mut indice_niseci = true;

    match args.len() {
        1 => {},
        _ => {
            for arg in &args[1..] {
                arg_i += 1;
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
                            println!("    {}: {};", field, RIFERIMENTO_NISECI_HEADER_FIELD_TYPES[i]);
                        }
                        println!("}}");
                        println!("Tipi header campionamento NISECI: {{");
                        for (i, field) in CAMPIONAMENTO_NISECI_HEADER_FIELDS.iter().enumerate() {
                            println!("    {}: {};", field, CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES[i]);
                        }
                        println!("}}");
                        println!("Tipi header anagrafica NISECI: {{");
                        for (i, field) in ANAGRAFICA_NISECI_HEADER_FIELDS.iter().enumerate() {
                            println!("    {}: {};", field, ANAGRAFICA_NISECI_HEADER_FIELD_TYPES[i]);
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
                            println!("    {}: {};", field, CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES[i]);
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
                        if ! SUPPORT_HEADLESS {
                            eprintln!("Headless run is not supported.");
                            return;
                        }
                        headless = true;
                        mutargs.remove(arg_i);
                    }
                    "--hfbi" => {
                        indice_niseci = false;
                        mutargs.remove(arg_i);
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
        },
    }


    print_copyright_splash();

    if headless {
        let res = run_headless(indice_niseci, &mutargs);

        if !res {
            eprintln!("Headless run failed");
        }
        return;
    }

    let img_load_res = Image::load_image_from_mem(".png", PROJECT_LOGO_DATA);

    let mut logo_img = None;
    match img_load_res {
        Ok(img) => {
            logo_img = Some(img);
        }
        Err(err) => {
            println!("Error loading logo img: {err}");
        }
    }

    let window_title = format!("F-value v{SHORT_PROJECT_VERSION}");

    let (mut rl, thread) = raylib::init()
        .size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT)
        .title(&window_title)
        .log_level(TraceLogLevel::LOG_ERROR) // Gets rid of raylib init text in the terminal
        .resizable()
        .build();

    rl.set_window_min_size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT);
    rl.set_exit_key(None); // This allows capturing the exit key with a message box
    rl.set_target_fps(30);

    let mut logo_texture = None;
    if let Some(img) = logo_img {
        logo_texture = Some(rl.load_texture_from_image(&thread, &img).unwrap());
    }

    // 10 is way too small for the default font height
    let gui_default_font_height: i32 = rl.gui_get_style(DEFAULT, TEXT_SIZE) *2;
    rl.gui_set_style(DEFAULT, TEXT_SIZE, gui_default_font_height);
    let gui_current_font_height: i32 = gui_default_font_height;

    let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL);
    let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR);
    let txt_spacing = rl.gui_get_style(DEFAULT, TEXT_SPACING) *2;
    rl.gui_set_style(DEFAULT, TEXT_SPACING, txt_spacing);
    let current_font = rl.gui_get_font();
    let mut main_state = MainState::new(
        gui_default_font_height,
        gui_current_font_height,
        txt_spacing,
        current_font,
        Color::get_color(txt_color_int as u32),
        Color::get_color(bg_color_int as u32),
        logo_texture
    );

    let controllers = Controllers::new();

    let mut views = Views::new(&mut rl, &thread, gui_current_font_height, txt_spacing);

    main_state.mainloop(&mut rl, &thread, &controllers, &mut views);

}
