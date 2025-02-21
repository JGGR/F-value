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

use std::process::exit;

use crate::core::*;
use crate::controllers::*;
use crate::model::index::Indice;
use crate::model::location::Location;
use crate::model::niseci::TipoComunitaNISECI;
use crate::model::niseci::ComunitaNISECI;
use crate::model::niseci::AreaNISECI;
use crate::model::niseci::IdroEcoRegioneNISECI;
use crate::model::niseci::AnagraficaNISECI;
use raylib::prelude::*;
use rfd::FileDialog;
use raylib::consts::GuiState::{STATE_NORMAL, STATE_DISABLED};
use raylib::consts::GuiIconName::{ICON_FILE_OPEN, ICON_BIN, ICON_OK_TICK, ICON_CROSS};
use std::ffi::CString;

// A view responsible for rendering the state
// Tightly coupled with its respective controller
pub struct HomeView {
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl HomeView {
    pub fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &HomeController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(&d, 100) as f32, propheight(&d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color
        );


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 100), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
            None,
            &mut self.spinner_value,
            0,
            100,
            self.spinner_edit_mode,
        );
        if updated_spinner {
            self.spinner_edit_mode = !self.spinner_edit_mode;
        }

        // gui_value_box() (and gui_spinner() too since it's used by it. The "value" argument
        // must be a value living for the whole draw loop, so we just dup them
        // to the View and ensure to set them on all frames to the model via
        // the controller.
        controller.set_value(self.spinner_value);
    }
}

pub struct SecondView {
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl SecondView {
    pub fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &SecondController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(&d, 100) as f32, propheight(&d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color
        );


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(&d, 100), propheight(&d, 50), propwidth(&d, 125), propheight(&d, 30)),
            None,
            &mut self.spinner_value,
            0,
            100,
            self.spinner_edit_mode,
        );
        if updated_spinner {
            self.spinner_edit_mode = !self.spinner_edit_mode;
        }

        // gui_value_box() (and gui_spinner() too since it's used by it. The "value" argument
        // must be a value living for the whole draw loop, so we just dup them
        // to the View and ensure to set them on all frames to the model via
        // the controller.
        controller.set_value(self.spinner_value);
    }
}

pub struct SelezioneIndiceView {

}

impl SelezioneIndiceView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &IndiceController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let button_niseci_width = propwidth(&d, 200);
        let button_niseci_x = d.get_screen_width() / 2 - button_niseci_width /2;
        let button_niseci_height = propwidth(&d, 50);

        let button_indice_y_spacing = button_niseci_height;

        let button_niseci_y = d.get_screen_height() / 2 - button_indice_y_spacing / 2 - button_niseci_height;

        let button_hfbi_width = button_niseci_width;
        let button_hfbi_x = button_niseci_x;
        let button_hfbi_height = button_niseci_height;
        let button_hfbi_y = button_niseci_y + button_niseci_height + button_indice_y_spacing;

        let groupbox_width = button_niseci_width + propwidth(&d, 100);
        let groupbox_x = button_niseci_x - propwidth(&d, 50);
        let groupbox_height = button_niseci_height * 2 + button_indice_y_spacing + propheight(&d, 100);
        let groupbox_y = button_niseci_y - propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Seleziona Indice"))
        );

        if d.gui_button(
            rrect(
                button_niseci_x,
                button_niseci_y,
                button_niseci_width,
                button_niseci_height
            ),
            Some(rstr!("NISECI"))
        ) {
            controller.set_indice_corrente(Indice::NISECI);
        }

        if d.gui_button(
            rrect(
                button_hfbi_x,
                button_hfbi_y,
                button_hfbi_width,
                button_hfbi_height,
            ),
            Some(rstr!("HFBI"))
        ) {
            controller.set_indice_corrente(Indice::HFBI);
        }
    }
}

pub struct SelezioneFileInputView {

}

