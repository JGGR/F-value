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
use crate::app::core::{Action, Action::*};
use crate::app::model::Model;
use crate::core::{RFD_BACKEND, SHORT_PROJECT_VERSION};
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::color::Color;
use raylib::consts::GuiIconName::{ICON_INFO, ICON_PLAYER_NEXT};
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;
use raylib::RaylibThread;
use std::cmp::max;
pub(crate) struct HomeView {}

impl View for HomeView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        _state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        d.clear_background(main_state.default_bg_color);

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

        let label_name_txt = "FISH-VALUE".to_string();
        let label_version_txt = format!("Version:   {}", SHORT_PROJECT_VERSION);
        let label_target_txt = format!(
            "Target:    {}-{}",
            std::env::consts::ARCH,
            std::env::consts::OS
        );
        let label_rfd_backend_txt = format!("rfd backend:    {}", RFD_BACKEND);
        let label_name_txt_bounds = main_state.current_font.measure_text(
            &label_name_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
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
        let label_rfd_backend_txt_bounds = main_state.current_font.measure_text(
            &label_rfd_backend_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let labels_width = propwidth(d, 25)
            + max(
                max(
                    max(
                        label_name_txt_bounds.x as i32,
                        label_version_txt_bounds.x as i32,
                    ),
                    label_target_txt_bounds.x as i32,
                ),
                label_rfd_backend_txt_bounds.x as i32,
            );
        let labels_x = propwidth(d, 50);
        let labels_y = propheight(d, 50);
        let labels_height = propheight(d, 25);

        let labels: Vec<String> = vec![
            label_name_txt,
            label_version_txt,
            label_target_txt,
            label_rfd_backend_txt,
        ];

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

        let label_andrea_marchi_txt = "Dr. Andrea Marchi".to_string();
        let label_hydrosynergy_txt = "Hydrosynergy Società Cooperativa".to_string();
        let label_societ_txt =
            "Società Spin-off accreditata dell'Alma Mater Studiorum - Università di Bologna"
                .to_string();
        let label_andrea_marchi_txt_bounds = main_state.current_font.measure_text(
            &label_andrea_marchi_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let label_hydrosynergy_txt_bounds = main_state.current_font.measure_text(
            &label_hydrosynergy_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let label_societ_txt_bounds = main_state.current_font.measure_text(
            &label_societ_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );

        let dr_andrea_labels_width = propwidth(d, 25)
            + max(
                max(
                    label_andrea_marchi_txt_bounds.x as i32,
                    label_hydrosynergy_txt_bounds.x as i32,
                ),
                label_societ_txt_bounds.x as i32,
            );

        let dr_andrea_labels: Vec<String> = vec![
            label_andrea_marchi_txt,
            label_hydrosynergy_txt,
            label_societ_txt,
        ];
        let dr_andrea_labels_y = propheight(d, 200);

        for (i, label) in dr_andrea_labels.iter().enumerate() {
            d.gui_label(
                rrect(
                    labels_x,
                    dr_andrea_labels_y + (i as i32 * labels_height),
                    dr_andrea_labels_width,
                    labels_height,
                ),
                label.as_str(),
            );
        }

        let andrea_email = "a.marchi@hsbologna.it";
        let andrea_mail_display_link = andrea_email;
        let andrea_mail_actual_link = "mailto:".to_owned() + andrea_email;
        let andrea_mail_link_str = andrea_mail_display_link;
        let andrea_mail_link_x = labels_x;
        let andrea_mail_link_y = dr_andrea_labels_y + dr_andrea_labels.len() as i32 * labels_height;
        let andrea_mail_link_width = propwidth(d, 50);
        let andrea_mail_link_height = labels_height;

        if d.gui_label_button(
            rrect(
                andrea_mail_link_x,
                andrea_mail_link_y,
                andrea_mail_link_width,
                andrea_mail_link_height,
            ),
            andrea_mail_link_str,
        ) {
            raylib::core::misc::open_url(&andrea_mail_actual_link);
        }

        let label_salvatore_de_bonis_txt = "Dr. Salvatore De Bonis".to_string();
        let label_agenzia_txt =
            "Agenzia Regionale per la Protezione Ambientale del Lazio".to_string();
        let label_dipartimento_txt =
            "Dipartimento Stato dell'Ambiente - Unità Risorse Idriche di Roma".to_string();
        let label_salvatore_de_bonis_txt_bounds = main_state.current_font.measure_text(
            &label_salvatore_de_bonis_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let label_agenzia_txt_bounds = main_state.current_font.measure_text(
            &label_agenzia_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );
        let label_dipartimento_txt_bounds = main_state.current_font.measure_text(
            &label_dipartimento_txt,
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
        );

        let dr_salvatore_labels: Vec<String> = vec![
            label_salvatore_de_bonis_txt,
            label_agenzia_txt,
            label_dipartimento_txt,
        ];

        let dr_salvatore_labels_width = propwidth(d, 25)
            + max(
                max(
                    label_salvatore_de_bonis_txt_bounds.x as i32,
                    label_agenzia_txt_bounds.x as i32,
                ),
                label_dipartimento_txt_bounds.x as i32,
            );

        let dr_salvatore_labels_y =
            dr_andrea_labels_y + (dr_andrea_labels.len() as i32 + 2) * labels_height;

        for (i, label) in dr_salvatore_labels.iter().enumerate() {
            d.gui_label(
                rrect(
                    labels_x,
                    dr_salvatore_labels_y + (i as i32 * labels_height),
                    dr_salvatore_labels_width,
                    labels_height,
                ),
                label.as_str(),
            );
        }

        let salvatore_email = "salvatore.debonis@arpalazio.it";
        let salvatore_mail_display_link = salvatore_email;
        let salvatore_mail_actual_link = "mailto:".to_owned() + salvatore_email;
        let salvatore_mail_link_str = salvatore_mail_display_link;
        let salvatore_mail_link_x = labels_x;
        let salvatore_mail_link_y =
            dr_salvatore_labels_y + dr_salvatore_labels.len() as i32 * labels_height;
        let salvatore_mail_link_width = propwidth(d, 50);
        let salvatore_mail_link_height = labels_height;

        if d.gui_label_button(
            rrect(
                salvatore_mail_link_x,
                salvatore_mail_link_y,
                salvatore_mail_link_width,
                salvatore_mail_link_height,
            ),
            salvatore_mail_link_str,
        ) {
            raylib::core::misc::open_url(&salvatore_mail_actual_link);
        }

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width() - continue_width - propwidth(d, 50);
        let continue_height = propwidth(d, 50);
        let continue_y = d.get_screen_height() - propheight(d, 150);

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        let info_width = continue_width;
        let info_x = continue_x;
        let info_height = continue_height;
        let info_y = continue_y - continue_height * 2;

        let info_itext = d.gui_icon_text(ICON_INFO, ": Info");

        let mut actions = Vec::<Action>::new();

        if d.gui_button(
            rrect(continue_x, continue_y, continue_width, continue_height),
            continue_itext.as_str(),
        ) {
            actions.push(UserContinued);
        }
        if d.gui_button(
            rrect(info_x, info_y, info_width, info_height),
            info_itext.as_str(),
        ) {
            actions.push(UserWantsInfo);
        }

        actions
    }
}

impl HomeView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
