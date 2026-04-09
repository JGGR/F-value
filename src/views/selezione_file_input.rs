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
use esox::domain::index::Indice;
use raylib::consts::GuiIconName::{ICON_BIN, ICON_FILE_OPEN};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
use rfd::FileDialog;
pub(crate) struct SelezioneFileInputView {}

impl View for SelezioneFileInputView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        self.draw_background(d, main_state);

        let current_index = match state.indice_model.get_selected_index() {
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

        let mut actions = Vec::<Action>::new();

        if current_index != Indice::Hfbi {
            if let Some(_filepath) = state.fileinput_model.get_riferimento_path() {
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
                    actions.push(PickRiferimentoPath(None));
                    //controller.set_riferimento_path(None); // Should already also clear the path_valid
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
                        actions.push(PickRiferimentoPath(Some(filepath)));
                    } else {
                        eprintln!("Error: failed getting a file.");
                    }
                }
            }
        }

        if let Some(_filepath) = state.fileinput_model.get_campionamento_path() {
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
                actions.push(PickCampionamentoPath(None));
                //controller.set_campionamento_path(None); // Should already also clear the path_valid
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
                    actions.push(PickCampionamentoPath(Some(filepath)));
                } else {
                    eprintln!("Error: failed getting a file.");
                }
            }
        }

        actions
    }
}

impl SelezioneFileInputView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
