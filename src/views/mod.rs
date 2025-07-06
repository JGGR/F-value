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
use std::cmp::max;
use raylib::prelude::*;
use raylib::consts::GuiState::{STATE_NORMAL, STATE_DISABLED};
use raylib::consts::GuiIconName::{ICON_FILE_OPEN, ICON_BIN, ICON_OK_TICK, ICON_CROSS, ICON_PLAYER_NEXT};
use rfd::FileDialog;
use crate::domain::{index::Indice, location::Location, niseci::{TipoComunitaNISECI, ComunitaNISECI, AreaNISECI, IdroEcoRegioneNISECI, AnagraficaNISECI}, hfbi::{TipoLagunaCostieraHFBI, StagioneHFBI, HabitatHFBI, AnagraficaHFBI}};
use crate::controllers::{Controller, Controllers, HomeController, SecondController, IndiceController, FileInputController, InfoAggiuntiveController, OutputController, ConsoleController};
use crate::app::core::{CurrentView, MainState, propwidth, propheight, CONSOLE_FONT_DATA};
use crate::app::model::SubModel;
use crate::core::SHORT_PROJECT_VERSION;
use crate::core::rrect;

pub(crate) struct Views {
    home_view: HomeView,
    second_view: SecondView,
    selezione_indice_view: SelezioneIndiceView,
    selezione_fileinput_view: SelezioneFileInputView,
    validazione_fileinput_view: ValidazioneFileInputView,
    selezione_infoaggiuntive_view: SelezioneInfoAggiuntiveView,
    validazione_infoaggiuntive_view: ValidazioneInfoAggiuntiveView,
    produzione_output_view: ProduzioneOutputView,
    produzione_pdf_view: ProduzionePDFView,
    console_view: ConsoleView,
}

impl Views {
    pub(crate) fn new(rl: &mut RaylibHandle, thread: &RaylibThread, gui_current_font_height: i32, txt_spacing: i32) -> Self {
        Self {
            home_view: HomeView::new(),
            second_view: SecondView::new(),
            selezione_indice_view: SelezioneIndiceView::new(),
            selezione_fileinput_view: SelezioneFileInputView::new(),
            validazione_fileinput_view: ValidazioneFileInputView::new(),
            selezione_infoaggiuntive_view: SelezioneInfoAggiuntiveView::new(),
            validazione_infoaggiuntive_view: ValidazioneInfoAggiuntiveView::new(),
            produzione_output_view: ProduzioneOutputView::new(),
            produzione_pdf_view: ProduzionePDFView::new(),
            console_view: ConsoleView::new(rl, thread, gui_current_font_height, txt_spacing),
        }
    }
    pub(crate) fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread, controllers: &Controllers, main_state: &MainState) {
        match main_state.current_view {
            CurrentView::Home => {
                self.home_view.draw(d, thread, &controllers.home_controller, main_state);
            }
            CurrentView::Second => {
                self.second_view.draw(d, thread, &controllers.second_controller, main_state);
            }
            CurrentView::SelezioneIndice => {
                self.selezione_indice_view.draw(d, thread, &controllers.indice_controller, main_state);
            }
            CurrentView::SelezioneFileInput => {
                self.selezione_fileinput_view.draw(d, thread, &controllers.fileinput_controller, main_state);
            }
            CurrentView::ValidazioneFileInput => {
                self.validazione_fileinput_view.draw(d, thread, &controllers.fileinput_controller, main_state);
            }
            CurrentView::SelezioneInfoAggiuntive => {
                self.selezione_infoaggiuntive_view.draw(d, thread, &controllers.infoaggiuntive_controller, main_state);
            }
            CurrentView::ValidazioneInfoAggiuntive => {
                self.validazione_infoaggiuntive_view.draw(d, thread, &controllers.infoaggiuntive_controller, main_state);
            }
            CurrentView::ProduzioneOutput => {
                self.produzione_output_view.draw(d, thread, &controllers.output_controller, main_state);
            }
            CurrentView::ProduzionePDF => {
                self.produzione_pdf_view.draw(d, thread, &controllers.output_controller, main_state);
            }
            CurrentView::Console => {
                self.console_view.draw(d, thread, &controllers.console_controller, main_state);
            }
        }
    }
}

pub(crate) trait View {
    type Controller: Controller;
    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState);
}

// A view responsible for rendering the state
// Tightly coupled with its respective controller
pub(crate) struct HomeView {}

impl View for HomeView {
    type Controller = HomeController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();

