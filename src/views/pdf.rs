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
use crate::views::{View, propwidth, propheight, rrect};
use crate::controllers::{Controller, output::OutputController};
use crate::MainState;
use crate::domain::index::Indice;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibThread;
use raylib::prelude::*;
use rfd::FileDialog;
pub(crate) struct ProduzionePDFView {}

impl View for ProduzionePDFView {
    type Controller = OutputController;

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        controller: &Self::Controller,
        main_state: &MainState,
    ) {
        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("ProduzionePDFView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };
        let button_esporta_width = propwidth(d, 200);
        let button_esporta_x = d.get_screen_width() / 2 - button_esporta_width / 2;
        let button_esporta_height = propwidth(d, 50);
        let button_esporta_y = d.get_screen_height() / 4 - button_esporta_height / 2;

        let groupbox_width = button_esporta_width + propwidth(d, 100);
        let groupbox_x = button_esporta_x - propwidth(d, 50);
        let groupbox_height = button_esporta_height + propheight(d, 100);
        let groupbox_y = button_esporta_y - propheight(d, 50);

        let panel_width = groupbox_width + propwidth(d, 100);
        let panel_x = d.get_screen_width() / 2 - panel_width / 2;
        let panel_y = groupbox_y + groupbox_height + propwidth(d, 50);
        let panel_height = groupbox_height + propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Produzione PDF",
        );

        if d.gui_button(
            rrect(
                button_esporta_x,
                button_esporta_y,
                button_esporta_width,
                button_esporta_height,
            ),
            "Esporta",
        ) {
            //TODO: esporta pdf
            let file = FileDialog::new()
                .add_filter("pdf", &["pdf"])
                .set_directory("/")
                .save_file();

            if let Some(filepath) = file {
                match current_index {
                    Indice::Niseci => {
                        controller.esporta_pdf_niseci(filepath);
                    }
                    Indice::Hfbi => {
                        controller.esporta_pdf_hfbi(filepath);
                    }
                }
            } else {
                eprintln!("Error: failed getting a file.");
                controller
                    .add_console_message("Failed getting a file for esportazione".to_string());
            }
        }

        d.gui_panel(
            rrect(panel_x, panel_y, panel_width, panel_height),
            "TODO: Output qui",
        );
    }
}

impl ProduzionePDFView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
