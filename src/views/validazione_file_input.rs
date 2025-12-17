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
use crate::app::core::{Action, Action::*, MainState};
use crate::app::model::Model;
use esox::domain::index::Indice;
use crate::views::{propheight, propwidth, rrect, View};
use raylib::consts::GuiState::{STATE_DISABLED, STATE_NORMAL};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
use std::process::exit;
pub(crate) struct ValidazioneFileInputView {}

impl View for ValidazioneFileInputView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        state: &Model,
        main_state: &MainState,
    ) -> Vec<Action> {
        d.clear_background(main_state.default_bg_color);

        let current_index = match state.indice_model.get_selected_index() {
            Some(index) => index,
            None => {
                eprintln!("Indice non selezionato");
                exit(1)
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
            "Valida file di input",
        );

        let mut actions = Vec::<Action>::new();

        if current_index != Indice::Hfbi
            && d.gui_button(
                rrect(
                    button_riferimento_x,
                    button_riferimento_y,
                    button_riferimento_width,
                    button_riferimento_height,
                ),
                "Valida Riferimento",
            )
        {
            actions.push(ValidaRiferimentoPath(true));
        }

        let mut turn_off_button_campionamento = false;
        if current_index == Indice::Niseci && !state.fileinput_model.get_riferimento_path_valid() {
            turn_off_button_campionamento = true;
            d.gui_lock();
            d.gui_set_state(STATE_DISABLED);
        }

        if d.gui_button(
            rrect(
                button_campionamento_x,
                button_campionamento_y,
                button_campionamento_width,
                button_campionamento_height,
            ),
            "Valida Campionamento",
        ) {
            actions.push(ValidaCampionamentoPath(true));
        }

        if turn_off_button_campionamento {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }

        actions
    }
}

impl ValidazioneFileInputView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
