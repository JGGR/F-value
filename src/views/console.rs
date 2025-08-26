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
use crate::app::core::CONSOLE_FONT_DATA;
use crate::views::View;
use crate::controllers::{Controller, console::ConsoleController};
use crate::MainState;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibThread;
use raylib::prelude::*;
pub(crate) struct ConsoleView {
    font: Font,
    current_font_size: i32,
    _default_font_size: i32,
    font_spacing: i32,
}

impl View for ConsoleView {
    type Controller = ConsoleController;

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        controller: &Self::Controller,
        main_state: &MainState,
    ) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();

        state.console.draw(
            d,
            controller,
            main_state.default_txt_color,
            self.current_font_size,
            self.font_spacing,
            &self.font,
        );
    }
}

impl ConsoleView {
    pub(crate) fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        font_size: i32,
        font_spacing: i32,
    ) -> Self {
        Self {
            font: rl
                .load_font_from_memory(
                    thread,
                    ".ttf",
                    CONSOLE_FONT_DATA,
                    font_size, // *2,
                    None,
                )
                .expect("failed loading console font"),
            _default_font_size: font_size,
            current_font_size: font_size,
            font_spacing: font_spacing * 2,
        }
    }
}