impl SelezioneFileInputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("Indice non selezionato");
                exit(1)
            }
        };

        let button_riferimento_width = propwidth(&d, 200);
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width /2;
        let button_riferimento_height = propwidth(&d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y = d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = match current_index {
            Indice::HFBI => button_riferimento_y + button_fileinput_y_spacing,
            Indice::NISECI => button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing,
        };

        let groupbox_width = button_riferimento_width + propwidth(&d, 100);
        let groupbox_x = button_riferimento_x - propwidth(&d, 50);
        let groupbox_height = button_riferimento_height * 2 + button_fileinput_y_spacing + propheight(&d, 100);
        let groupbox_y = button_riferimento_y - propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Seleziona file di input"))
        );

        if current_index != Indice::HFBI {
            if let Some(_filepath) = controller.get_riferimento_path() { // A file is already set, display button to clear it
                let rif_itext = d.gui_icon_text(ICON_BIN, Some(rstr!("Annulla Riferimento")));
                let rif_itext = CString::new(rif_itext).unwrap();
                if d.gui_button(
                    rrect(
                        button_riferimento_x,
                        button_riferimento_y,
                        button_riferimento_width,
                        button_riferimento_height
                    ),
                    Some(rif_itext.as_c_str())
                ) {
                    controller.set_riferimento_path(None); // Should already also clear the path_valid
                                                           // state inside it
                }
            } else {
                let rif_itext = d.gui_icon_text(ICON_FILE_OPEN, Some(rstr!("Riferimento")));
                let rif_itext = CString::new(rif_itext).unwrap();
                if d.gui_button(
                    rrect(
                        button_riferimento_x,
                        button_riferimento_y,
                        button_riferimento_width,
                        button_riferimento_height
                    ),
                    Some(rif_itext.as_c_str())
                ) {
                    let file = FileDialog::new()
                            .add_filter("csv", &["csv"])
                            .set_directory("/")
                            .pick_file();

                    if let Some(filepath) = file {
                        controller.set_riferimento_path(Some(filepath));
                    } else {
                        eprintln!("Error: failed getting a file.");
                        controller.add_console_message("Failed getting a file for riferimento".to_string());
                    }
                }
            }
        }

        if let Some(_filepath) = controller.get_campionamento_path() { // A file is already set, display button to clear it
            let camp_itext = d.gui_icon_text(ICON_BIN, Some(rstr!("Annulla Campionamento")));
            let camp_itext = CString::new(camp_itext).unwrap();
            if d.gui_button(
                rrect(
                    button_campionamento_x,
                    button_campionamento_y,
                    button_campionamento_width,
                    button_campionamento_height,
                ),
                Some(camp_itext.as_c_str())
            ) {
                controller.set_campionamento_path(None); // Should already also clear the path_valid
                                                       // state inside it
            }
        } else {
            let camp_itext = d.gui_icon_text(ICON_FILE_OPEN, Some(rstr!("Campionamento")));
            let camp_itext = CString::new(camp_itext).unwrap();
            if d.gui_button(
                rrect(
                    button_campionamento_x,
                    button_campionamento_y,
                    button_campionamento_width,
                    button_campionamento_height,
                ),
                Some(camp_itext.as_c_str())
            ) {
                let file = FileDialog::new()
                        .add_filter("csv", &["csv"])
                        .set_directory("/")
                        .pick_file();

                if let Some(filepath) = file {
                    controller.set_campionamento_path(Some(filepath));
                } else {
                    eprintln!("Error: failed getting a file.");
                    controller.add_console_message("Failed getting a file for campionamento".to_string());
                }
            }
        }
    }
}

pub struct ValidazioneFileInputView {

}

impl ValidazioneFileInputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &FileInputController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("Indice non selezionato");
                exit(1)
            }
        };

        let button_riferimento_width = propwidth(&d, 200);
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width /2;
        let button_riferimento_height = propwidth(&d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y = d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = match current_index {
            Indice::HFBI => button_riferimento_y + button_fileinput_y_spacing,
            Indice::NISECI => button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing,
        };

        let groupbox_width = button_riferimento_width + propwidth(&d, 100);
        let groupbox_x = button_riferimento_x - propwidth(&d, 50);
        let groupbox_height = button_riferimento_height * 2 + button_fileinput_y_spacing + propheight(&d, 100);
        let groupbox_y = button_riferimento_y - propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Valida file di input"))
        );

        if current_index != Indice::HFBI {
            if d.gui_button(
                rrect(
                    button_riferimento_x,
                    button_riferimento_y,
                    button_riferimento_width,
                    button_riferimento_height
                ),
                Some(rstr!("Valida Riferimento"))
            ) {
                controller.valida_riferimento_niseci_path();
            }
        }

        let mut turn_off_button_campionamento = false;
        if current_index == Indice::NISECI && !controller.get_riferimento_path_valid() {
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
            Some(rstr!("Valida Campionamento"))
        ) {
            match current_index {
                Indice::NISECI => {
                    controller.valida_campionamento_niseci_path();
                }
                Indice::HFBI => {
                    todo!("Implement controller.valida_campionamento_hfbi_path()");
                    // controller.valida_campionamento_hfbi_path();
                }
            }
        }

        if turn_off_button_campionamento {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }
    }
}

pub struct SelezioneInfoAggiuntiveView {
    valuebox_codice_stazione_edit_mode: bool,
    valuebox_codice_stazione_value: i32,
    textbox_corpo_idrico_edit_mode: bool,
    textbox_corpo_idrico_buffer: [u8; 64],
    listview_regione_value: i32,
    listview_regione_scroll_value: i32,
    textbox_provincia_edit_mode: bool,
    textbox_provincia_buffer: [u8; 64],
    textbox_data_edit_mode: bool,
    textbox_data_buffer: [u8; 64],
    valuebox_lunghezza_stazione_edit_mode: bool,
    valuebox_lunghezza_stazione_value: i32,
    valuebox_larghezza_stazione_edit_mode: bool,
    valuebox_larghezza_stazione_value: i32,
    dropdownbox_tipocomunit_niseci_edit_mode: bool,
    dropdownbox_tipocomunit_niseci_value: i32,
    textbox_fontecomunit_niseci_edit_mode: bool,
    textbox_fontecomunit_niseci_buffer: [u8; 64],
    textbox_protocollocomunit_niseci_edit_mode: bool,
    textbox_protocollocomunit_niseci_buffer: [u8; 64],
    listview_idroecoregione_niseci_value: i32,
    listview_idroecoregione_niseci_scroll_value: i32,
    combobox_area_niseci_value: i32,
    textbox_bacino_niseci_edit_mode: bool,
    textbox_bacino_niseci_buffer: [u8; 64],
}

impl SelezioneInfoAggiuntiveView {

