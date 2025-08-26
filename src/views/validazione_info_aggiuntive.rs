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
use crate::controllers::{info_aggiuntive::InfoAggiuntiveController, Controller};
use crate::domain::index::Indice;
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use raylib::consts::GuiIconName::ICON_CROSS;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
pub(crate) struct ValidazioneInfoAggiuntiveView {}

impl View for ValidazioneInfoAggiuntiveView {
    type Controller = InfoAggiuntiveController;

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
                eprintln!("ValidazioneInfoAggiuntiveView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

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

        if d.gui_button(
            rrect(
                button_valida_x,
                button_valida_y,
                button_valida_width,
                button_valida_height,
            ),
            "Valida info aggiuntive",
        ) {
            //Ask controller to validate info aggiuntive indice
            match current_index {
                Indice::Niseci => {
                    controller.valida_anagrafica_niseci();
                }
                Indice::Hfbi => controller.valida_anagrafica_hfbi(),
            }
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
            //Ask controller to go back and edit further
            match current_index {
                Indice::Niseci => {
                    controller.backout_anagrafica_niseci();
                }
                Indice::Hfbi => {
                    controller.backout_anagrafica_hfbi();
                }
            }
        }
    }
}

impl ValidazioneInfoAggiuntiveView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
