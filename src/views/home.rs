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
        self.draw_background(d, main_state);

        let logo_texture_target_width = propwidth(d, 205);
        let logo_texture_target_height = propheight(d, 205);
        let logo_texture_target_x = d.get_screen_width() / 2 - logo_texture_target_width / 2;
        let logo_texture_target_y = propheight(d, 50);
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
                    x: logo_texture_target_x as f32,
                    y: logo_texture_target_y as f32,
                    width: logo_texture_target_width as f32,
                    height: logo_texture_target_height as f32,
                },
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        // logo_name.png is 512 x 82
        let logo_name_texture_target_width = propwidth(d, 256);
        let logo_name_texture_target_height = propheight(d, 41);
        let logo_name_texture_target_x = d.get_screen_width() / 2 - logo_name_texture_target_width / 2;
        let logo_name_texture_target_y = logo_texture_target_y + logo_texture_target_height + propheight(d, 10);
        if let Some(ref texture) = main_state.logo_name_texture {
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture.width() as f32,
                    height: texture.height() as f32,
                },
                Rectangle {
                    x: logo_name_texture_target_x as f32,
                    y: logo_name_texture_target_y as f32,
                    width: logo_name_texture_target_width as f32,
                    height: logo_name_texture_target_height as f32,
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
        let label_rfd_backend_txt = format!("rfd backend:    {}", RFD_BACKEND);
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
                    label_version_txt_bounds.x as i32,
                    label_target_txt_bounds.x as i32
                ),
                label_rfd_backend_txt_bounds.x as i32,
            );
        let labels_x = (d.get_screen_width() - labels_width) / 2;
        let labels_y = logo_name_texture_target_y + logo_name_texture_target_height + propheight(d, 50);
        let labels_height = propheight(d, 25);

        let labels: Vec<String> = vec![
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

        let buttons_offset = propwidth(d, 50);
        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width() - continue_width - buttons_offset;
        let continue_height = propwidth(d, 50);
        let continue_y = d.get_screen_height() - propheight(d, 150);

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        let info_width = continue_width;
        let info_x = buttons_offset;
        let info_height = continue_height;
        let info_y = continue_y;

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
