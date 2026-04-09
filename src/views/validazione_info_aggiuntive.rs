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
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::consts::GuiIconName::ICON_CROSS;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
pub(crate) struct ValidazioneInfoAggiuntiveView {}

impl View for ValidazioneInfoAggiuntiveView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        _state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        self.draw_background(d, main_state);

        let button_valida_width = propwidth(d, 200);
        let button_valida_x = d.get_screen_width() / 2 - button_valida_width / 2;
        let button_valida_height = propwidth(d, 50);
        let button_valida_y = d.get_screen_height() / 2 - button_valida_height / 2;

        let y_spacing = button_valida_height;
        let button_backout_width = button_valida_width;
        let button_backout_x = button_valida_x;
        let button_backout_height = button_valida_height;
        let button_backout_y = button_valida_y + button_valida_height + y_spacing;

        let groupbox_width = button_valida_width + propwidth(d, 100);
        let groupbox_x = button_valida_x - propwidth(d, 50);
        let groupbox_height = button_valida_height * 3 + propheight(d, 100);
        let groupbox_y = button_valida_y - propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Valida informazioni aggiuntive",
        );

        let mut actions = Vec::<Action>::new();
        if d.gui_button(
            rrect(
                button_valida_x,
                button_valida_y,
                button_valida_width,
                button_valida_height,
            ),
            "Valida info aggiuntive",
        ) {
            actions.push(CheckAnagrafica);
        }

        let indietro_itext = d.gui_icon_text(ICON_CROSS, "Indietro");
        if d.gui_button(
            rrect(
                button_backout_x,
                button_backout_y,
                button_backout_width,
                button_backout_height,
            ),
            indietro_itext.as_str(),
        ) {
            actions.push(BackoutAnagrafica);
        }
        actions
    }
}

impl ValidazioneInfoAggiuntiveView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