    pub fn new() -> Self {
        let mut corpo_idrico_buffer = [0u8; 64];
        let corpo_idrico_buffer_bytes = "Inserisci nome".as_bytes();
        let corpo_idrico_buffer_len = corpo_idrico_buffer_bytes.len().min(64);
        corpo_idrico_buffer[..corpo_idrico_buffer_len].copy_from_slice(&corpo_idrico_buffer_bytes[..corpo_idrico_buffer_len]);

        let mut regione_buffer = [0u8; 64];
        let regione_buffer_bytes = "Inserisci regione".as_bytes();
        let regione_buffer_len = regione_buffer_bytes.len().min(64);
        regione_buffer[..regione_buffer_len].copy_from_slice(&regione_buffer_bytes[..regione_buffer_len]);

        let mut provincia_buffer = [0u8; 64];
        let provincia_buffer_bytes = "Inserisci provincia".as_bytes();
        let provincia_buffer_len = provincia_buffer_bytes.len().min(64);
        provincia_buffer[..provincia_buffer_len].copy_from_slice(&provincia_buffer_bytes[..provincia_buffer_len]);
        let mut data_buffer = [0u8; 64];
        let data_buffer_bytes = "Inserisci data".as_bytes();
        let data_buffer_len = data_buffer_bytes.len().min(64);
        data_buffer[..data_buffer_len].copy_from_slice(&data_buffer_bytes[..data_buffer_len]);

        let mut fonte_comunit_buffer = [0u8; 64];
        let fonte_comunit_buffer_bytes = "Inserisci fonte".as_bytes();
        let fonte_comunit_buffer_len = fonte_comunit_buffer_bytes.len().min(64);
        fonte_comunit_buffer[..fonte_comunit_buffer_len].copy_from_slice(&fonte_comunit_buffer_bytes[..fonte_comunit_buffer_len]);

        let mut protocollo_comunit_buffer = [0u8; 64];
        let protocollo_comunit_buffer_bytes = "Inserisci protocollo".as_bytes();
        let protocollo_comunit_buffer_len = protocollo_comunit_buffer_bytes.len().min(64);
        protocollo_comunit_buffer[..protocollo_comunit_buffer_len].copy_from_slice(&protocollo_comunit_buffer_bytes[..protocollo_comunit_buffer_len]);

        let mut bacino_buffer = [0u8; 64];
        let bacino_buffer_bytes = "Inserisci bacino".as_bytes();
        let bacino_buffer_len = bacino_buffer_bytes.len().min(64);
        bacino_buffer[..bacino_buffer_len].copy_from_slice(&bacino_buffer_bytes[..bacino_buffer_len]);

        Self {
            valuebox_codice_stazione_edit_mode: false,
            valuebox_codice_stazione_value: 0,
            textbox_corpo_idrico_edit_mode: false,
            textbox_corpo_idrico_buffer: corpo_idrico_buffer,
            listview_regione_value: 0,
            listview_regione_scroll_value: 0,
            textbox_provincia_edit_mode: false,
            textbox_provincia_buffer: provincia_buffer,
            textbox_data_edit_mode: false,
            textbox_data_buffer: data_buffer,
            valuebox_lunghezza_stazione_edit_mode: false,
            valuebox_lunghezza_stazione_value: 0,
            valuebox_larghezza_stazione_edit_mode: false,
            valuebox_larghezza_stazione_value: 0,
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
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("SelezioneInfoAggiuntiveView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::NISECI
            }
        };

        let groupbox_width = propwidth(&d, 600);
        let groupbox_x = d.get_screen_width() /2 - groupbox_width /2;
        let groupbox_height = propheight(&d, 450);
        let groupbox_y = d.get_screen_height() / 2 - groupbox_height /2;

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Inserisci informazioni aggiuntive"))
        );

        let submit_width = propwidth(&d, 100);
        let groupbox_x_end = groupbox_x + groupbox_width;
        let submit_x = groupbox_x_end + (d.get_screen_width() - groupbox_x_end)/2 - submit_width/2;
        let submit_height = propheight(&d, 50);
        let submit_y = d.get_screen_height() /2 - submit_height /2;

