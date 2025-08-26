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
use crate::app::model::SubModel;
use crate::controllers::{home::HomeController, Controller};
use crate::views::{draw_rainbow_text, propheight, propwidth, rrect, View};
use crate::MainState;
use crate::SHORT_PROJECT_VERSION;
use raylib::color::Color;
use raylib::consts::GuiIconName::ICON_PLAYER_NEXT;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;
use raylib::RaylibThread;
use std::cmp::max;
pub(crate) struct HomeView {}

impl View for HomeView {
    type Controller = HomeController;

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

impl HomeView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
