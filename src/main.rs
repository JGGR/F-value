#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod model;
mod views;
mod controllers;
mod core;

use crate::core::*;
use crate::core::view::*;
use crate::core::controller::*;
use crate::controllers::*;
use crate::views::*;
use raylib::prelude::*;
use raylib::consts::GuiControl::DEFAULT;
use raylib::consts::GuiDefaultProperty::TEXT_SIZE;
use raylib::consts::GuiDefaultProperty::BACKGROUND_COLOR;
use raylib::consts::GuiControlProperty::TEXT_COLOR_NORMAL;
fn main() {
    eprintln!("{PROJECT_VERSION}");
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

    let window_title = format!("esox v{SHORT_PROJECT_VERSION}");

    let (mut rl, thread) = raylib::init()
        .size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT)
        .title(&window_title)
        .resizable()
        .build();

    rl.set_window_min_size(ESOX_SCREEN_WIDTH, ESOX_SCREEN_HEIGHT);
    rl.set_exit_key(None); // This allows capturing the exit key with a message box
    rl.set_target_fps(30);

    let mut default_font_height : i32 = rl.gui_get_style(DEFAULT, TEXT_SIZE as i32);
    let mut current_font_height : i32 = default_font_height;


    let txt_color_int = rl.gui_get_style(DEFAULT, TEXT_COLOR_NORMAL as i32);
    let bg_color_int = rl.gui_get_style(DEFAULT, BACKGROUND_COLOR as i32);
    let mut main_state = MainState::new(
        Color::get_color(txt_color_int as u32),
        Color::get_color(bg_color_int as u32)
    );

    while !main_state.should_quit {

        // Base update step
        update_main(&mut rl, &mut main_state, &mut default_font_height, &mut current_font_height);

        // Current view update step
        match main_state.current_view {
            CurrentView::HOME => {
                home_controller.update(&rl);
            }
            CurrentView::SECOND => {
                second_controller.update(&rl);
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
        }

        let mut d = rl.begin_drawing(&thread);

        // Ask the view for render, passing the controller for state changes
        // Current view draw step
        match main_state.current_view {
            CurrentView::HOME => {
                home_view.draw(&mut d, &thread, &home_controller, &main_state, current_font_height);
            }
            CurrentView::SECOND => {
                second_view.draw(&mut d, &thread, &second_controller, &main_state, current_font_height);
            }
            CurrentView::SelezioneIndice => {
                selezione_indice_view.draw(&mut d, &thread, &indice_controller, &main_state, current_font_height);
            }
            CurrentView::SelezioneFileInput => {
                selezione_fileinput_view.draw(&mut d, &thread, &fileinput_controller, &main_state, current_font_height);
            }
            CurrentView::ValidazioneFileInput => {
                validazione_fileinput_view.draw(&mut d, &thread, &fileinput_controller, &main_state, current_font_height);
            }
            CurrentView::SelezioneInfoAggiuntive => {
                selezione_infoaggiuntive_view.draw(&mut d, &thread, &infoaggiuntive_controller, &main_state, current_font_height);
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                validazione_infoaggiuntive_view.draw(&mut d, &thread, &infoaggiuntive_controller, &main_state, current_font_height);
            }
            CurrentView::ProduzioneOutput => {
                produzione_output_view.draw(&mut d, &thread, &output_controller, &main_state, current_font_height);
            }
            CurrentView::ProduzionePDF => {
                produzione_pdf_view.draw(&mut d, &thread, &output_controller, &main_state, current_font_height);
            }
        }

        // Base draw step
        // Render stuff not depending on view
        draw_main(&mut d, &mut main_state, &mut current_font_height, default_font_height);
    }
}