        let confirm_itext = d.gui_icon_text(ICON_OK_TICK, Some(rstr!("Conferma")));
        let confirm_itext = CString::new(confirm_itext).unwrap();
        if d.gui_button(rrect(submit_x, submit_y, submit_width, submit_height), Some(confirm_itext.as_c_str())) {

            //TODO: impl TryInto<u32> for a new custom RegioneItaliana or smth ?
            //But this was request as a free string originally...
            let regione_string = match self.listview_regione_value {
                0 => "Abruzzo".to_string(),
                1 => "Basilicata".to_string(),
                2 => "Calabria".to_string(),
                3 => "Campania".to_string(),
                4 => "Emilia-Romagna".to_string(),
                5 => "Friuli-Venezia-Giulia".to_string(),
                6 => "Lazio".to_string(),
                7 => "Liguria".to_string(),
                8 => "Lombardia".to_string(),
                9 => "Marche".to_string(),
                10 => "Molise".to_string(),
                11 => "Piemonte".to_string(),
                12 => "Puglia".to_string(),
                13 => "Sardegna".to_string(),
                14 => "Sicilia".to_string(),
                15 => "Toscana".to_string(),
                16 => "Trentino-Alto-Adige".to_string(),
                17 => "Umbria".to_string(),
                18 => "Valle d'Aosta".to_string(),
                19 => "Veneto".to_string(),
                _ => { panic!("Unexpected regione_string in SelezioneInfoAggiuntiveView::draw()"); }
            };

            // Raylib has trouble handling the string downstream if we don't ensure to do this
            let end = self.textbox_provincia_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_provincia_buffer.len());
            let provincia_string = match String::from_utf8(self.textbox_provincia_buffer[..end].to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    //TODO: signal error: invalid UTF-8
                    "ERROR".to_string()
                }
            };
            let posizione = Location {
                regione: regione_string,
                provincia: provincia_string,
            };
            let larghezza_stazione = self.valuebox_larghezza_stazione_value;
            let lunghezza_stazione = self.valuebox_lunghezza_stazione_value;
            let codice_stazione = self.valuebox_codice_stazione_value;

            // Raylib has trouble handling the string downstream if we don't ensure to do this
            let end = self.textbox_data_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_data_buffer.len());
            let date_string = match String::from_utf8(self.textbox_data_buffer[..end].to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    //TODO: signal error: invalid UTF-8
                    "ERROR".to_string()
                }
            };

            // Raylib has trouble handling the string downstream if we don't ensure to do this
            let end = self.textbox_corpo_idrico_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_corpo_idrico_buffer.len());
            let corpo_idrico = match String::from_utf8(self.textbox_corpo_idrico_buffer[..end].to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    //TODO: signal error: invalid UTF-8
                    "ERROR".to_string()
                }
            };

            //TODO: impl TryInto<u32> for TipoComunitaNISECI
            //Which would also handle errors better than this crap
            let tipo_comunita = match self.dropdownbox_tipocomunit_niseci_value {
                0 => TipoComunitaNISECI::Redatta,
                1 => TipoComunitaNISECI::Recuperata,
                2 => TipoComunitaNISECI::Dm260_2010,
                3 => TipoComunitaNISECI::AffinataDalMase,
                _ => { panic!("Unexpected tipo_comunita in SelezioneInfoAggiuntiveView::draw()"); }
            };
            let mut opt_fonte: Option<String> = None;
            let mut opt_num_protocollo: Option<String> = None;
            match tipo_comunita {
                TipoComunitaNISECI::Recuperata => {
                    // Raylib has trouble handling the string downstream if we don't ensure to do this
                    let end = self.textbox_fontecomunit_niseci_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_fontecomunit_niseci_buffer.len());
                    opt_fonte = Some(match String::from_utf8(self.textbox_fontecomunit_niseci_buffer[..end].to_vec()) {
                        Ok(s) => s,
                        Err(_) => {
                            //TODO: signal error: invalid UTF-8
                            "ERROR".to_string()
                        }
                    });
                }
                TipoComunitaNISECI::AffinataDalMase => {
                    // Raylib has trouble handling the string downstream if we don't ensure to do this
                    let end = self.textbox_protocollocomunit_niseci_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_protocollocomunit_niseci_buffer.len());
                    opt_num_protocollo = Some(match String::from_utf8(self.textbox_protocollocomunit_niseci_buffer[..end].to_vec()) {
                        Ok(s) => s,
                        Err(_) => {
                            //TODO: signal error: invalid UTF-8
                            "ERROR".to_string()
                        }
                    });
                }
                _ => {}
            }
            let comunita = ComunitaNISECI {
                tipo: tipo_comunita,
                fonte: opt_fonte,
                numero_protocollo: opt_num_protocollo
            };
            //TODO: impl TryInto<u32> for AreaNISECI
            //Which would also handle errors better than this crap
            let area = match self.combobox_area_niseci_value {
                0 => { // 0 == Alpina I guess
                    AreaNISECI::Alpina
                }
                1 => { // 1 == Mediterranea I guess
                    AreaNISECI::Mediterranea
                }
                _ => { panic!("Unexpected area_niseci in SelezioneInfoAggiuntiveView::draw()"); }
            };

            // Raylib has trouble handling the string downstream if we don't ensure to do this
            let end = self.textbox_bacino_niseci_buffer.iter().position(|&b| b == 0).unwrap_or(self.textbox_bacino_niseci_buffer.len());
            let bacino_niseci = match String::from_utf8(self.textbox_bacino_niseci_buffer[..end].to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    //TODO: signal error: invalid UTF-8
                    "ERROR".to_string()
                }
            };

            //TODO: impl TryInto<u32> for IdroEcoRegioneNISECI
            //Which would also handle errors better than this crap
            let idro_ecoregione_niseci = match self.listview_idroecoregione_niseci_value {
                0 => IdroEcoRegioneNISECI::AlpiCentroOrientali,
                1 => IdroEcoRegioneNISECI::AlpiMediterranee,
                2 => IdroEcoRegioneNISECI::AlpiMeridionali,
                3 => IdroEcoRegioneNISECI::AlpiOccidentali,
                4 => IdroEcoRegioneNISECI::AppenninoCentrale,
                5 => IdroEcoRegioneNISECI::AppenninoMeridionale,
                6 => IdroEcoRegioneNISECI::AppenninoPiemontese,
                7 => IdroEcoRegioneNISECI::AppenninoSettentrionale,
                8 => IdroEcoRegioneNISECI::BasilicataTavoliere,
                9 => IdroEcoRegioneNISECI::BassoLazio,
                10 => IdroEcoRegioneNISECI::CalabriaNebrodi,
                11 => IdroEcoRegioneNISECI::Carso,
                12 => IdroEcoRegioneNISECI::CostaAdriatica,
                13 => IdroEcoRegioneNISECI::Monferrato,
                14 => IdroEcoRegioneNISECI::PianuraPadana,
                15 => IdroEcoRegioneNISECI::PrealpiDolomiti,
                16 => IdroEcoRegioneNISECI::PugliaGargano,
                17 => IdroEcoRegioneNISECI::RomaViterbeseVesuvio,
                18 => IdroEcoRegioneNISECI::Sardegna,
                19 => IdroEcoRegioneNISECI::Sicilia,
                20 => IdroEcoRegioneNISECI::Toscana,
                _ => { panic!("Unexpected idroecoregione_niseci in SelezioneInfoAggiuntiveView::draw()"); }
            };

            let anagrafica = AnagraficaNISECI {
                comunita: comunita,
                codice_stazione: codice_stazione as u32,
                date_string: date_string,
                area: area,
                nome_fiume: corpo_idrico,
                bacino_appartenenza: bacino_niseci,
                idro_eco_regione: idro_ecoregione_niseci,
                posizione: posizione,
                lunghezza_media_stazione: lunghezza_stazione as f32,
                larghezza_media_stazione: larghezza_stazione as f32,
            };

            controller.submit_anagrafica_niseci(anagrafica);
        }

        let x_padding = groupbox_width / 20;
        let y_padding = groupbox_height / 15;

        // 2 columns: 2 paddings per side + 1 between the columns
        let column_width = (groupbox_width - (x_padding * 3))/2;
        let column_1_x = groupbox_x + x_padding;
        let column_2_x = column_1_x + column_width + x_padding;
        let column_1_y = groupbox_y + y_padding;
        let column_2_y = column_1_y;
        // 1 padding on top, 2 below
        let column_1_height = groupbox_height - y_padding*3;
        // 1 padding on top, 0 below
        let column_2_height = groupbox_height - y_padding;

        // Column 1

        let column_1_labels_width = column_width / 2;
        let column_1_fields_y_spacing = y_padding / 3;
        let column_1_labels_count = 7;
        let column_1_labels_x = column_1_x;
        let column_1_labels_height = (column_1_height - (column_1_fields_y_spacing*13) ) / column_1_labels_count;

        let column_1_label_stazione_y = column_1_y;
        let column_1_label_corpo_idrico_y = column_1_label_stazione_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_regione_y = column_1_label_corpo_idrico_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_provincia_y = column_1_label_regione_y + column_1_labels_height + column_1_fields_y_spacing*8;
        let column_1_label_data_y = column_1_label_provincia_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_lunghezza_stazione_y = column_1_label_data_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_larghezza_stazione_y = column_1_label_lunghezza_stazione_y + column_1_labels_height + column_1_fields_y_spacing;

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_stazione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Codice stazione"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_corpo_idrico_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Nome del corpo idrico"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_regione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Regione"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_provincia_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Provincia"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_data_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Data"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_lunghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Lunghezza stazione"))
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_larghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            Some(rstr!("Larghezza stazione"))
        );

        let column_1_boxes_width = column_1_labels_width;
        let column_1_boxes_height = column_1_labels_height;
        let column_1_boxes_x = column_1_labels_x + column_1_labels_width;

        if d.gui_value_box(
            rrect(
                column_1_boxes_x,
                column_1_label_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            None,
            &mut self.valuebox_codice_stazione_value,
            0,
            100000, //TODO: ask a reasonable max for this
            self.valuebox_codice_stazione_edit_mode
        ) {
            self.valuebox_codice_stazione_edit_mode = !self.valuebox_codice_stazione_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_corpo_idrico_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_corpo_idrico_buffer,
            self.textbox_corpo_idrico_edit_mode
        ) {
            self.textbox_corpo_idrico_edit_mode = !self.textbox_corpo_idrico_edit_mode;
        }

        let mut _listview_regione_italiana_pick = -1;
        _listview_regione_italiana_pick = d.gui_list_view(
            rrect(
                column_1_boxes_x,
                column_1_label_regione_y,
                column_1_boxes_width,
                column_1_boxes_height*3
            ),
            Some(rstr!(
                "Abruzzo;Basilicata;Calabria;Campania;Emilia-Romagna;\
                Friuli-Venezia-Giulia;Lazio;Liguria;Lombardia;Marche;\
                Molise;Piemonte;Puglia;Sardegna;Sicilia;Toscana;\
                Trentino-Alto-Adige;Umbria;Valle d'Aosta;Veneto"
            )),
            &mut self.listview_regione_scroll_value,
            &mut self.listview_regione_value,
        );
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_provincia_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_provincia_buffer,
            self.textbox_provincia_edit_mode
        ) {
            self.textbox_provincia_edit_mode = !self.textbox_provincia_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_data_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_data_buffer,
            self.textbox_data_edit_mode
        ) {
            self.textbox_data_edit_mode = !self.textbox_data_edit_mode;
        }
        if d.gui_value_box(
            rrect(
                column_1_boxes_x,
                column_1_label_lunghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            None,
            &mut self.valuebox_lunghezza_stazione_value,
            0,
            100000, //TODO: ask a reasonable max for this
            self.valuebox_lunghezza_stazione_edit_mode
        ) {
            self.valuebox_lunghezza_stazione_edit_mode = !self.valuebox_lunghezza_stazione_edit_mode;
        }
        if d.gui_value_box(
            rrect(
                column_1_boxes_x,
                column_1_label_larghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            None,
            &mut self.valuebox_larghezza_stazione_value,
            0,
            100000, //TODO: ask a resonable max for this
            self.valuebox_larghezza_stazione_edit_mode
        ) {
            self.valuebox_larghezza_stazione_edit_mode = !self.valuebox_larghezza_stazione_edit_mode;
        }
        // Column 2

        // spacing between the two groupboxes
        let column_2_groupbox_y_padding = y_padding/2;
        let column_2_groupbox_niseci_width = column_width;
        let column_2_groupbox_niseci_x = column_2_x;
        let column_2_groupbox_niseci_y = column_2_y;
        let column_2_groupbox_niseci_height = column_2_height - column_2_groupbox_y_padding/2;// - y_padding;
        let column_2_groupbox_hfbi_width = column_2_groupbox_niseci_width;
        let column_2_groupbox_hfbi_x = column_2_groupbox_niseci_x;
        //let column_2_groupbox_hfbi_y = column_2_groupbox_niseci_y + column_2_groupbox_niseci_height + column_2_groupbox_y_padding;
        //let column_2_groupbox_hfbi_height = column_2_height - column_2_groupbox_niseci_height;
        let column_2_groupbox_hfbi_y = column_2_groupbox_niseci_y;
        let column_2_groupbox_hfbi_height = column_2_groupbox_niseci_height - column_2_groupbox_y_padding - y_padding;

        let column_2_comunit_x_padding = x_padding/4;
        let column_2_comunit_y_padding = column_2_groupbox_y_padding;
        let column_2_groupbox_comunit_x = column_2_groupbox_niseci_x + column_2_comunit_x_padding;
        let column_2_groupbox_comunit_y = column_2_groupbox_niseci_y + column_2_comunit_y_padding;
        let column_2_groupbox_comunit_width = column_2_groupbox_niseci_width - column_2_comunit_x_padding*2;
        let column_2_groupbox_comunit_height = column_1_labels_height*3 + column_1_fields_y_spacing*4;

        match current_index {
            Indice::NISECI => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_niseci_x,
                        column_2_groupbox_niseci_y,
                        column_2_groupbox_niseci_width,
                        column_2_groupbox_niseci_height
                    ),
                    Some(rstr!("NISECI"))
                );

                d.gui_group_box(
                    rrect(
                        column_2_groupbox_comunit_x,
                        column_2_groupbox_comunit_y,
                        column_2_groupbox_comunit_width,
                        column_2_groupbox_comunit_height
                    ),
                    Some(rstr!("Comunità NISECI"))
                );

            }
            Indice::HFBI => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_hfbi_x,
                        column_2_groupbox_hfbi_y,
                        column_2_groupbox_hfbi_width,
                        column_2_groupbox_hfbi_height
                    ),
                    Some(rstr!("HFBI"))
                );
            }
        }

        let column_2_groupbox_labels_x_spacing = column_2_comunit_x_padding;
        let column_2_groupbox_labels_width = (column_2_groupbox_comunit_width - column_2_groupbox_labels_x_spacing*2) / 2;
        let column_2_groupbox_fields_y_spacing = column_1_fields_y_spacing;
        let column_2_groupbox_labels_x = column_2_groupbox_comunit_x + column_2_groupbox_labels_x_spacing;
        let column_2_labels_height = column_1_labels_height;

        match current_index {
            Indice::NISECI => {
                let column_2_label_tipo_comunit_y = column_2_groupbox_comunit_y + column_2_groupbox_fields_y_spacing;
                let column_2_label_fonte_comunit_y = column_2_label_tipo_comunit_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
                let column_2_label_protocollo_comunit_y = column_2_label_fonte_comunit_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
                let column_2_label_idroecoregione_y = column_2_groupbox_comunit_y + column_2_groupbox_comunit_height + column_2_groupbox_fields_y_spacing;
                let column_2_label_area_niseci_y = column_2_label_idroecoregione_y + column_2_labels_height*4 + column_2_groupbox_fields_y_spacing;
                let column_2_label_bacino_y = column_2_label_area_niseci_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_tipo_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Tipo"))
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_fonte_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Fonte"))
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_protocollo_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Protocollo"))
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_idroecoregione_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Idroecoregione"))
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Area"))
                );


                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_bacino_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    Some(rstr!("Bacino"))
                );

                let column_2_groupbox_boxes_width = column_2_groupbox_labels_width;
                let column_2_groupbox_boxes_height = column_2_labels_height;
                let column_2_groupbox_boxes_x = column_2_groupbox_labels_x + column_2_groupbox_labels_width;

                match self.dropdownbox_tipocomunit_niseci_value {
                    1 => { /* 1 == Fonte I guess */ }
                    _ => { // Not fonte I guess
                        d.gui_lock();
                        d.gui_set_state(STATE_DISABLED);
                    }
                }
                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_fonte_comunit_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
                    ),
                    &mut self.textbox_fontecomunit_niseci_buffer,
                    self.textbox_fontecomunit_niseci_edit_mode
                ) {
                    self.textbox_fontecomunit_niseci_edit_mode = !self.textbox_fontecomunit_niseci_edit_mode;
                }
                match self.dropdownbox_tipocomunit_niseci_value {
                    1 => { /* 1 == Fonte I guess */ }
                    _ => { // Not fonte I guess
                        d.gui_set_state(STATE_NORMAL);
                        d.gui_unlock();
                    }
                }

                match self.dropdownbox_tipocomunit_niseci_value {
                    3 => { /* 3 == Mase I guess */ }
                    _ => { // Not Mase I guess
                        d.gui_lock();
                        d.gui_set_state(STATE_DISABLED);
                    }
                }
                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_protocollo_comunit_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
                    ),
                    &mut self.textbox_protocollocomunit_niseci_buffer,
                    self.textbox_protocollocomunit_niseci_edit_mode
                ) {
                    self.textbox_protocollocomunit_niseci_edit_mode = !self.textbox_protocollocomunit_niseci_edit_mode;
                }
                match self.dropdownbox_tipocomunit_niseci_value {
                    3 => { /* 3 == Mase I guess */ }
                    _ => { // Not Mase I guess
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
                        column_2_groupbox_boxes_height*4
                    ),
                    Some(rstr!(
                        "AlpiCentroOrientali;AlpiMediterranee;AlpiMeridionali;\
                        AlpiOccidentali;AppenninoCentrale;AppenninoMeridionale;\
                        AppenninoPiemontese;AppenninoSettentrionale;BasilicataTavoliere;\
                        BassoLazio;CalabriaNebrodi;Carso;CostaAdriatica;Monferrato;\
                        PianuraPadana;PrealpiDolomiti;PugliaGargano;RomaViterbeseVesuvio;\
                        Sardegna;Sicilia;Toscana"
                    )),
                    &mut self.listview_idroecoregione_niseci_scroll_value,
                    &mut self.listview_idroecoregione_niseci_value,
                );
                let mut _comboBoxPick = -1;
                _comboBoxPick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
                    ),
                    Some(rstr!("Alpina;Mediterranea")),
                    &mut self.combobox_area_niseci_value,
                );

                if d.gui_text_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_bacino_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
                    ),
                    &mut self.textbox_bacino_niseci_buffer,
                    self.textbox_bacino_niseci_edit_mode
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
                    Some(rstr!("Redatta;Fonti;DM260/2010;Mase")),
                    &mut self.dropdownbox_tipocomunit_niseci_value,
                    self.dropdownbox_tipocomunit_niseci_edit_mode,
                ) {
                    self.dropdownbox_tipocomunit_niseci_edit_mode = !self.dropdownbox_tipocomunit_niseci_edit_mode;
                }
            }
            Indice::HFBI => {
                let rainbow_speed = 0.03;
                let todo_hfbi_font_scale = 2;
                let todo_hfbi_font_height = main_state.current_font_height * todo_hfbi_font_scale;

                let todo_hfbi_txt = "TODO: HFBI controls";
                let todo_hfbi_txt_bounds = main_state.current_font.measure_text(todo_hfbi_txt, todo_hfbi_font_height as f32, main_state.default_txt_spacing as f32);
                let todo_hfbi_txt_x = column_2_groupbox_hfbi_x + (column_2_groupbox_hfbi_width / 2) - (todo_hfbi_txt_bounds.x as i32 / 2);
                let todo_hfbi_txt_y = column_2_groupbox_hfbi_y + (column_2_groupbox_hfbi_height / 2) - (todo_hfbi_txt_bounds.y as i32 / 2);

                draw_rainbow_text(d, todo_hfbi_txt_x, todo_hfbi_txt_y, "TODO: HFBI controls", frame_counter, rainbow_speed, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height, todo_hfbi_font_scale);
            }
        }

    }
}