        let texture_target_width = propwidth(d, 205);
        let texture_target_height = propheight(d, 205);
        let texture_target_x = d.get_screen_width()/2 - texture_target_width /2;
        let texture_target_y = propheight(d, 50);
        if let Some(ref texture) = main_state.logo_texture {
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture.width() as f32,
                    height: texture.height() as f32,
                },
                Rectangle {
                    x: texture_target_x as f32,
                    y: texture_target_y as f32,
                    width: texture_target_width as f32,
                    height: texture_target_height as f32,
                },
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        let label_version_txt = format!("Version:   {}", SHORT_PROJECT_VERSION);
        let label_target_txt = format!("Target:    {}-{}", std::env::consts::ARCH, std::env::consts::OS);
        let label_version_txt_bounds = main_state.current_font.measure_text(&label_version_txt, main_state.current_font_height as f32, main_state.default_txt_spacing as f32);
        let label_target_txt_bounds = main_state.current_font.measure_text(&label_target_txt, main_state.current_font_height as f32, main_state.default_txt_spacing as f32);
        let labels_width = propwidth(d, 25) + max(label_version_txt_bounds.x as i32, label_target_txt_bounds.x as i32);
        let labels_x = d.get_screen_width()/2 - labels_width/2;
        let labels_y = propheight(d, 300);
        let labels_height = propheight(d, 25);

        let labels: Vec<String> = vec!(
            label_version_txt,
            label_target_txt
        );

        for (i, label) in labels.iter().enumerate() {
            d.gui_label(
                rrect(
                    labels_x,
                    labels_y + (i as i32 * labels_height),
                    labels_width,
                    labels_height
                ),
                label.as_str()
            );
        }

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width()/2 - continue_width/2;
        let continue_height = propwidth(d, 50);
        let continue_y_padding = propwidth(d, 25);
        let continue_y = labels_y + (labels_height * labels.len() as i32) + continue_y_padding;

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        if d.gui_button(
            rrect(
                continue_x,
                continue_y,
                continue_width,
                continue_height
            ),
            continue_itext.as_str()
        ) {
            controller.set_user_continued(true);
        }

        let rainbow_speed = 0.03;
        let todo_font_scale = 3;
        let todo_font_height = main_state.current_font_height * todo_font_scale;

        let todo_txt = "TODO: WELCOME";
        let todo_txt_bounds = main_state.current_font.measure_text(todo_txt, todo_font_height as f32, main_state.default_txt_spacing as f32);
        let todo_txt_x = (d.get_screen_width() / 2) - (todo_txt_bounds.x as i32 / 2);
        let todo_txt_y = (d.get_screen_height() / 2) - (todo_txt_bounds.y as i32 / 2);

        draw_rainbow_text(d, todo_txt_x, todo_txt_y, "TODO: WELCOME", frame_counter, rainbow_speed, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height, todo_font_scale);

    }
}

impl HomeView {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

pub(crate) struct SecondView {
    spinner_value: i32,
    spinner_edit_mode: bool,
}

impl View for SecondView {
    type Controller = SecondController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        // Draw the state retrieved via the Controller
        let state = controller.get_state();
        let frame_counter = state.get_frame_counter();
        let state_name = state.get_name();
        let line = format!("Value: {}, Name: {}", state.get_value(), state_name);
        d.draw_text_ex(
            &main_state.current_font,
            &line,
            // We use propwidth/height for the text starting position:
            // this is not the bound
            Vector2::new(propwidth(d, 100) as f32, propheight(d, 10) as f32),
            main_state.current_font_height as f32,
            main_state.default_txt_spacing as f32,
            main_state.default_txt_color
        );


