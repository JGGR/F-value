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
use crate::app::core::{Action, Action::*, RegioneItaliana};
use crate::app::model::Model;
use crate::views::{propheight, propwidth, rrect, View};
use crate::MainState;
use esox::domain::{
    hfbi::{AnagraficaHFBIDraft, HabitatHFBI, StagioneHFBI, TipoLagunaCostieraHFBI},
    index::Indice,
    location::Location,
    niseci::{
        AnagraficaNISECIDraft, AreaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI, TipoComunitaNISECI,
    },
};
use raylib::consts::GuiIconName::ICON_OK_TICK;
use raylib::consts::GuiState::{STATE_DISABLED, STATE_NORMAL};
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::*;
use raylib::RaylibThread;
pub(crate) struct SelezioneInfoAggiuntiveView {
    textbox_codice_stazione_edit_mode: bool,
    textbox_codice_stazione_buffer: String,
    textbox_corpo_idrico_edit_mode: bool,
    textbox_corpo_idrico_buffer: String,
    listview_regione_value: i32,
    listview_regione_scroll_value: i32,
    textbox_provincia_edit_mode: bool,
    textbox_provincia_buffer: String,
    textbox_data_edit_mode: bool,
    textbox_data_buffer: String,
    textbox_lunghezza_stazione_edit_mode: bool,
    textbox_lunghezza_stazione_buffer: String,
    textbox_larghezza_stazione_edit_mode: bool,
    textbox_larghezza_stazione_buffer: String,
    dropdownbox_tipocomunit_niseci_edit_mode: bool,
    dropdownbox_tipocomunit_niseci_value: i32,
    textbox_fontecomunit_niseci_edit_mode: bool,
    textbox_fontecomunit_niseci_buffer: String,
    textbox_protocollocomunit_niseci_edit_mode: bool,
    textbox_protocollocomunit_niseci_buffer: String,
    listview_idroecoregione_niseci_value: i32,
    listview_idroecoregione_niseci_scroll_value: i32,
    combobox_area_niseci_value: i32,
    textbox_bacino_niseci_edit_mode: bool,
    textbox_bacino_niseci_buffer: String,
    combobox_stagione_hfbi_value: i32,
    combobox_habitat_vegetato_hfbi_value: i32,
    dropdownbox_tipolaguna_hfbi_edit_mode: bool,
    dropdownbox_tipolaguna_hfbi_value: i32,
}

impl View for SelezioneInfoAggiuntiveView {
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
                eprintln!("SelezioneInfoAggiuntiveView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

        let groupbox_width = propwidth(d, 750);
        let groupbox_x = d.get_screen_width() / 2 - groupbox_width / 2;
        let groupbox_height = propheight(d, 450);
        let groupbox_y = d.get_screen_height() / 2 - groupbox_height / 2;

        d.gui_group_box(
            rrect(groupbox_x, groupbox_y, groupbox_width, groupbox_height),
            "Inserisci informazioni aggiuntive",
        );

        let x_padding = groupbox_width / 20;
        let y_padding = groupbox_height / 15;

        // 2 columns: 2 paddings per side + 1 between the columns
        let column_width = (groupbox_width - (x_padding * 3)) / 2;
        let column_1_x = groupbox_x + x_padding;
        let column_2_x = column_1_x + column_width + x_padding;
        let column_1_y = groupbox_y + y_padding;
        let column_2_y = column_1_y;
        // 1 padding on top, 2 below
        let column_1_height = groupbox_height - y_padding * 3;
        // 1 padding on top, 0 below
        let column_2_height = groupbox_height - y_padding;

        // Column 1

        let column_1_labels_width = column_width / 2;
        let column_1_fields_y_spacing = y_padding / 3;
        let column_1_labels_count = 7;
        let column_1_labels_x = column_1_x;
        let column_1_labels_height =
            (column_1_height - (column_1_fields_y_spacing * 13)) / column_1_labels_count;

        let column_1_label_stazione_y = column_1_y;
        let column_1_label_corpo_idrico_y =
            column_1_label_stazione_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_regione_y =
            column_1_label_corpo_idrico_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_provincia_y =
            column_1_label_regione_y + column_1_labels_height + column_1_fields_y_spacing * 8;
        let column_1_label_data_y =
            column_1_label_provincia_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_lunghezza_stazione_y =
            column_1_label_data_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_larghezza_stazione_y = column_1_label_lunghezza_stazione_y
            + column_1_labels_height
            + column_1_fields_y_spacing;

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_stazione_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            "Codice stazione",
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_corpo_idrico_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            "Nome del corpo idrico",
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_regione_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            "Regione",
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_provincia_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            "Provincia",
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_data_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            "Data",
        );