pub struct ValidazioneInfoAggiuntiveView {

}

impl ValidazioneInfoAggiuntiveView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();

        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("ValidazioneInfoAggiuntiveView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::NISECI
            }
        };

        let button_valida_width = propwidth(&d, 200);
        let button_valida_x = d.get_screen_width() / 2 - button_valida_width /2;
        let button_valida_height = propwidth(&d, 50);
        let button_valida_y = d.get_screen_height() / 2 - button_valida_height/2;

        let y_spacing = button_valida_height;
        let button_backout_width = button_valida_width;
        let button_backout_x = button_valida_x;
        let button_backout_height = button_valida_height;
        let button_backout_y = button_valida_y + button_valida_height + y_spacing;

        let groupbox_width = button_valida_width + propwidth(&d, 100);
        let groupbox_x = button_valida_x - propwidth(&d, 50);
        let groupbox_height = button_valida_height * 3 + propheight(&d, 100);
        let groupbox_y = button_valida_y - propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Valida informazioni aggiuntive"))
        );

        if d.gui_button(
            rrect(
                button_valida_x,
                button_valida_y,
                button_valida_width,
                button_valida_height
            ),
            Some(rstr!("Valida info aggiuntive"))
        ) {
            //Ask controller to validate info aggiuntive indice
            match current_index {
                Indice::NISECI => {
                    controller.valida_anagrafica_niseci();
                }
                Indice::HFBI => {
                    //TODO: implement this
                    //controller.valida_anagrafica_hfbi()
                }
            }
        }

        let indietro_itext = d.gui_icon_text(ICON_CROSS, Some(rstr!("Indietro")));
        let indietro_itext = CString::new(indietro_itext).unwrap();
        if d.gui_button(
            rrect(
                button_backout_x,
                button_backout_y,
                button_backout_width,
                button_backout_height,
            ),
            Some(indietro_itext.as_c_str())
        ) {
            //Ask controller to go back and edit further
            match current_index {
                Indice::NISECI => {
                    controller.backout_anagrafica_niseci();
                }
                Indice::HFBI => {
                    //TODO: implement this
                    //controller.backout_anagrafica_hfbi();
                }
            }
        }
    }
}