        let updated_spinner = d.gui_spinner(
            rrect(propwidth(d, 100), propheight(d, 50), propwidth(d, 125), propheight(d, 30)),
            "",
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

        let texture_target_width = propwidth(d, 205);
        let texture_target_height = propheight(d, 205);
        let texture_target_x = d.get_screen_width()/2 - texture_target_width /2;
        let texture_target_y = propheight(d, 50);
        if let Some(ref texture) = main_state.logo_texture {
            d.draw_texture_pro(
                texture,
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: texture.width() as f32,
                    height: texture.height() as f32,
                },
                Rectangle {
                    x: texture_target_x as f32,
                    y: texture_target_y as f32,
                    width: texture_target_width as f32,
                    height: texture_target_height as f32,
                },
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        let label_version_txt = format!("Version:   {}", SHORT_PROJECT_VERSION);
        let label_target_txt = format!("Target:    {}-{}", std::env::consts::ARCH, std::env::consts::OS);
        let label_version_txt_bounds = main_state.current_font.measure_text(&label_version_txt, main_state.current_font_height as f32, main_state.default_txt_spacing as f32);
        let label_target_txt_bounds = main_state.current_font.measure_text(&label_target_txt, main_state.current_font_height as f32, main_state.default_txt_spacing as f32);
        let labels_width = propwidth(d, 25) + max(label_version_txt_bounds.x as i32, label_target_txt_bounds.x as i32);
        let labels_x = d.get_screen_width()/2 - labels_width/2;
        let labels_y = propheight(d, 300);
        let labels_height = propheight(d, 25);

        let labels: Vec<String> = vec!(
            label_version_txt,
            label_target_txt
        );

        for (i, label) in labels.iter().enumerate() {
            d.gui_label(
                rrect(
                    labels_x,
                    labels_y + (i as i32 * labels_height),
                    labels_width,
                    labels_height
                ),
                label.as_str()
            );
        }

        let continue_width = propwidth(d, 150);
        let continue_x = d.get_screen_width()/2 - continue_width/2;
        let continue_height = propwidth(d, 50);
        let continue_y_padding = propwidth(d, 25);
        let continue_y = labels_y + (labels_height * labels.len() as i32) + continue_y_padding;

        let continue_itext = d.gui_icon_text(ICON_PLAYER_NEXT, ": Continua");

        if d.gui_button(
            rrect(
                continue_x,
                continue_y,
                continue_width,
                continue_height
            ),
            continue_itext.as_str()
        ) {
            controller.set_user_continued(true);
        }

        let rainbow_speed = 0.03;
        let todo_font_scale = 3;
        let todo_font_height = main_state.current_font_height * todo_font_scale;

        let todo_txt = "TODO: WELCOME";
        let todo_txt_bounds = main_state.current_font.measure_text(todo_txt, todo_font_height as f32, main_state.default_txt_spacing as f32);
        let todo_txt_x = (d.get_screen_width() / 2) - (todo_txt_bounds.x as i32 / 2);
        let todo_txt_y = (d.get_screen_height() / 2) - (todo_txt_bounds.y as i32 / 2);

        draw_rainbow_text(d, todo_txt_x, todo_txt_y, "TODO: WELCOME", frame_counter, rainbow_speed, &main_state.current_font, main_state.default_txt_spacing, main_state.current_font_height, todo_font_scale);

    }
}

impl SecondView {
    pub(crate) fn new() -> Self {
        Self {
            spinner_value: 0,
            spinner_edit_mode: false,
        }
    }
}

pub(crate) struct SelezioneIndiceView {

}

impl View for SelezioneIndiceView {
    type Controller = IndiceController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let button_niseci_width = propwidth(d, 200);
        let button_niseci_x = d.get_screen_width() / 2 - button_niseci_width /2;
        let button_niseci_height = propwidth(d, 50);

        let button_indice_y_spacing = button_niseci_height;

        let button_niseci_y = d.get_screen_height() / 2 - button_indice_y_spacing / 2 - button_niseci_height;

        let button_hfbi_width = button_niseci_width;
        let button_hfbi_x = button_niseci_x;
        let button_hfbi_height = button_niseci_height;
        let button_hfbi_y = button_niseci_y + button_niseci_height + button_indice_y_spacing;

        let groupbox_width = button_niseci_width + propwidth(d, 100);
        let groupbox_x = button_niseci_x - propwidth(d, 50);
        let groupbox_height = button_niseci_height * 2 + button_indice_y_spacing + propheight(d, 100);
        let groupbox_y = button_niseci_y - propheight(d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Seleziona Indice"
        );

        if d.gui_button(
            rrect(
                button_niseci_x,
                button_niseci_y,
                button_niseci_width,
                button_niseci_height
            ),
            "NISECI"
        ) {
            controller.set_indice_corrente(Indice::Niseci);
        }

