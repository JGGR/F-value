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

use crate::app::core::{propheight, propwidth, Action, CurrentView, MainState};
use crate::app::model::Model;
use crate::core::rrect;
use raylib::prelude::*;

pub(crate) mod home;
use home::HomeView;
pub(crate) mod help;
use help::HelpView;
pub(crate) mod indice;
use indice::SelezioneIndiceView;
pub(crate) mod selezione_file_input;
use selezione_file_input::SelezioneFileInputView;
pub(crate) mod validazione_file_input;
use validazione_file_input::ValidazioneFileInputView;
pub(crate) mod selezione_info_aggiuntive;
use selezione_info_aggiuntive::SelezioneInfoAggiuntiveView;
pub(crate) mod validazione_info_aggiuntive;
use validazione_info_aggiuntive::ValidazioneInfoAggiuntiveView;
pub(crate) mod output;
use output::ProduzioneOutputView;
pub(crate) mod pdf;
use pdf::ProduzionePDFView;
pub(crate) mod console;
use console::ConsoleView;

pub(crate) struct Views {
    home_view: HomeView,
    help_view: HelpView,
    selezione_indice_view: SelezioneIndiceView,
    selezione_fileinput_view: SelezioneFileInputView,
    validazione_fileinput_view: ValidazioneFileInputView,
    selezione_infoaggiuntive_view: SelezioneInfoAggiuntiveView,
    validazione_infoaggiuntive_view: ValidazioneInfoAggiuntiveView,
    produzione_output_view: ProduzioneOutputView,
    produzione_pdf_view: ProduzionePDFView,
    console_view: ConsoleView,
}

impl Views {
    pub(crate) fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        gui_current_font_height: i32,
        txt_spacing: i32,
    ) -> Self {
        Self {
            home_view: HomeView::new(),
            help_view: HelpView::new(),
            selezione_indice_view: SelezioneIndiceView::new(),
            selezione_fileinput_view: SelezioneFileInputView::new(),
            validazione_fileinput_view: ValidazioneFileInputView::new(),
            selezione_infoaggiuntive_view: SelezioneInfoAggiuntiveView::new(),
            validazione_infoaggiuntive_view: ValidazioneInfoAggiuntiveView::new(),
            produzione_output_view: ProduzioneOutputView::new(),
            produzione_pdf_view: ProduzionePDFView::new(),
            console_view: ConsoleView::new(rl, thread, gui_current_font_height, txt_spacing),
        }
    }
    pub(crate) fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        match main_state.current_view {
            CurrentView::Home => self.home_view.draw(d, thread, state, main_state),
            CurrentView::Help => self.help_view.draw(d, thread, state, main_state),
            CurrentView::SelezioneIndice => self
                .selezione_indice_view
                .draw(d, thread, state, main_state),
            CurrentView::SelezioneFileInput => self
                .selezione_fileinput_view
                .draw(d, thread, state, main_state),
            CurrentView::ValidazioneFileInput => self
                .validazione_fileinput_view
                .draw(d, thread, state, main_state),
            CurrentView::SelezioneInfoAggiuntive => self
                .selezione_infoaggiuntive_view
                .draw(d, thread, state, main_state),
            CurrentView::ValidazioneInfoAggiuntive => self
                .validazione_infoaggiuntive_view
                .draw(d, thread, state, main_state),
            CurrentView::ProduzioneOutput => self
                .produzione_output_view
                .draw(d, thread, state, main_state),
            CurrentView::ProduzionePDF => {
                self.produzione_pdf_view.draw(d, thread, state, main_state)
            }
            CurrentView::Console => self.console_view.draw(d, thread, state, main_state),
        }
    }
}

pub(crate) trait View {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        state: &Model,
        main_state: &MainState,
    ) -> Vec<Action>;
}

fn _rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    Color::new(red, green, blue, 255)
}

fn _draw_rainbow_text(
    d: &mut RaylibDrawHandle,
    x: i32,
    y: i32,
    text: &str,
    frame_counter: u32,
    rainbow_speed: f32,
    font: &WeakFont,
    text_spacing: i32,
    current_font_height: i32,
    font_height_scale: i32,
) {
    assert!(font_height_scale > 0);
    // Smaller speed = slower cycle
    let rainbow_color = _rainbow_color_from_framecounter(frame_counter, rainbow_speed);

    let text_font_height = current_font_height * font_height_scale;
    //let text_bounds = font.measure_text(&text, text_font_height as f32, text_spacing as f32);
    let text_x = x; //- text_bounds.x as i32 / 2;
    let text_y = y; //- text_bounds.y as i32 / 2;
    d.draw_text_ex(
        font,
        text,
        Vector2::new(text_x as f32, text_y as f32),
        text_font_height as f32,
        text_spacing as f32,
        rainbow_color,
    );
}