pub struct ProduzioneOutputView {

}

impl ProduzioneOutputView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let button_calcola_width = propwidth(&d, 200);
        let button_calcola_x = d.get_screen_width() / 2 - button_calcola_width /2;
        let button_calcola_height = propwidth(&d, 50);
        let button_calcola_y = d.get_screen_height() / 4 - button_calcola_height/2;

        let groupbox_width = button_calcola_width + propwidth(&d, 100);
        let groupbox_x = button_calcola_x - propwidth(&d, 50);
        let groupbox_height = button_calcola_height + propheight(&d, 100);
        let groupbox_y = button_calcola_y - propheight(&d, 50);

        let panel_width = groupbox_width + propwidth(&d, 100);
        let panel_x = d.get_screen_width() / 2 - panel_width /2;
        let panel_y = groupbox_y + groupbox_height + propwidth(&d, 50);
        let panel_height = groupbox_height + propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Produzione output"))
        );

        if d.gui_button(
            rrect(
                button_calcola_x,
                button_calcola_y,
                button_calcola_width,
                button_calcola_height
            ),
            Some(rstr!("Calcola"))
        ) {
            //TODO: calcola indice
            println!("TODO: call controller to update model.");
        }

        d.gui_panel(
            rrect(
                panel_x,
                panel_y,
                panel_width,
                panel_height
            ),
            Some(rstr!("TODO: Output qui"))
        );
    }
}

