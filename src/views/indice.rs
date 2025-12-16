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
use crate::app::core::{Action, Action::*};
use crate::app::model::Model;
use crate::domain::index::Indice;
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
pub(crate) struct SelezioneIndiceView {}

impl View for SelezioneIndiceView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        _state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        d.clear_background(main_state.default_bg_color);

        let button_niseci_width = propwidth(d, 200);
        let button_niseci_x = d.get_screen_width() / 2 - button_niseci_width / 2;
        let button_niseci_height = propwidth(d, 50);

        let button_indice_y_spacing = button_niseci_height;

        let button_niseci_y =
            d.get_screen_height() / 2 - button_indice_y_spacing / 2 - button_niseci_height;

        let button_hfbi_width = button_niseci_width;
        let button_hfbi_x = button_niseci_x;
        let button_hfbi_height = button_niseci_height;
        let button_hfbi_y = button_niseci_y + button_niseci_height + button_indice_y_spacing;

        let groupbox_width = button_niseci_width + propwidth(d, 100);
        let groupbox_x = button_niseci_x - propwidth(d, 50);
        let groupbox_height =
            button_niseci_height * 2 + button_indice_y_spacing + propheight(d, 100);
        let groupbox_y = button_niseci_y - propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Seleziona Indice",
        );

        let mut actions = Vec::<Action>::new();

        if d.gui_button(
            rrect(
                button_niseci_x,
                button_niseci_y,
                button_niseci_width,
                button_niseci_height,
            ),
            "NISECI",
        ) {
            actions.push(PickIndice(Indice::Niseci));
        }

        if d.gui_button(
            rrect(
                button_hfbi_x,
                button_hfbi_y,
                button_hfbi_width,
                button_hfbi_height,
            ),
            "HFBI",
        ) {
            actions.push(PickIndice(Indice::Hfbi));
        }

        actions
    }
}

impl SelezioneIndiceView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
