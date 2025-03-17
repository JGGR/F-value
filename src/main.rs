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

mod state;
mod model;
mod views;
mod controllers;
mod console;
mod core;
mod engines;
#[cfg(test)]
mod tests;

use crate::core::*;
use crate::core::view::*;
use crate::core::controller::*;
use crate::core::cli::*;
use crate::controllers::*;
use crate::views::*;
use raylib::prelude::*;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::{BACKGROUND_COLOR, TEXT_SIZE, TEXT_SPACING};
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;

use std::env;

fn main() {

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

    let home_controller = HomeController::new();
    let mut home_view = HomeView::new();
    let second_controller = SecondController::new();
    let mut second_view = SecondView::new();
    let indice_controller = IndiceController::new();
    let mut selezione_indice_view = SelezioneIndiceView::new();
    let fileinput_controller = FileInputController::new();
    let mut selezione_fileinput_view = SelezioneFileInputView::new();
    let mut validazione_fileinput_view = ValidazioneFileInputView::new();
    let infoaggiuntive_controller = InfoAggiuntiveController::new();
    let mut selezione_infoaggiuntive_view = SelezioneInfoAggiuntiveView::new();
    let mut validazione_infoaggiuntive_view = ValidazioneInfoAggiuntiveView::new();
    let output_controller = OutputController::new();
    let mut produzione_output_view = ProduzioneOutputView::new();
    let mut produzione_pdf_view = ProduzionePDFView::new();
    let console_controller = ConsoleController::new();

    let window_title = format!("esox v{SHORT_PROJECT_VERSION}");

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
    match logo_img {
        Some(img) => {
            logo_texture = Some(rl.load_texture_from_image(&thread, &img).unwrap());
        }
        None => {}
    }

    // 10 is way too small for the default font height
    let gui_default_font_height: i32 = rl.gui_get_style(DEFAULT, TEXT_SIZE as i32) *2;
    rl.gui_set_style(DEFAULT, TEXT_SIZE as i32, gui_default_font_height);
    let gui_current_font_height: i32 = gui_default_font_height;

    let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL as i32);
    let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR as i32);
    let txt_spacing = rl.gui_get_style(DEFAULT, TEXT_SPACING as i32);
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

    let mut console_view = ConsoleView::new(&mut rl, &thread, gui_current_font_height, txt_spacing);

    while !main_state.should_quit {

        // Base update step
        update_main(&mut rl, &mut main_state);

        // Current view update step
        match main_state.current_view {
            CurrentView::HOME => {
                home_controller.update(&rl, &mut main_state);
            }
            CurrentView::SECOND => {
                second_controller.update(&rl, &mut main_state);
            }
            CurrentView::SelezioneIndice => {
                indice_controller.update(&rl, &mut main_state);
            }
            CurrentView::SelezioneFileInput | CurrentView::ValidazioneFileInput => {
                fileinput_controller.update(&rl, &mut main_state);
            }
            CurrentView::SelezioneInfoAggiuntive | CurrentView::ValidazioneInfoAggiuntive => {
                infoaggiuntive_controller.update(&rl, &mut main_state);
            }
            CurrentView::ProduzioneOutput | CurrentView::ProduzionePDF=> {
                output_controller.update(&rl, &mut main_state);
            }
            CurrentView::CONSOLE => {
                console_controller.update(&mut rl, &mut main_state);
            }
        }

        let mut d = rl.begin_drawing(&thread);

        let lock_view = main_state.get_gui_should_lock();

        if lock_view {
            d.gui_lock();
        }

        // Ask the view for render, passing the controller for state changes
        // Current view draw step
        match main_state.current_view {
            CurrentView::HOME => {
                home_view.draw(&mut d, &thread, &home_controller, &main_state);
            }
            CurrentView::SECOND => {
                second_view.draw(&mut d, &thread, &second_controller, &main_state);
            }
            CurrentView::SelezioneIndice => {
                selezione_indice_view.draw(&mut d, &thread, &indice_controller, &main_state);
            }
            CurrentView::SelezioneFileInput => {
                selezione_fileinput_view.draw(&mut d, &thread, &fileinput_controller, &main_state);
            }
            CurrentView::ValidazioneFileInput => {
                validazione_fileinput_view.draw(&mut d, &thread, &fileinput_controller, &main_state);
            }
            CurrentView::SelezioneInfoAggiuntive => {
                selezione_infoaggiuntive_view.draw(&mut d, &thread, &infoaggiuntive_controller, &main_state);
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                validazione_infoaggiuntive_view.draw(&mut d, &thread, &infoaggiuntive_controller, &main_state);
            }
            CurrentView::ProduzioneOutput => {
                produzione_output_view.draw(&mut d, &thread, &output_controller, &main_state);
            }
            CurrentView::ProduzionePDF => {
                produzione_pdf_view.draw(&mut d, &thread, &output_controller, &main_state);
            }
            CurrentView::CONSOLE => {
                console_view.draw(&mut d, &thread, &console_controller, &main_state);
            }
        }

        if lock_view {
            d.gui_unlock();
        }

        // Base draw step
        // Render stuff not depending on view
        draw_main(&mut d, &mut main_state);
    }
}