        let label_for_station_length = match current_index {
            Indice::Niseci => "Lunghezza stazione",
            Indice::Hfbi => "Lunghezza transetto",
        };
        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_lunghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            label_for_station_length,
        );

        let label_for_station_width = match current_index {
            Indice::Niseci => "Larghezza stazione",
            Indice::Hfbi => "Larghezza transetto",
        };

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_larghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height,
            ),
            label_for_station_width,
        );

        let column_1_boxes_width = column_1_labels_width;
        let column_1_boxes_height = column_1_labels_height;
        let column_1_boxes_x = column_1_labels_x + column_1_labels_width;

        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_codice_stazione_buffer,
            self.textbox_codice_stazione_edit_mode,
        ) {
            self.textbox_codice_stazione_edit_mode = !self.textbox_codice_stazione_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_corpo_idrico_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_corpo_idrico_buffer,
            self.textbox_corpo_idrico_edit_mode,
        ) {
            self.textbox_corpo_idrico_edit_mode = !self.textbox_corpo_idrico_edit_mode;
        }

        let mut _listview_regione_italiana_pick = -1;
        _listview_regione_italiana_pick = d.gui_list_view(
            rrect(
                column_1_boxes_x,
                column_1_label_regione_y,
                column_1_boxes_width,
                column_1_boxes_height * 3,
            ),
            RegioneItaliana::COMBOBOX_STR,
            &mut self.listview_regione_scroll_value,
            &mut self.listview_regione_value,
        );
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_provincia_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_provincia_buffer,
            self.textbox_provincia_edit_mode,
        ) {
            self.textbox_provincia_edit_mode = !self.textbox_provincia_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_data_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_data_buffer,
            self.textbox_data_edit_mode,
        ) {
            self.textbox_data_edit_mode = !self.textbox_data_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_lunghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_lunghezza_stazione_buffer,
            self.textbox_lunghezza_stazione_edit_mode,
        ) {
            self.textbox_lunghezza_stazione_edit_mode = !self.textbox_lunghezza_stazione_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_larghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height,
            ),
            &mut self.textbox_larghezza_stazione_buffer,
            self.textbox_larghezza_stazione_edit_mode,
        ) {
            self.textbox_larghezza_stazione_edit_mode = !self.textbox_larghezza_stazione_edit_mode;
        }
        // Column 2

        // spacing between the two groupboxes
        let column_2_groupbox_y_padding = y_padding / 2;
        let column_2_groupbox_niseci_width = column_width;
        let column_2_groupbox_niseci_x = column_2_x;
        let column_2_groupbox_niseci_y = column_2_y;
        let column_2_groupbox_niseci_height = column_2_height - column_2_groupbox_y_padding / 2; // - y_padding;
        let column_2_groupbox_hfbi_width = column_2_groupbox_niseci_width;
        let column_2_groupbox_hfbi_x = column_2_groupbox_niseci_x;
        //let column_2_groupbox_hfbi_y = column_2_groupbox_niseci_y + column_2_groupbox_niseci_height + column_2_groupbox_y_padding;
        //let column_2_groupbox_hfbi_height = column_2_height - column_2_groupbox_niseci_height;
        let column_2_groupbox_hfbi_y = column_2_groupbox_niseci_y;
        let column_2_groupbox_hfbi_height =
            column_2_groupbox_niseci_height - column_2_groupbox_y_padding - y_padding;

        let column_2_comunit_x_padding = x_padding / 4;
        let column_2_comunit_y_padding = column_2_groupbox_y_padding;
        let column_2_groupbox_comunit_x = column_2_groupbox_niseci_x + column_2_comunit_x_padding;
        let column_2_groupbox_comunit_y = column_2_groupbox_niseci_y + column_2_comunit_y_padding;
        let column_2_groupbox_comunit_width =
            column_2_groupbox_niseci_width - column_2_comunit_x_padding * 2;
        let column_2_groupbox_comunit_height =
            column_1_labels_height * 3 + column_1_fields_y_spacing * 4;

        match current_index {
            Indice::Niseci => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_niseci_x,
                        column_2_groupbox_niseci_y,
                        column_2_groupbox_niseci_width,
                        column_2_groupbox_niseci_height,
                    ),
                    "NISECI",
                );

                d.gui_group_box(
                    rrect(
                        column_2_groupbox_comunit_x,
                        column_2_groupbox_comunit_y,
                        column_2_groupbox_comunit_width,
                        column_2_groupbox_comunit_height,
                    ),
                    "Comunità NISECI",
                );
            }
            Indice::Hfbi => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_hfbi_x,
                        column_2_groupbox_hfbi_y,
                        column_2_groupbox_hfbi_width,
                        column_2_groupbox_hfbi_height,
                    ),
                    "HFBI",
                );
            }
        }

        let column_2_groupbox_labels_x_spacing = column_2_comunit_x_padding;
        let column_2_groupbox_labels_width =
            (column_2_groupbox_comunit_width - column_2_groupbox_labels_x_spacing * 2) / 2;
        let column_2_groupbox_fields_y_spacing = column_1_fields_y_spacing;
        let column_2_groupbox_labels_x =
            column_2_groupbox_comunit_x + column_2_groupbox_labels_x_spacing;
        let column_2_labels_height = column_1_labels_height;

        match current_index {
            Indice::Niseci => {
                let column_2_label_tipo_comunit_y =
                    column_2_groupbox_comunit_y + column_2_groupbox_fields_y_spacing;
                let column_2_label_fonte_comunit_y = column_2_label_tipo_comunit_y
                    + column_2_labels_height
                    + column_2_groupbox_fields_y_spacing;
                let column_2_label_protocollo_comunit_y = column_2_label_fonte_comunit_y
                    + column_2_labels_height
                    + column_2_groupbox_fields_y_spacing;
                let column_2_label_idroecoregione_y = column_2_groupbox_comunit_y
                    + column_2_groupbox_comunit_height
                    + column_2_groupbox_fields_y_spacing;
                let column_2_label_area_niseci_y = column_2_label_idroecoregione_y
                    + column_2_labels_height * 4
                    + column_2_groupbox_fields_y_spacing;
                let column_2_label_bacino_y = column_2_label_area_niseci_y
                    + column_2_labels_height
                    + column_2_groupbox_fields_y_spacing;

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_tipo_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Tipo",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_fonte_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Fonte",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_protocollo_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Protocollo",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_idroecoregione_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Idroecoregione",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Area",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_bacino_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Bacino",
                );

                let column_2_groupbox_boxes_width = column_2_groupbox_labels_width;
                let column_2_groupbox_boxes_height = column_2_labels_height;
                let column_2_groupbox_boxes_x =
                    column_2_groupbox_labels_x + column_2_groupbox_labels_width;

                match self.dropdownbox_tipocomunit_niseci_value {
                    1 => { /* 1 == Fonte I guess */ }
                    _ => {
                        // Not fonte I guess
                        d.gui_lock();
                        d.gui_set_state(STATE_DISABLED);
                    }
                }
                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_fonte_comunit_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    &mut self.textbox_fontecomunit_niseci_buffer,
                    self.textbox_fontecomunit_niseci_edit_mode,
                ) {
                    self.textbox_fontecomunit_niseci_edit_mode =
                        !self.textbox_fontecomunit_niseci_edit_mode;
                }
                match self.dropdownbox_tipocomunit_niseci_value {
                    1 => { /* 1 == Fonte I guess */ }
                    _ => {
                        // Not fonte I guess
                        d.gui_set_state(STATE_NORMAL);
                        d.gui_unlock();
                    }
                }

                match self.dropdownbox_tipocomunit_niseci_value {
                    3 => { /* 3 == Mase I guess */ }
                    _ => {
                        // Not Mase I guess
                        d.gui_lock();
                        d.gui_set_state(STATE_DISABLED);
                    }
                }
                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_protocollo_comunit_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    &mut self.textbox_protocollocomunit_niseci_buffer,
                    self.textbox_protocollocomunit_niseci_edit_mode,
                ) {
                    self.textbox_protocollocomunit_niseci_edit_mode =
                        !self.textbox_protocollocomunit_niseci_edit_mode;
                }
                match self.dropdownbox_tipocomunit_niseci_value {
                    3 => { /* 3 == Mase I guess */ }
                    _ => {
                        // Not Mase I guess
                        d.gui_set_state(STATE_NORMAL);
                        d.gui_unlock();
                    }
                }

                let mut _listview_idroecoregione_pick = -1;
                _listview_idroecoregione_pick = d.gui_list_view(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_idroecoregione_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height * 4,
                    ),
                    "1-AlpiOccidentali;2-Prealpi Dolomiti;3-Alpi Centro-Orientali;\
                    4-Alpi Meridionali;5-Monferrato;6-Pianura Padana;7-Carso;\
                    8-Appennino Piemontese;9-Alpi Mediterranee;10-Appennino Piemontese;\
                    11-Toscana;12-Costa Adriatica;13-Appennino Centrale;14-Roma-Viterbese;\
                    15-Basso Lazio;16-Vesuvio;17-Basilicata-Tavoliere;18-Puglia-Carsica;\
                    19-Appennino Meridionale;20-Sicilia;21-Sardegna",
                    &mut self.listview_idroecoregione_niseci_scroll_value,
                    &mut self.listview_idroecoregione_niseci_value,
                );
                let mut _combo_box_pick = -1;
                _combo_box_pick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    "Alpina;Mediterranea",
                    &mut self.combobox_area_niseci_value,
                );

                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_bacino_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    &mut self.textbox_bacino_niseci_buffer,
                    self.textbox_bacino_niseci_edit_mode,
                ) {
                    self.textbox_bacino_niseci_edit_mode = !self.textbox_bacino_niseci_edit_mode;
                }

                if d.gui_dropdown_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_tipo_comunit_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    "Redatta;Fonti;DM260/2010;Mase",
                    &mut self.dropdownbox_tipocomunit_niseci_value,
                    self.dropdownbox_tipocomunit_niseci_edit_mode,
                ) {
                    self.dropdownbox_tipocomunit_niseci_edit_mode =
                        !self.dropdownbox_tipocomunit_niseci_edit_mode;
                }
            }
            Indice::Hfbi => {
                let column_2_label_stagione_y =
                    column_2_groupbox_comunit_y + column_2_groupbox_fields_y_spacing;
                let column_2_label_habitat_vegetato_y = column_2_label_stagione_y
                    + column_2_labels_height
                    + column_2_groupbox_fields_y_spacing;
                let column_2_label_tipo_laguna_y = column_2_label_habitat_vegetato_y
                    + column_2_labels_height
                    + column_2_groupbox_fields_y_spacing;
                let column_2_groupbox_boxes_width = column_2_groupbox_labels_width;
                let column_2_groupbox_boxes_height = column_2_labels_height;
                let column_2_groupbox_boxes_x =
                    column_2_groupbox_labels_x + column_2_groupbox_labels_width;

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_stagione_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Stagione",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_habitat_vegetato_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Habitat",
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_tipo_laguna_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height,
                    ),
                    "Tipo Laguna",
                );

                let mut _combo_box_stagione_pick = -1;
                _combo_box_stagione_pick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_stagione_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    "Primavera;Autunno",
                    &mut self.combobox_stagione_hfbi_value,
                );

                let mut _combo_box_habitat_vegetato_pick = -1;
                _combo_box_habitat_vegetato_pick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_habitat_vegetato_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    "Vegetato;Non vegetato",
                    &mut self.combobox_habitat_vegetato_hfbi_value,
                );

                if d.gui_dropdown_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_tipo_laguna_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height,
                    ),
                    "M-AT-1;M-AT-2;M-AT-3",
                    &mut self.dropdownbox_tipolaguna_hfbi_value,
                    self.dropdownbox_tipolaguna_hfbi_edit_mode,
                ) {
                    self.dropdownbox_tipolaguna_hfbi_edit_mode =
                        !self.dropdownbox_tipolaguna_hfbi_edit_mode;
                }
            }
        }
        let submit_width = propwidth(d, 120);
        let groupbox_x_end = groupbox_x + groupbox_width;
        let column_2_groupbox_x_end = column_2_x + column_width;
        let submit_x = column_2_groupbox_x_end + (groupbox_x_end - column_2_groupbox_x_end) / 2;
        let submit_height = propheight(d, 50);
        let submit_y = d.get_screen_height() / 2 - submit_height / 2;

        let confirm_itext = d.gui_icon_text(ICON_OK_TICK, "Conferma");
        let mut actions = Vec::<Action>::new();
        if d.gui_button(
            rrect(submit_x, submit_y, submit_width, submit_height),
            confirm_itext.as_str(),
        ) {
            let regione =
                match <RegioneItaliana as TryFrom<i32>>::try_from(self.listview_regione_value) {
                    Ok(r) => r,
                    Err(_) => {
                        panic!("Unexpected regione_string in SelezioneInfoAggiuntiveView::draw()");
                    }
                };
            let regione_string = regione.to_string();

            let provincia_string = String::from(&self.textbox_provincia_buffer);
            let posizione = Location {
                regione: regione_string,
                provincia: provincia_string,
            };

            let larghezza_stazione_str = String::from(&self.textbox_larghezza_stazione_buffer);

            let lunghezza_stazione_str = String::from(&self.textbox_lunghezza_stazione_buffer);

            let codice_stazione = String::from(&self.textbox_codice_stazione_buffer);
            let date_string = String::from(&self.textbox_data_buffer);

            let corpo_idrico = String::from(&self.textbox_corpo_idrico_buffer);
            match current_index {
                Indice::Niseci => {
                    let tipo_comunita = match <TipoComunitaNISECI as TryFrom<i32>>::try_from(
                        self.dropdownbox_tipocomunit_niseci_value,
                    ) {
                        Ok(v) => v,
                        _ => {
                            panic!(
                                "Unexpected tipo_comunita in SelezioneInfoAggiuntiveView::draw()"
                            );
                        }
                    };
                    let mut opt_fonte: Option<String> = None;
                    let mut opt_num_protocollo: Option<String> = None;
                    match tipo_comunita {
                        TipoComunitaNISECI::Recuperata => {
                            // Raylib has trouble handling the string downstream if we don't ensure to do this
                            opt_fonte = Some(self.textbox_fontecomunit_niseci_buffer.clone());
                        }
                        TipoComunitaNISECI::AffinataDalMase => {
                            // Raylib has trouble handling the string downstream if we don't ensure to do this
                            opt_num_protocollo =
                                Some(self.textbox_protocollocomunit_niseci_buffer.clone());
                        }
                        _ => {}
                    }
                    let comunita = ComunitaNISECI {
                        tipo: tipo_comunita,
                        fonte: opt_fonte,
                        numero_protocollo: opt_num_protocollo,
                    };
                    let area = match <AreaNISECI as TryFrom<i32>>::try_from(
                        self.combobox_area_niseci_value,
                    ) {
                        Ok(v) => v,
                        _ => {
                            panic!("Unexpected area_niseci in SelezioneInfoAggiuntiveView::draw()");
                        }
                    };

                    let bacino_niseci = String::from(&self.textbox_bacino_niseci_buffer);

                    let idro_ecoregione_niseci =
                        match <IdroEcoRegioneNISECI as TryFrom<i32>>::try_from(
                            self.listview_idroecoregione_niseci_value,
                        ) {
                            Ok(v) => v,
                            _ => {
                                panic!("Unexpected idroecoregione_niseci in SelezioneInfoAggiuntiveView::draw()");
                            }
                        };

                    let anagrafica = AnagraficaNISECIDraft {
                        comunita,
                        codice_stazione,
                        date_string,
                        area,
                        corpo_idrico,
                        bacino_appartenenza: bacino_niseci,
                        idro_eco_regione: idro_ecoregione_niseci,
                        posizione,
                        lunghezza_media_stazione: lunghezza_stazione_str,
                        larghezza_media_stazione: larghezza_stazione_str,
                    };

                    actions.push(SubmitAnagraficaNISECI(anagrafica));
                }
                Indice::Hfbi => {
                    let tipo_laguna_costiera =
                        match <TipoLagunaCostieraHFBI as TryFrom<i32>>::try_from(
                            self.dropdownbox_tipolaguna_hfbi_value,
                        ) {
                            Ok(v) => v,
                            _ => {
                                panic!(
                                    "Unexpected tipo_laguna in SelezioneInfoAggiuntiveView::draw()"
                                );
                            }
                        };
                    let stagione = match <StagioneHFBI as TryFrom<i32>>::try_from(
                        self.combobox_stagione_hfbi_value,
                    ) {
                        Ok(v) => v,
                        _ => {
                            panic!("Unexpected stagione in SelezioneInfoAggiuntiveView::draw()");
                        }
                    };
                    let habitat_vegetato = match <HabitatHFBI as TryFrom<i32>>::try_from(
                        self.combobox_habitat_vegetato_hfbi_value,
                    ) {
                        Ok(v) => v,
                        _ => {
                            panic!("Unexpected habitat in SelezioneInfoAggiuntiveView::draw()");
                        }
                    };
                    let anagrafica = AnagraficaHFBIDraft {
                        codice_stazione,
                        corpo_idrico,
                        posizione,
                        date_string,
                        tipo_laguna: tipo_laguna_costiera,
                        stagione,
                        habitat_vegetato,
                        lunghezza_media_transetto: lunghezza_stazione_str,
                        larghezza_media_transetto: larghezza_stazione_str,
                    };
                    actions.push(SubmitAnagraficaHFBI(anagrafica));
                }
            }
        }
        actions
    }
}