        if d.gui_button(
            rrect(
                button_hfbi_x,
                button_hfbi_y,
                button_hfbi_width,
                button_hfbi_height,
            ),
            "HFBI"
        ) {
            controller.set_indice_corrente(Indice::Hfbi);
        }
    }
}

impl SelezioneIndiceView {

    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

pub(crate) struct SelezioneFileInputView {

}

impl View for SelezioneFileInputView {
    type Controller = FileInputController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

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
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width /2;
        let button_riferimento_height = propwidth(d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y = d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = match current_index {
            Indice::Hfbi => button_riferimento_y + button_fileinput_y_spacing,
            Indice::Niseci => button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing,
        };

        let groupbox_width = button_riferimento_width + propwidth(d, 100);
        let groupbox_x = button_riferimento_x - propwidth(d, 50);
        let groupbox_height = button_riferimento_height * 2 + button_fileinput_y_spacing + propheight(d, 100);
        let groupbox_y = button_riferimento_y - propheight(d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Seleziona file di input"
        );

        if current_index != Indice::Hfbi {
            if let Some(_filepath) = controller.get_riferimento_path() { // A file is already set, display button to clear it
                let rif_itext = d.gui_icon_text(ICON_BIN, "Annulla Riferimento");
                if d.gui_button(
                    rrect(
                        button_riferimento_x,
                        button_riferimento_y,
                        button_riferimento_width,
                        button_riferimento_height
                    ),
                    rif_itext.as_str()
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
                        button_riferimento_height
                    ),
                    rif_itext.as_str()
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
            let camp_itext = d.gui_icon_text(ICON_BIN, "Annulla Campionamento");
            if d.gui_button(
                rrect(
                    button_campionamento_x,
                    button_campionamento_y,
                    button_campionamento_width,
                    button_campionamento_height,
                ),
                camp_itext.as_str()
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
                camp_itext.as_str()
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

impl SelezioneFileInputView {

    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

pub(crate) struct ValidazioneFileInputView {

}

impl View for ValidazioneFileInputView {
    type Controller = FileInputController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("Indice non selezionato");
                exit(1)
            }
        };

        let button_riferimento_width = propwidth(d, 200);
        let button_riferimento_x = d.get_screen_width() / 2 - button_riferimento_width /2;
        let button_riferimento_height = propwidth(d, 50);

        let button_fileinput_y_spacing = button_riferimento_height;

        let button_riferimento_y = d.get_screen_height() / 2 - button_fileinput_y_spacing / 2 - button_riferimento_height;

        let button_campionamento_width = button_riferimento_width;
        let button_campionamento_x = button_riferimento_x;
        let button_campionamento_height = button_riferimento_height;
        let button_campionamento_y = match current_index {
            Indice::Hfbi => button_riferimento_y + button_fileinput_y_spacing,
            Indice::Niseci => button_riferimento_y + button_riferimento_height + button_fileinput_y_spacing,
        };

        let groupbox_width = button_riferimento_width + propwidth(d, 100);
        let groupbox_x = button_riferimento_x - propwidth(d, 50);
        let groupbox_height = button_riferimento_height * 2 + button_fileinput_y_spacing + propheight(d, 100);
        let groupbox_y = button_riferimento_y - propheight(d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Valida file di input"
        );

        if current_index != Indice::Hfbi && d.gui_button(
                rrect(
                    button_riferimento_x,
                    button_riferimento_y,
                    button_riferimento_width,
                    button_riferimento_height
                ),
                "Valida Riferimento"
        ) {
                controller.valida_riferimento_niseci_path();
        }

        let mut turn_off_button_campionamento = false;
        if current_index == Indice::Niseci && !controller.get_riferimento_path_valid() {
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
            "Valida Campionamento"
        ) {
            match current_index {
                Indice::Niseci => {
                    controller.valida_campionamento_niseci_path();
                }
                Indice::Hfbi => {
                    controller.valida_campionamento_hfbi_path();
                }
            }
        }

        if turn_off_button_campionamento {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }
    }
}

impl ValidazioneFileInputView {

    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

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
    type Controller = InfoAggiuntiveController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("SelezioneInfoAggiuntiveView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

        let groupbox_width = propwidth(d, 750);
        let groupbox_x = d.get_screen_width() /2 - groupbox_width /2;
        let groupbox_height = propheight(d, 450);
        let groupbox_y = d.get_screen_height() / 2 - groupbox_height /2;

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Inserisci informazioni aggiuntive"
        );

        let submit_width = propwidth(d, 100);
        let groupbox_x_end = groupbox_x + groupbox_width;
        let submit_x = groupbox_x_end + (d.get_screen_width() - groupbox_x_end)/2 - submit_width/2;
        let submit_height = propheight(d, 50);
        let submit_y = d.get_screen_height() /2 - submit_height /2;

        let confirm_itext = d.gui_icon_text(ICON_OK_TICK, "Conferma");
        if d.gui_button(rrect(submit_x, submit_y, submit_width, submit_height), confirm_itext.as_str()) {

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

            let provincia_string = String::from(&self.textbox_provincia_buffer);
            let posizione = Location {
                regione: regione_string,
                provincia: provincia_string,
            };


            let larghezza_stazione_str = String::from(&self.textbox_larghezza_stazione_buffer);

            let larghezza_stazione = match controller.check_larghezza_stazione_string(&larghezza_stazione_str) {
                Ok(v) => {
                    v
                }
                Err(_e) => {
                    return; // This is not very appropriate but we expect the controller to change
                            // the view for us in case of error so it's ok I guess
                }
            };

            let lunghezza_stazione_str = String::from(&self.textbox_lunghezza_stazione_buffer);

            let lunghezza_stazione = match controller.check_lunghezza_stazione_string(&lunghezza_stazione_str) {
                Ok(v) => {
                    v
                }
                Err(_e) => {
                    return; // This is not very appropriate but we expect the controller to change
                            // the view for us in case of error so it's ok I guess
                }
            };

            let codice_stazione = String::from(&self.textbox_codice_stazione_buffer);
            let date_string =  String::from(&self.textbox_data_buffer);

            let corpo_idrico = String::from(&self.textbox_corpo_idrico_buffer);

            match current_index {
                Indice::Niseci => {
                    let tipo_comunita = match <TipoComunitaNISECI as TryFrom<i32>>::try_from(self.dropdownbox_tipocomunit_niseci_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected tipo_comunita in SelezioneInfoAggiuntiveView::draw()"); }
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
                            opt_num_protocollo = Some(self.textbox_protocollocomunit_niseci_buffer.clone());
                        }
                        _ => {}
                    }
                    let comunita = ComunitaNISECI {
                        tipo: tipo_comunita,
                        fonte: opt_fonte,
                        numero_protocollo: opt_num_protocollo
                    };
                    let area = match <AreaNISECI as TryFrom<i32>>::try_from(self.combobox_area_niseci_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected area_niseci in SelezioneInfoAggiuntiveView::draw()"); }
                    };

                    let bacino_niseci = String::from(&self.textbox_bacino_niseci_buffer);

                    let idro_ecoregione_niseci = match <IdroEcoRegioneNISECI as TryFrom<i32>>::try_from(self.listview_idroecoregione_niseci_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected idroecoregione_niseci in SelezioneInfoAggiuntiveView::draw()"); }
                    };

                    let anagrafica = AnagraficaNISECI {
                        comunita,
                        codice_stazione,
                        date_string,
                        area,
                        corpo_idrico,
                        bacino_appartenenza: bacino_niseci,
                        idro_eco_regione: idro_ecoregione_niseci,
                        posizione,
                        lunghezza_media_stazione: lunghezza_stazione,
                        larghezza_media_stazione: larghezza_stazione,
                    };

                    controller.submit_anagrafica_niseci(anagrafica);
                }
                Indice::Hfbi => {
                    let tipo_laguna_costiera = match <TipoLagunaCostieraHFBI as TryFrom<i32>>::try_from(self.dropdownbox_tipolaguna_hfbi_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected tipo_laguna in SelezioneInfoAggiuntiveView::draw()"); }
                    };
                    let stagione = match <StagioneHFBI as TryFrom<i32>>::try_from(self.combobox_stagione_hfbi_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected stagione in SelezioneInfoAggiuntiveView::draw()"); }
                    };
                    let habitat_vegetato = match <HabitatHFBI as TryFrom<i32>>::try_from(self.combobox_habitat_vegetato_hfbi_value) {
                        Ok(v) => v,
                        _ => { panic!("Unexpected habitat in SelezioneInfoAggiuntiveView::draw()"); }
                    };
                    let anagrafica = AnagraficaHFBI {
                        codice_stazione,
                        corpo_idrico,
                        posizione,
                        date_string,
                        tipo_laguna: tipo_laguna_costiera,
                        stagione,
                        habitat_vegetato,
                        lunghezza_media_transetto: lunghezza_stazione,
                        larghezza_media_transetto: larghezza_stazione
                    };
                    controller.submit_anagrafica_hfbi(anagrafica);
                }
            }
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
            "Codice stazione"
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_corpo_idrico_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            "Nome del corpo idrico"
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_regione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            "Regione"
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_provincia_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            "Provincia"
        );

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_data_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            "Data"
        );

        let label_for_station_length;
        match current_index {
            Indice::Niseci => {
                label_for_station_length = "Lunghezza stazione";
            }
            Indice::Hfbi => {
                label_for_station_length = "Lunghezza transetto";
            }
        }
        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_lunghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            label_for_station_length
        );

