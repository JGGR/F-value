use std::process::exit;

use crate::core::*;
use crate::controllers::*;
use crate::model::index::Indice;
use raylib::prelude::*;
use rfd::FileDialog;

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
            controller.select_index(Indice::NISECI);
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
            controller.select_index(Indice::HFBI);
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
            if d.gui_button(
                rrect(
                    button_riferimento_x,
                    button_riferimento_y,
                    button_riferimento_width,
                    button_riferimento_height
                ),
                Some(rstr!("Riferimento"))
            ) {
                let file = FileDialog::new()
                        .add_filter("csv", &["csv"])
                        .set_directory("/")
                        .pick_file();

                if let Some(filepath) = file {
                    controller.set_riferimento_path(Some(filepath));
                } else {
                    eprintln!("Error: failed getting a file.");
                }
            }
        }

        if d.gui_button(
            rrect(
                button_campionamento_x,
                button_campionamento_y,
                button_campionamento_width,
                button_campionamento_height,
            ),
            Some(rstr!("Campionamento"))
        ) {
            let file = FileDialog::new()
                    .add_filter("csv", &["csv"])
                    .set_directory("/")
                    .pick_file();

            if let Some(filepath) = file {
                controller.set_campionamento_path(Some(filepath));
            } else {
                eprintln!("Error: failed getting a file.");
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
        //TODO: get current indice

        let button_riferimento_width = propwidth(&d, 200);
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width /2;
        let button_riferimento_height = propwidth(&d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y = d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing;

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

        //TODO: handle buttons depending on current indice

        if d.gui_button(
            rrect(
                button_riferimento_x,
                button_riferimento_y,
                button_riferimento_width,
                button_riferimento_height
            ),
            Some(rstr!("Valida Riferimento"))
        ) {
            println!("TODO: handle click on Valida Riferimento");
            println!("TODO: call controller to update model. Controller can update main_state.current_view on next frame in update()");
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
            println!("TODO: handle click on Valida Campionamento");
            println!("TODO: call controller to update model. Controller can update main_state.current_view on next frame in update()");
        }
    }
}

pub struct SelezioneInfoAggiuntiveView {
    valuebox_codice_stazione_edit_mode: bool,
    valuebox_codice_stazione_value: i32,
    textbox_corpo_idrico_edit_mode: bool,
    textbox_corpo_idrico_buffer: [u8; 64],
    textbox_regione_edit_mode: bool,
    textbox_regione_buffer: [u8; 64],
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
            textbox_regione_edit_mode: false,
            textbox_regione_buffer: regione_buffer,
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
            textbox_bacino_niseci_edit_mode: false,
            textbox_bacino_niseci_buffer: bacino_buffer,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &InfoAggiuntiveController, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
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
        // 1 padding on top, 1 below
        let column_2_height = groupbox_height - y_padding*2;

        // Column 1

        let column_1_labels_width = column_width / 2;
        let column_1_fields_y_spacing = y_padding / 3;
        let column_1_big_y_spacing = y_padding*2;
        let column_1_labels_count = 7;
        let column_1_labels_x = column_1_x;
        let column_1_labels_height = (column_1_height - (column_1_fields_y_spacing*5) - column_1_big_y_spacing) / column_1_labels_count;

        let column_1_label_stazione_y = column_1_y;
        let column_1_label_corpo_idrico_y = column_1_label_stazione_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_regione_y = column_1_label_corpo_idrico_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_provincia_y = column_1_label_regione_y + column_1_labels_height + column_1_fields_y_spacing;
        let column_1_label_data_y = column_1_label_provincia_y + column_1_labels_height + column_1_big_y_spacing;
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
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_regione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_regione_buffer,
            self.textbox_regione_edit_mode
        ) {
            self.textbox_regione_edit_mode = !self.textbox_regione_edit_mode;
        }
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
        let column_2_groupbox_niseci_height = column_2_height - column_2_groupbox_y_padding - y_padding*4;
        let column_2_groupbox_hfbi_width = column_2_groupbox_niseci_width;
        let column_2_groupbox_hfbi_x = column_2_groupbox_niseci_x;
        let column_2_groupbox_hfbi_y = column_2_groupbox_niseci_y + column_2_groupbox_niseci_height + column_2_groupbox_y_padding;
        let column_2_groupbox_hfbi_height = column_2_height - column_2_groupbox_niseci_height;

        let column_2_comunit_x_padding = x_padding/4;
        let column_2_comunit_y_padding = column_2_groupbox_y_padding;
        let column_2_groupbox_comunit_x = column_2_groupbox_niseci_x + column_2_comunit_x_padding;
        let column_2_groupbox_comunit_y = column_2_groupbox_niseci_y + column_2_comunit_y_padding;
        let column_2_groupbox_comunit_width = column_2_groupbox_niseci_width - column_2_comunit_x_padding*2;
        let column_2_groupbox_comunit_height = column_1_labels_height*3 + column_1_fields_y_spacing*4;

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

        d.gui_group_box(
            rrect(
                column_2_groupbox_hfbi_x,
                column_2_groupbox_hfbi_y,
                column_2_groupbox_hfbi_width,
                column_2_groupbox_hfbi_height
            ),
            Some(rstr!("HFBI"))
        );

        let column_2_groupbox_labels_x_spacing = column_2_comunit_x_padding;
        let column_2_groupbox_labels_width = (column_2_groupbox_comunit_width - column_2_groupbox_labels_x_spacing*2) / 2;
        let column_2_groupbox_fields_y_spacing = column_1_fields_y_spacing;
        let column_2_groupbox_labels_x = column_2_groupbox_comunit_x + column_2_groupbox_labels_x_spacing;
        let column_2_labels_height = column_1_labels_height;

        let column_2_label_tipo_comunit_y = column_2_groupbox_comunit_y + column_2_groupbox_fields_y_spacing;
        let column_2_label_fonte_comunit_y = column_2_label_tipo_comunit_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
        let column_2_label_protocollo_comunit_y = column_2_label_fonte_comunit_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
        let column_2_label_bacino_y = column_2_groupbox_comunit_y + column_2_groupbox_comunit_height + column_2_groupbox_fields_y_spacing*2;

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
                column_2_label_bacino_y,
                column_2_groupbox_labels_width,
                column_2_labels_height
            ),
            Some(rstr!("Bacino"))
        );

        let column_2_groupbox_boxes_width = column_2_groupbox_labels_width;
        let column_2_groupbox_boxes_height = column_2_labels_height;
        let column_2_groupbox_boxes_x = column_2_groupbox_labels_x + column_2_groupbox_labels_width;

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
        let button_valida_width = propwidth(&d, 200);
        let button_valida_x = d.get_screen_width() / 2 - button_valida_width /2;
        let button_valida_height = propwidth(&d, 50);
        let button_valida_y = d.get_screen_height() / 2 - button_valida_height/2;

        let groupbox_width = button_valida_width + propwidth(&d, 100);
        let groupbox_x = button_valida_x - propwidth(&d, 50);
        let groupbox_height = button_valida_height + propheight(&d, 100);
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
            //TODO: valida info aggiuntive indice
            println!("TODO: call controller to update model. Controller can update main_state.current_view on next frame in update()");
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