pub struct ProduzionePDFView {

}

impl ProduzionePDFView {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &OutputController, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let button_esporta_width = propwidth(&d, 200);
        let button_esporta_x = d.get_screen_width() / 2 - button_esporta_width /2;
        let button_esporta_height = propwidth(&d, 50);
        let button_esporta_y = d.get_screen_height() / 4 - button_esporta_height/2;

        let groupbox_width = button_esporta_width + propwidth(&d, 100);
        let groupbox_x = button_esporta_x - propwidth(&d, 50);
        let groupbox_height = button_esporta_height + propheight(&d, 100);
        let groupbox_y = button_esporta_y - propheight(&d, 50);

        let panel_width = groupbox_width + propwidth(&d, 100);
        let panel_x = d.get_screen_width() / 2 - panel_width /2;
        let panel_y = groupbox_y + groupbox_height + propwidth(&d, 50);
        let panel_height = groupbox_height + propheight(&d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            Some(rstr!("Produzione PDF"))
        );

        if d.gui_button(
            rrect(
                button_esporta_x,
                button_esporta_y,
                button_esporta_width,
                button_esporta_height
            ),
            Some(rstr!("Esporta"))
        ) {
            //TODO: esporta pdf
            println!("TODO: call controller to update model.");
        }

        d.gui_panel(
            rrect(
                panel_x,
                panel_y,
                panel_width,
                panel_height
            ),
            Some(rstr!("TODO: Output qui"))
        );
    }
}

