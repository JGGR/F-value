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

    fn draw_background(&mut self, d: &mut RaylibDrawHandle, main_state: &MainState) {
        d.clear_background(main_state.colors.default_bg_color);
        let texture_target_width = d.get_screen_width();
        let texture_target_height = d.get_screen_height();
        let texture_target_x = 0;
        let texture_target_y = 0;
        if let Some(ref texture) = main_state.textures.bg_texture {
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture.width() as f32,
                    height: texture.height() as f32,
                },
                Rectangle {
                    x: texture_target_x as f32,
                    y: texture_target_y as f32,
                    width: texture_target_width as f32,
                    height: texture_target_height as f32,
                },
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
        let panels_line_thickness = 1.0;
        d.draw_rectangle(
            propwidth(d, 100),
            0,
            d.get_screen_width() - propwidth(d, 200),
            d.get_screen_height(),
            main_state.colors.default_bg_color,
        );
        d.draw_rectangle_lines_ex(
            rrect(
                propwidth(d, 100),
                0,
                d.get_screen_width() - propwidth(d, 200),
                d.get_screen_height(),
            ),
            panels_line_thickness,
            main_state.colors.default_txt_color,
        );
    }
}
