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
use raylib::consts::GuiIconName::ICON_OK_TICK;
use raylib::consts::GuiState::{STATE_DISABLED, STATE_NORMAL};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
pub(crate) struct ProduzioneOutputView {}

impl View for ProduzioneOutputView {
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
                eprintln!("ProduzioneOutputView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

        let button_calcola_width = propwidth(d, 200);
        let button_calcola_x = d.get_screen_width() / 2 - button_calcola_width / 2;
        let button_calcola_height = propwidth(d, 50);
        let button_calcola_y = d.get_screen_height() / 4 - button_calcola_height / 2;

        let groupbox_width = button_calcola_width + propwidth(d, 100);
        let groupbox_x = button_calcola_x - propwidth(d, 50);
        let groupbox_height = button_calcola_height + propheight(d, 100);
        let groupbox_y = button_calcola_y - propheight(d, 50);

        let panel_width = groupbox_width + propwidth(d, 175);
        let panel_x = d.get_screen_width() / 2 - panel_width / 2;
        let panel_y = groupbox_y + groupbox_height + propwidth(d, 50);
        let panel_height = groupbox_height + propheight(d, 50);

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Produzione output",
        );

        let mut actions = Vec::<Action>::new();

        if d.gui_button(
            rrect(
                button_calcola_x,
                button_calcola_y,
                button_calcola_width,
                button_calcola_height,
            ),
            "Calcola",
        ) {
            actions.push(RunCalc);
        }

        let submit_width = propwidth(d, 125);
        let panel_x_end = panel_x + panel_width;
        let submit_x = panel_x_end + (d.get_screen_width() - panel_x_end) / 2 - submit_width / 2;
        let submit_height = propheight(d, 50);
        let submit_y = d.get_screen_height() / 2 - submit_height / 2;

        let confirm_itext = d.gui_icon_text(ICON_OK_TICK, "Conferma");

        let done_calc = state.output_model.is_done_calc();

        let _really_done_calc = match current_index {
            // This distinguishes the case where the calc finished but the resulting value is None.
            // It should only really happen when we try to get x2 with a divide-by-zero, as in:
            // our campionamento had no records matching a riferimentore record with specie_attesa == 1
            Indice::Niseci => done_calc && state.data_model.get_niseci_value().is_some(),
            Indice::Hfbi => {
                false //TODO: Implement this
                      //
                      //Maybe the is_some won't be needed
                      //done_calc && controller.get_hfbi_value().is_some()
            }
        };

        let lock_user_submit = !done_calc;

        // If one wanted to avoid users pressing submit when the calc ended in the None case
        // let lock_submit = !done_calc || !really_done_calc;

        if lock_user_submit {
            d.gui_lock();
            d.gui_set_state(STATE_DISABLED);
        }
        if d.gui_button(
            rrect(submit_x, submit_y, submit_width, submit_height),
            confirm_itext.as_str(),
        ) {
            actions.push(ConfirmCalc);
        }
        if lock_user_submit {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }

        d.gui_panel(rrect(panel_x, panel_y, panel_width, panel_height), "Output");

        let y_spacing = main_state.current_font_height + propwidth(d, 5);
        let output_start_y = panel_y + propwidth(d, 15);

        match current_index {
            Indice::Niseci => {
                let niseci_opt = state.data_model.get_niseci_value();
                let niseci_str = match niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            // Could use done_calc and avoid another
                            // lock call
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let niseci_line = format!("NISECI: {}", niseci_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &niseci_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
                let rqe_niseci_opt = state.data_model.get_rqe_niseci_value();
                let rqe_niseci_str = match rqe_niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let rqe_line = format!("RQE NISECI: {}", rqe_niseci_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &rqe_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 2)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
                let stato_eco_niseci_opt = state.data_model.get_stato_eco_niseci_value(state);
                let stato_eco_niseci_str = match stato_eco_niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let stato_eco_line = format!("STATO ECOLOGICO NISECI: {}", stato_eco_niseci_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &stato_eco_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 3)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );

                let x1_opt = state.data_model.get_x1_value();
                let x1_str = match x1_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let x1_line = format!("X1: {}", x1_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &x1_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 4)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );

                let x2_opt = state.data_model.get_x2_value();
                let x2_str = match x2_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let x2_line = format!("X2: {}", x2_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &x2_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 5)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );

                let x3_opt = state.data_model.get_x3_value();
                let x3_str = match x3_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let x3_line = format!("X3: {}", x3_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &x3_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 6)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
            }
            Indice::Hfbi => {
                let hfbi_opt = state.data_model.get_hfbi_value();
                let hfbi_str = match hfbi_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            // Could use done_calc and avoid another
                            // lock call
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let hfbi_line = format!("HFBI: {}", hfbi_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &hfbi_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
                let data_res_opt = state.data_model.get_risultato_hfbi();
                let mmi_str = match data_res_opt {
                    Some(v) => {
                        format!("{}", v.get_intermediates().mmi)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            // Could use done_calc and avoid another
                            // lock call
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let mmi_line = format!("MMI: {}", mmi_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &mmi_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 2)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
                let stato_eco_hfbi_opt = state.data_model.get_stato_eco_hfbi_value(state);
                let stato_eco_hfbi_str = match stato_eco_hfbi_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if state.output_model.is_done_calc() {
                            "NC".to_string()
                        } else {
                            "Non calcolato".to_string()
                        }
                    }
                };
                let stato_eco_line = format!("STATO ECOLOGICO HFBI: {}", stato_eco_hfbi_str);
                d.draw_text_ex(
                    &main_state.current_font,
                    &stato_eco_line,
                    // We use propwidth/height for the text starting position:
                    // this is not the bound
                    Vector2::new(
                        (panel_x + propwidth(d, 25)) as f32,
                        (output_start_y + (y_spacing * 3)) as f32,
                    ),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color,
                );
            }
        }
        actions
    }
}

impl ProduzioneOutputView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
