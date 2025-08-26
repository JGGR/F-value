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

use crate::app::core::{propheight, propwidth, CurrentView, MainState};
use crate::app::model::SubModel;
use crate::controllers::{Controller, Controllers, SecondController};
use crate::core::rrect;
use crate::core::SHORT_PROJECT_VERSION;
use raylib::consts::GuiIconName::ICON_PLAYER_NEXT;
use raylib::prelude::*;
use std::cmp::max;

pub(crate) mod home;
use home::HomeView;
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
    second_view: SecondView,
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
            second_view: SecondView::new(),
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
        controllers: &Controllers,
        main_state: &MainState,
    ) {
        match main_state.current_view {
            CurrentView::Home => {
                self.home_view
                    .draw(d, thread, &controllers.home_controller, main_state);
            }
            CurrentView::Second => {
                self.second_view
                    .draw(d, thread, &controllers.second_controller, main_state);
            }
            CurrentView::SelezioneIndice => {
                self.selezione_indice_view.draw(
                    d,
                    thread,
                    &controllers.indice_controller,
                    main_state,
                );
            }
            CurrentView::SelezioneFileInput => {
                self.selezione_fileinput_view.draw(
                    d,
                    thread,
                    &controllers.fileinput_controller,
                    main_state,
                );
            }
            CurrentView::ValidazioneFileInput => {
                self.validazione_fileinput_view.draw(
                    d,
                    thread,
                    &controllers.fileinput_controller,
                    main_state,
                );
            }
            CurrentView::SelezioneInfoAggiuntive => {
                self.selezione_infoaggiuntive_view.draw(
                    d,
                    thread,
                    &controllers.infoaggiuntive_controller,
                    main_state,
                );
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                self.validazione_infoaggiuntive_view.draw(
                    d,
                    thread,
                    &controllers.infoaggiuntive_controller,
                    main_state,
                );
            }
            CurrentView::ProduzioneOutput => {
                self.produzione_output_view.draw(
                    d,
                    thread,
                    &controllers.output_controller,
                    main_state,
                );
            }
            CurrentView::ProduzionePDF => {
                self.produzione_pdf_view.draw(
                    d,
                    thread,
                    &controllers.output_controller,
                    main_state,
                );
            }
            CurrentView::Console => {
                self.console_view
                    .draw(d, thread, &controllers.console_controller, main_state);
            }
        }
    }
}

pub(crate) trait View {
    type Controller: Controller;
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        controller: &Self::Controller,
        main_state: &MainState,
    );
}

// A view responsible for rendering the state
// Tightly coupled with its respective controller

pub(crate) struct SecondView {
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl View for SecondView {
    type Controller = SecondController;

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        controller: &Self::Controller,
        main_state: &MainState,
    ) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(d, 100) as f32, propheight(d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color,
        );

        let updated_spinner = d.gui_spinner(
            rrect(
                propwidth(d, 100),
                propheight(d, 50),
                propwidth(d, 125),
                propheight(d, 30),
            ),
            "",
            &mut self.spinner_value,
            0,
            100,
            self.spinner_edit_mode,
        );
        if updated_spinner {
            self.spinner_edit_mode = !self.spinner_edit_mode;
        }

        // gui_value_box() (and gui_spinner() too since it's used by it. The "value" argument
        // must be a value living for the whole draw loop, so we just dup them
        // to the View and ensure to set them on all frames to the model via
        // the controller.
        controller.set_value(self.spinner_value);

        let texture_target_width = propwidth(d, 205);
        let texture_target_height = propheight(d, 205);
        let texture_target_x = d.get_screen_width() / 2 - texture_target_width / 2;
        let texture_target_y = propheight(d, 50);
        if let Some(ref texture) = main_state.logo_texture {
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

        let label_version_txt = format!("Version:   {}", SHORT_PROJECT_VERSION);
        let label_target_txt = format!(
            "Target:    {}-{}",
            std::env::consts::ARCH,
            std::env::consts::OS
        );
        let label_version_txt_bounds = main_state.current_font.measure_text(
            &label_version_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let label_target_txt_bounds = main_state.current_font.measure_text(
            &label_target_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let labels_width = propwidth(d, 25)
            + max(
                label_version_txt_bounds.x as i32,
                label_target_txt_bounds.x as i32,
            );
        let labels_x = d.get_screen_width() / 2 - labels_width / 2;
        let labels_y = propheight(d, 300);
        let labels_height = propheight(d, 25);

        let labels: Vec<String> = vec![label_version_txt, label_target_txt];

        for (i, label) in labels.iter().enumerate() {
            d.gui_label(
                rrect(
                    labels_x,
                    labels_y + (i as i32 * labels_height),
                    labels_width,
                    labels_height,
                ),
                label.as_str(),
            );
        }

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width() / 2 - continue_width / 2;
        let continue_height = propwidth(d, 50);
        let continue_y_padding = propwidth(d, 25);
        let continue_y = labels_y + (labels_height * labels.len() as i32) + continue_y_padding;

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        if d.gui_button(
            rrect(continue_x, continue_y, continue_width, continue_height),
            continue_itext.as_str(),
        ) {
            controller.set_user_continued(true);
        }

        let rainbow_speed = 0.03;
        let todo_font_scale = 3;
        let todo_font_height = main_state.current_font_height * todo_font_scale;

        let todo_txt = "TODO: WELCOME";
        let todo_txt_bounds = main_state.current_font.measure_text(
            todo_txt,
            todo_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let todo_txt_x = (d.get_screen_width() / 2) - (todo_txt_bounds.x as i32 / 2);
        let todo_txt_y = (d.get_screen_height() / 2) - (todo_txt_bounds.y as i32 / 2);

        draw_rainbow_text(
            d,
            todo_txt_x,
            todo_txt_y,
            "TODO: WELCOME",
            frame_counter,
            rainbow_speed,
            &main_state.current_font,
            main_state.default_txt_spacing,
            main_state.current_font_height,
            todo_font_scale,
        );
    }
}

impl SecondView {
    pub(crate) fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
}

fn rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    Color::new(red, green, blue, 255)
}

fn draw_rainbow_text(
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
    let rainbow_color = rainbow_color_from_framecounter(frame_counter, rainbow_speed);

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
