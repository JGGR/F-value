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
use crate::controllers::{Controller, file_input::FileInputController};
use crate::MainState;
use crate::domain::index::Indice;
use raylib::drawing::RaylibDrawHandle;
use raylib::consts::GuiIconName::{ICON_BIN, ICON_FILE_OPEN};
use raylib::RaylibThread;
use raylib::prelude::*;
use rfd::FileDialog;
pub(crate) struct SelezioneFileInputView {}

impl View for SelezioneFileInputView {
    type Controller = FileInputController;

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
                eprintln!("SelezioneFileInputView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

        let button_riferimento_width = propwidth(d, 200);
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width / 2;
        let button_riferimento_height = propwidth(d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y =
            d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = match current_index {
            Indice::Hfbi => button_riferimento_y + button_fileinput_y_spacing,
            Indice::Niseci => {
                button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing
            }
        };

        let groupbox_width = button_riferimento_width + propwidth(d, 100);
        let groupbox_x = button_riferimento_x - propwidth(d, 50);
        let groupbox_height =
            button_riferimento_height * 2 + button_fileinput_y_spacing + propheight(d, 100);
        let groupbox_y = button_riferimento_y - propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Seleziona file di input",
        );

        if current_index != Indice::Hfbi {
            if let Some(_filepath) = controller.get_riferimento_path() {
                // A file is already set, display button to clear it
                let rif_itext = d.gui_icon_text(ICON_BIN, "Annulla Riferimento");
                if d.gui_button(
                    rrect(
                        button_riferimento_x,
                        button_riferimento_y,
                        button_riferimento_width,
                        button_riferimento_height,
                    ),
                    rif_itext.as_str(),
                ) {
                    controller.set_riferimento_path(None); // Should already also clear the path_valid
                                                           // state inside it
                }
            } else {
                let rif_itext = d.gui_icon_text(ICON_FILE_OPEN, "Riferimento");
                if d.gui_button(
                    rrect(
                        button_riferimento_x,
                        button_riferimento_y,
                        button_riferimento_width,
                        button_riferimento_height,
                    ),
                    rif_itext.as_str(),
                ) {
                    let file = FileDialog::new()
                        .add_filter("csv", &["csv"])
                        .set_directory("/")
                        .pick_file();

                    if let Some(filepath) = file {
                        controller.set_riferimento_path(Some(filepath));
                    } else {
                        eprintln!("Error: failed getting a file.");
                        controller.add_console_message(
                            "Failed getting a file for riferimento".to_string(),
                        );
                    }
                }
            }
        }

        if let Some(_filepath) = controller.get_campionamento_path() {
            // A file is already set, display button to clear it
            let camp_itext = d.gui_icon_text(ICON_BIN, "Annulla Campionamento");
            if d.gui_button(
                rrect(
                    button_campionamento_x,
                    button_campionamento_y,
                    button_campionamento_width,
                    button_campionamento_height,
                ),
                camp_itext.as_str(),
            ) {
                controller.set_campionamento_path(None); // Should already also clear the path_valid
                                                         // state inside it
            }
        } else {
            let camp_itext = d.gui_icon_text(ICON_FILE_OPEN, "Campionamento");
            if d.gui_button(
                rrect(
                    button_campionamento_x,
                    button_campionamento_y,
                    button_campionamento_width,
                    button_campionamento_height,
                ),
                camp_itext.as_str(),
            ) {
                let file = FileDialog::new()
                    .add_filter("csv", &["csv"])
                    .set_directory("/")
                    .pick_file();

                if let Some(filepath) = file {
                    controller.set_campionamento_path(Some(filepath));
                } else {
                    eprintln!("Error: failed getting a file.");
                    controller
                        .add_console_message("Failed getting a file for campionamento".to_string());
                }
            }
        }
    }
}

impl SelezioneFileInputView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