fn rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    let rainbow_color = Color::new(red, green, blue, 255);
    return rainbow_color;
}

fn draw_rainbow_text(d: &mut RaylibDrawHandle, x: i32, y: i32, text: &str, frame_counter: u32, rainbow_speed: f32, font: &WeakFont, text_spacing: i32, current_font_height: i32, font_height_scale: i32) {
    assert!(font_height_scale > 0);
    // Smaller speed = slower cycle
    let rainbow_color = rainbow_color_from_framecounter(frame_counter, rainbow_speed);

    let text_font_height = current_font_height * font_height_scale;
    //let text_bounds = font.measure_text(&text, text_font_height as f32, text_spacing as f32);
    let text_x = x; //- text_bounds.x as i32 / 2;
    let text_y = y; //- text_bounds.y as i32 / 2;
    d.draw_text_ex(font, text, Vector2::new(text_x as f32, text_y as f32), text_font_height as f32, text_spacing as f32, rainbow_color);
}

pub struct ConsoleView {
    font : Font,
    current_font_size : i32,
    default_font_size : i32,
    font_spacing : i32,
}

impl ConsoleView {
    pub fn new(rl: &mut RaylibHandle, thread : &RaylibThread, font_size : i32, font_spacing: i32) -> Self {
        Self {
            font : rl.load_font_from_memory(&thread,
                ".ttf",
                CONSOLE_FONT_DATA,
                font_size,// *2,
                None).expect("failed loading console font"),
            default_font_size : font_size,
            current_font_size : font_size,
            font_spacing : font_spacing,
        }
    }
    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &ConsoleController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();

        state.console.draw(d, main_state.default_txt_color, self.current_font_size, self.font_spacing, &self.font);
    }
}