impl SelezioneInfoAggiuntiveView {
    pub(crate) fn new() -> Self {
        let codice_stazione_buffer = String::with_capacity(250);
        let corpo_idrico_buffer = String::with_capacity(250);

        let provincia_buffer = String::with_capacity(250);

        let data_buffer = String::with_capacity(25);

        let lunghezza_stazione_buffer = String::with_capacity(250);

        let larghezza_stazione_buffer = String::with_capacity(250);

        let fonte_comunit_buffer = String::with_capacity(250);

        let protocollo_comunit_buffer = String::with_capacity(250);

        let bacino_buffer = String::with_capacity(250);

        Self {
            textbox_codice_stazione_edit_mode: false,
            textbox_codice_stazione_buffer: codice_stazione_buffer,
            textbox_corpo_idrico_edit_mode: false,
            textbox_corpo_idrico_buffer: corpo_idrico_buffer,
            listview_regione_value: 0,
            listview_regione_scroll_value: 0,
            textbox_provincia_edit_mode: false,
            textbox_provincia_buffer: provincia_buffer,
            textbox_data_edit_mode: false,
            textbox_data_buffer: data_buffer,
            textbox_lunghezza_stazione_edit_mode: false,
            textbox_lunghezza_stazione_buffer: lunghezza_stazione_buffer,
            textbox_larghezza_stazione_edit_mode: false,
            textbox_larghezza_stazione_buffer: larghezza_stazione_buffer,
            dropdownbox_tipocomunit_niseci_edit_mode: false,
            dropdownbox_tipocomunit_niseci_value: 0,
            textbox_fontecomunit_niseci_edit_mode: false,
            textbox_fontecomunit_niseci_buffer: fonte_comunit_buffer,
            textbox_protocollocomunit_niseci_edit_mode: false,
            textbox_protocollocomunit_niseci_buffer: protocollo_comunit_buffer,
            listview_idroecoregione_niseci_value: 0,
            listview_idroecoregione_niseci_scroll_value: 0,
            combobox_area_niseci_value: 0,
            textbox_bacino_niseci_edit_mode: false,
            textbox_bacino_niseci_buffer: bacino_buffer,
            combobox_stagione_hfbi_value: 0,
            combobox_habitat_vegetato_hfbi_value: 0,
            dropdownbox_tipolaguna_hfbi_edit_mode: false,
            dropdownbox_tipolaguna_hfbi_value: 0,
        }
    }
}