        let label_for_station_width;
        match current_index {
            Indice::Niseci => {
                label_for_station_width = "Larghezza stazione";
            }
            Indice::Hfbi => {
                label_for_station_width = "Larghezza transetto";
            }
        }

        d.gui_label(
            rrect(
                column_1_labels_x,
                column_1_label_larghezza_stazione_y,
                column_1_labels_width,
                column_1_labels_height
            ),
            label_for_station_width
        );

        let column_1_boxes_width = column_1_labels_width;
        let column_1_boxes_height = column_1_labels_height;
        let column_1_boxes_x = column_1_labels_x + column_1_labels_width;

        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_codice_stazione_buffer,
            self.textbox_codice_stazione_edit_mode
        ) {
            self.textbox_codice_stazione_edit_mode = !self.textbox_codice_stazione_edit_mode;
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
            "Abruzzo;Basilicata;Calabria;Campania;Emilia-Romagna;\
            Friuli-Venezia-Giulia;Lazio;Liguria;Lombardia;Marche;\
            Molise;Piemonte;Puglia;Sardegna;Sicilia;Toscana;\
            Trentino-Alto-Adige;Umbria;Valle d'Aosta;Veneto",
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
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_lunghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_lunghezza_stazione_buffer,
            self.textbox_lunghezza_stazione_edit_mode
        ) {
            self.textbox_lunghezza_stazione_edit_mode = !self.textbox_lunghezza_stazione_edit_mode;
        }
        if d.gui_text_box(
            rrect(
                column_1_boxes_x,
                column_1_label_larghezza_stazione_y,
                column_1_boxes_width,
                column_1_boxes_height
            ),
            &mut self.textbox_larghezza_stazione_buffer,
            self.textbox_larghezza_stazione_edit_mode
        ) {
            self.textbox_larghezza_stazione_edit_mode = !self.textbox_larghezza_stazione_edit_mode;
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
            Indice::Niseci => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_niseci_x,
                        column_2_groupbox_niseci_y,
                        column_2_groupbox_niseci_width,
                        column_2_groupbox_niseci_height
                    ),
                    "NISECI"
                );

