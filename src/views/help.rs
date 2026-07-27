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
use raylib::consts::GuiIconName::ICON_PLAYER_NEXT;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::*;
use raylib::RaylibThread;

// A view responsible for rendering the state
// Tightly coupled with its respective controller

pub(crate) struct HelpView {}

impl View for HelpView {
    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        _thread: &RaylibThread,
        state: &Model,
    ) -> Vec<Action> {
        d.clear_background(state.app_model.colors.default_bg_color);

        let txt =
"Fish-Value è un software sviluppato per supportare i professionisti e i tecnici ambientali
nel calcolo degli indici NISECI (Nuovo Indice dello Stato Ecologico delle Comunità Ittiche)
e HFBI (Habitat FishBio-Indicator), strumenti fondamentali per la classificazione dello
stato ecologico dei corpi idrici superficiali sulla base dell'EQB fauna ittica.
\nLa valutazione dello stato ecologico è un requisito previsto dalla normativa italiana in
recepimento della Direttiva Quadro sulle Acque 2000/60/CE, che stabilisce un approccio
integrato per la tutela e la gestione sostenibile delle risorse idriche. In Italia,
l'applicazione della Direttiva è disciplinata dal D.Lgs. 152/2006 e dai successivi
aggiornamenti, che prevedono l'uso di Elementi di Qualità Biologica (EQB), tra cui
l'ittiofauna, per la determinazione della qualità ecologica dei corpi idrici.
\nCosa fa Fish-Value
  -	Calcola in modo automatico e accurato gli indici NISECI e HFBI a partire dai dati
    ittiologici osservati nell'ambito di un campionamento eseguito secondo i protocolli
    descritti da ISPRA (MLG 111/2014 - 2040 e MLG 168/2017).
  -	Fornisce una classificazione dello stato ecologico conforme agli standard normativi
    nazionali tramite un Rapporto di Qualità Ecologica (RQE)".to_string();

        let txt_x = propwidth(d, 25);
        let txt_y = propheight(d, 70);

        d.draw_text_ex(
            &state.app_model.current_font,
            &txt,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(txt_x as f32, txt_y as f32),
            state.app_model.current_font_height as f32,
            state.app_model.default_txt_spacing as f32,
            state.app_model.colors.default_txt_color,
        );

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width() - continue_width - propwidth(d, 50);
        let continue_height = propwidth(d, 50);
        let continue_y = d.get_screen_height() - propheight(d, 100);

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        let mut actions = Vec::<Action>::new();

        if d.gui_button(
            rrect(continue_x, continue_y, continue_width, continue_height),
            continue_itext.as_str(),
        ) {
            actions.push(UserContinued);
        }
        actions
    }
}

impl HelpView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
