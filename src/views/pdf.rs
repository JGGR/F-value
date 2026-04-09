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
use crate::views::GuiIconName::ICON_RESTART;
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::consts::GuiState::{STATE_DISABLED, STATE_NORMAL};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
use rfd::FileDialog;
pub(crate) struct ProduzionePDFView {}

impl View for ProduzionePDFView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        self.draw_background(d, main_state);

        let button_esporta_width = propwidth(d, 200);
        let button_esporta_x = d.get_screen_width() / 2 - button_esporta_width / 2;
        let button_esporta_height = propwidth(d, 50);
        let button_esporta_y = d.get_screen_height() / 2 - button_esporta_height / 2;

        let groupbox_width = button_esporta_width + propwidth(d, 100);
        let groupbox_x = button_esporta_x - propwidth(d, 50);
        let groupbox_height = button_esporta_height + propheight(d, 100);
        let groupbox_y = button_esporta_y - propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Produzione PDF",
        );

        let mut actions = Vec::<Action>::new();

        if d.gui_button(
            rrect(
                button_esporta_x,
                button_esporta_y,
                button_esporta_width,
                button_esporta_height,
            ),
            "Esporta",
        ) {
            let file = FileDialog::new()
                .add_filter("pdf", &["pdf"])
                .set_directory("/")
                .save_file();

            if let Some(filepath) = file {
                actions.push(ExportPdf(filepath));
            } else {
                eprintln!("Error: failed getting a file.");
            }
        }

        let button_reset_width = propwidth(d, 200);
        let button_reset_x = groupbox_x
            + groupbox_width
            + (d.get_screen_width() - (groupbox_x + groupbox_width + button_reset_width)) / 2;
        let button_reset_height = propwidth(d, 50);
        let button_reset_y = d.get_screen_height() / 2 - button_reset_height / 2;

        let done_export = state.output_model.is_done_export();
        let reset_itext = d.gui_icon_text(ICON_RESTART, "Reset");

        let lock_button_reset = !done_export;

        if lock_button_reset {
            d.gui_lock();
            d.gui_set_state(STATE_DISABLED);
        }
        if d.gui_button(
            rrect(
                button_reset_x,
                button_reset_y,
                button_reset_width,
                button_reset_height,
            ),
            reset_itext.as_str(),
        ) {
            actions.push(Reset);
        }
        if lock_button_reset {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }
        actions
    }
}

impl ProduzionePDFView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