                d.gui_group_box(
                    rrect(
                        column_2_groupbox_comunit_x,
                        column_2_groupbox_comunit_y,
                        column_2_groupbox_comunit_width,
                        column_2_groupbox_comunit_height
                    ),
                    "Comunità NISECI"
                );

            }
            Indice::Hfbi => {
                d.gui_group_box(
                    rrect(
                        column_2_groupbox_hfbi_x,
                        column_2_groupbox_hfbi_y,
                        column_2_groupbox_hfbi_width,
                        column_2_groupbox_hfbi_height
                    ),
                    "HFBI"
                );
            }
        }

        let column_2_groupbox_labels_x_spacing = column_2_comunit_x_padding;
        let column_2_groupbox_labels_width = (column_2_groupbox_comunit_width - column_2_groupbox_labels_x_spacing*2) / 2;
        let column_2_groupbox_fields_y_spacing = column_1_fields_y_spacing;
        let column_2_groupbox_labels_x = column_2_groupbox_comunit_x + column_2_groupbox_labels_x_spacing;
        let column_2_labels_height = column_1_labels_height;

        match current_index {
            Indice::Niseci => {
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
                    "Tipo"
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_fonte_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    "Fonte"
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_protocollo_comunit_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    "Protocollo"
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_idroecoregione_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    "Idroecoregione"
                );

                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    "Area"
                );


                d.gui_label(
                    rrect(
                        column_2_groupbox_labels_x,
                        column_2_label_bacino_y,
                        column_2_groupbox_labels_width,
                        column_2_labels_height
                    ),
                    "Bacino"
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
                    "AlpiCentroOrientali;AlpiMediterranee;AlpiMeridionali;\
                    AlpiOccidentali;AppenninoCentrale;AppenninoMeridionale;\
                    AppenninoPiemontese;AppenninoSettentrionale;BasilicataTavoliere;\
                    BassoLazio;CalabriaNebrodi;Carso;CostaAdriatica;Monferrato;\
                    PianuraPadana;PrealpiDolomiti;PugliaGargano;RomaViterbeseVesuvio;\
                    Sardegna;Sicilia;Toscana",
                    &mut self.listview_idroecoregione_niseci_scroll_value,
                    &mut self.listview_idroecoregione_niseci_value,
                );
                let mut _combo_box_pick = -1;
                _combo_box_pick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_area_niseci_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
                    ),
                    "Alpina;Mediterranea",
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
                    "Redatta;Fonti;DM260/2010;Mase",
                    &mut self.dropdownbox_tipocomunit_niseci_value,
                    self.dropdownbox_tipocomunit_niseci_edit_mode,
                ) {
                    self.dropdownbox_tipocomunit_niseci_edit_mode = !self.dropdownbox_tipocomunit_niseci_edit_mode;
                }
            }
            Indice::Hfbi => {
                let column_2_label_stagione_y = column_2_groupbox_comunit_y + column_2_groupbox_fields_y_spacing;
                let column_2_label_habitat_vegetato_y = column_2_label_stagione_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
                let column_2_label_tipo_laguna_y = column_2_label_habitat_vegetato_y + column_2_labels_height + column_2_groupbox_fields_y_spacing;
                let column_2_groupbox_boxes_width = column_2_groupbox_labels_width;
                let column_2_groupbox_boxes_height = column_2_labels_height;
                let column_2_groupbox_boxes_x = column_2_groupbox_labels_x + column_2_groupbox_labels_width;
                let mut _combo_box_stagione_pick = -1;
                _combo_box_stagione_pick = d.gui_combo_box(
                    rrect(
                        column_2_groupbox_boxes_x,
                        column_2_label_stagione_y,
                        column_2_groupbox_boxes_width,
                        column_2_groupbox_boxes_height
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
                        column_2_groupbox_boxes_height
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
                    self.dropdownbox_tipolaguna_hfbi_edit_mode = !self.dropdownbox_tipolaguna_hfbi_edit_mode;
                }
            }
        }

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

