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
use crate::controllers::help::HelpController;
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::consts::GuiIconName::ICON_PLAYER_NEXT;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;
use raylib::RaylibThread;

// A view responsible for rendering the state
// Tightly coupled with its respective controller

pub(crate) struct HelpView {
}

impl View for HelpView {
    type Controller = HelpController;

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        controller: &Self::Controller,
        main_state: &MainState,
    ) {
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

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width() - continue_width - propwidth(d, 50);
        let continue_height = propwidth(d, 50);
        let continue_y = d.get_screen_height() - propheight(d, 150);

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        if d.gui_button(
            rrect(continue_x, continue_y, continue_width, continue_height),
            continue_itext.as_str(),
        ) {
            controller.set_user_continued(true);
        }
    }
}

impl HelpView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