pub(crate) struct ValidazioneInfoAggiuntiveView {

}

impl View for ValidazioneInfoAggiuntiveView {
    type Controller = InfoAggiuntiveController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

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
        let button_valida_x = d.get_screen_width() / 2 - button_valida_width /2;
        let button_valida_height = propwidth(d, 50);
        let button_valida_y = d.get_screen_height() / 2 - button_valida_height/2;

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
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Valida informazioni aggiuntive"
        );

        if d.gui_button(
            rrect(
                button_valida_x,
                button_valida_y,
                button_valida_width,
                button_valida_height
            ),
            "Valida info aggiuntive"
        ) {
            //Ask controller to validate info aggiuntive indice
            match current_index {
                Indice::Niseci => {
                    controller.valida_anagrafica_niseci();
                }
                Indice::Hfbi => {
                    controller.valida_anagrafica_hfbi()
                }
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
            indietro_itext.as_str()
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
        Self {

        }
    }
}

pub(crate) struct ProduzioneOutputView {

}

impl View for ProduzioneOutputView {
    type Controller = OutputController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

        d.clear_background(main_state.default_bg_color);

        let _state = controller.get_state();
        let current_index = match controller.get_current_index() {
            Some(index) => index,
            None => {
                eprintln!("ProduzioneOutputView: Per qualche assurdo motivo l'indice corrente non è validato. Uso NISECI.");
                Indice::Niseci
            }
        };

        let button_calcola_width = propwidth(d, 200);
        let button_calcola_x = d.get_screen_width() / 2 - button_calcola_width /2;
        let button_calcola_height = propwidth(d, 50);
        let button_calcola_y = d.get_screen_height() / 4 - button_calcola_height/2;

        let groupbox_width = button_calcola_width + propwidth(d, 100);
        let groupbox_x = button_calcola_x - propwidth(d, 50);
        let groupbox_height = button_calcola_height + propheight(d, 100);
        let groupbox_y = button_calcola_y - propheight(d, 50);

        let panel_width = groupbox_width + propwidth(d, 175);
        let panel_x = d.get_screen_width() / 2 - panel_width /2;
        let panel_y = groupbox_y + groupbox_height + propwidth(d, 50);
        let panel_height = groupbox_height + propheight(d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Produzione output"
        );

        if d.gui_button(
            rrect(
                button_calcola_x,
                button_calcola_y,
                button_calcola_width,
                button_calcola_height
            ),
            "Calcola"
        ) {
            //TODO: calcola indice
            match current_index {
                Indice::Niseci => {
                    controller.calc_niseci();
                },
                Indice::Hfbi => {
                    controller.calc_hfbi();
                }
            }
        }

        let submit_width = propwidth(d, 125);
        let panel_x_end = panel_x + panel_width;
        let submit_x = panel_x_end + (d.get_screen_width() - panel_x_end)/2 - submit_width/2;
        let submit_height = propheight(d, 50);
        let submit_y = d.get_screen_height() /2 - submit_height /2;

        let confirm_itext = d.gui_icon_text(ICON_OK_TICK, "Conferma");

        let done_calc = controller.get_is_done_calc();

        let _really_done_calc = match current_index {
            // This distinguishes the case where the calc finished but the resulting value is None.
            // It should only really happen when we try to get x2 with a divide-by-zero, as in:
            // our campionamento had no records matching a riferimentore record with specie_attesa == 1

            Indice::Niseci => {
                done_calc && controller.get_niseci_value().is_some()
            }
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
        if d.gui_button(rrect(submit_x, submit_y, submit_width, submit_height), confirm_itext.as_str()) {
            controller.user_confirm_calc();
        }
        if lock_user_submit {
            d.gui_set_state(STATE_NORMAL);
            d.gui_unlock();
        }

        d.gui_panel(
            rrect(
                panel_x,
                panel_y,
                panel_width,
                panel_height
            ),
            "Output"
        );

        let y_spacing = main_state.current_font_height + propwidth(d, 5);
        let output_start_y = panel_y + propwidth(d, 15);

        match current_index {
            Indice::Niseci => {
                let niseci_opt = controller.get_niseci_value();
                let niseci_str = match niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() { // Could use done_calc and avoid another
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing )) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );
                let rqe_niseci_opt = controller.get_rqe_niseci_value();
                let rqe_niseci_str = match rqe_niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() {
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing * 2)) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );
                let stato_eco_niseci_opt = controller.get_stato_eco_niseci_value();
                let stato_eco_niseci_str = match stato_eco_niseci_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() {
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing * 3)) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );

                let x1_opt = controller.get_x1_value();
                let x1_str = match x1_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() {
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing * 4)) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );

                let x2_opt = controller.get_x2_value();
                let x2_str = match x2_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() {
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing * 5)) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );

                let x3_opt = controller.get_x3_value();
                let x3_str = match x3_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() {
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing * 6)) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );
            },
            Indice::Hfbi => {
                let hfbi_opt = controller.get_hfbi_value();
                let hfbi_str = match hfbi_opt {
                    Some(v) => {
                        format!("{}", v)
                    }
                    None => {
                        if controller.get_is_done_calc() { // Could use done_calc and avoid another
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
                    Vector2::new((panel_x + propwidth(d, 25)) as f32, (output_start_y + (y_spacing )) as f32),
                    main_state.current_font_height as f32,
                    main_state.default_txt_spacing as f32,
                    main_state.default_txt_color
                );
            }
        }
    }
}

impl ProduzioneOutputView {

    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

pub(crate) struct ProduzionePDFView {

}

impl View for ProduzionePDFView {
    type Controller = OutputController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {

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
        let button_esporta_x = d.get_screen_width() / 2 - button_esporta_width /2;
        let button_esporta_height = propwidth(d, 50);
        let button_esporta_y = d.get_screen_height() / 4 - button_esporta_height/2;

        let groupbox_width = button_esporta_width + propwidth(d, 100);
        let groupbox_x = button_esporta_x - propwidth(d, 50);
        let groupbox_height = button_esporta_height + propheight(d, 100);
        let groupbox_y = button_esporta_y - propheight(d, 50);

        let panel_width = groupbox_width + propwidth(d, 100);
        let panel_x = d.get_screen_width() / 2 - panel_width /2;
        let panel_y = groupbox_y + groupbox_height + propwidth(d, 50);
        let panel_height = groupbox_height + propheight(d, 50);

        d.gui_group_box(
            rrect(
                groupbox_x,
                groupbox_y,
                groupbox_width,
                groupbox_height
            ),
            "Produzione PDF"
        );

        if d.gui_button(
            rrect(
                button_esporta_x,
                button_esporta_y,
                button_esporta_width,
                button_esporta_height
            ),
            "Esporta"
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
                controller.add_console_message("Failed getting a file for esportazione".to_string());
            }
        }

        d.gui_panel(
            rrect(
                panel_x,
                panel_y,
                panel_width,
                panel_height
            ),
            "TODO: Output qui"
        );

    }
}

impl ProduzionePDFView {

    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

fn rainbow_color_from_framecounter(frame_counter: u32, speed: f32) -> Color {
    let red = (0.5 * (1.0 + (frame_counter as f32 * speed).sin()) * 255.0) as u8;
    let green = (0.5 * (1.0 + (frame_counter as f32 * speed + 2.0).sin()) * 255.0) as u8;
    let blue = (0.5 * (1.0 + (frame_counter as f32 * speed + 4.0).sin()) * 255.0) as u8;

    Color::new(red, green, blue, 255)
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

pub(crate) struct ConsoleView {
    font : Font,
    current_font_size : i32,
    _default_font_size : i32,
    font_spacing : i32,
}

impl View for ConsoleView {
    type Controller = ConsoleController;

    fn draw(&mut self, d: &mut RaylibDrawHandle, _thread: &RaylibThread, controller: &Self::Controller, main_state: &MainState) {
        d.clear_background(main_state.default_bg_color);

        let state = controller.get_state();

        state.console.draw(d, controller, main_state.default_txt_color, self.current_font_size, self.font_spacing, &self.font);
    }
}

impl ConsoleView {
    pub(crate) fn new(rl: &mut RaylibHandle, thread : &RaylibThread, font_size : i32, font_spacing: i32) -> Self {
        Self {
            font : rl.load_font_from_memory(thread,
                ".ttf",
                CONSOLE_FONT_DATA,
                font_size,// *2,
                None).expect("failed loading console font"),
            _default_font_size: font_size,
            current_font_size: font_size,
            font_spacing: font_spacing*2
        }
    }
}
