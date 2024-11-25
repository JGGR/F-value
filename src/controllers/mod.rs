use crate::model::core::*;
use crate::core::{MainState, check_campionamento_niseci_path, check_riferimento_niseci_path};
use crate::model::index::Indice;
use crate::state::GLOBAL_STATE;
use crate::CurrentView;
use raylib::RaylibHandle;
use std::path::PathBuf;
use raylib::consts::KeyboardKey::*;

// Controller to update and access the state
pub struct HomeController;

impl HomeController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> HomeModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.home_model.clone();
    }

    pub fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name(name);
    }

    pub fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_value(val);
    }
}

pub struct SecondController;

impl SecondController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> SecondModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.second_model.clone();
    }

    pub fn _set_name(&self, name: String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name(name);
    }

    pub fn set_value(&self, val: i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_value(val);
    }
}

pub struct IndiceController;

impl IndiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.increment_frame_counter();

        match state.indice_model.get_selected_index() {
            Some(index) => {
                eprintln!("IndiceController:  L'utente ha selezionato indice {index}");
                eprintln!("IndiceController:  Let's update current view and go to SelezioneFileInput.");
                main_state.set_current_view(CurrentView::SelezioneFileInput)
            },
            None => ()
        }
    }

    pub fn set_indice_corrente(&self, index: Indice) -> () {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.set_selected_index(index);
    }
}

pub struct FileInputController;

impl FileInputController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.increment_frame_counter();

        let current_indice;
        if let Some(idx) = state.indice_model.get_selected_index() {
            current_indice = idx;
        } else {
            eprintln!("FileInputController:  User did not select an index");
            eprintln!("FileInputController:  Let's update current view and go back to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice);
            return;
        }
        match main_state.current_view {
            CurrentView::SelezioneFileInput => {
                match current_indice {
                    Indice::NISECI => {
                        let mut riferimento_ready = false;

                        if let Some(_rif_path) = state.fileinput_model.get_riferimento_path() {
                            riferimento_ready = true;
                        }
                        let mut campionamento_ready = false;
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            // Assumes the path is ready to be used.
                            // The current selection used by the only forces .csv extension
                            campionamento_ready = true;
                        }

                        if riferimento_ready && campionamento_ready {
                            eprintln!("FileInputController:  NISECI - L'utente ha fornito riferimento e campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to ValidazioneFileInput.");
                            main_state.set_current_view(CurrentView::ValidazioneFileInput);
                        }
                    }
                    Indice::HFBI => {
                        let mut campionamento_ready = false;
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            // Assumes the path is ready to be used.
                            // The current selection used by the only forces .csv extension
                            campionamento_ready = true;
                        }
                        if campionamento_ready {
                            eprintln!("FileInputController:  HFBI - L'utente ha fornito campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to ValidazioneFileInput.");
                            main_state.set_current_view(CurrentView::ValidazioneFileInput);
                        }
                    }
                }
            },
            CurrentView::ValidazioneFileInput => {
                match current_indice {
                    Indice::NISECI => {
                        if let Some(_rif_path) = state.fileinput_model.get_riferimento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a riferimento niseci path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }

                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento niseci path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }

                        let riferimento_valid = state.fileinput_model.get_riferimento_path_valid();
                        let campionamento_valid = state.fileinput_model.get_campionamento_path_valid();

                        if riferimento_valid && campionamento_valid {
                            eprintln!("FileInputController:  NISECI - L'utente ha validato riferimento e campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                    Indice::HFBI => {
                        if let Some(_campionamento_path) = state.fileinput_model.get_campionamento_path() {
                            //
                        } else {
                            eprintln!("FileInputController:  User did not select a campionamento hfbi path");
                            eprintln!("FileInputController:  Let's update current view and go back to SelezioneFileInput.");
                            main_state.set_current_view(CurrentView::SelezioneFileInput);
                            return;
                        }
                        let campionamento_valid = state.fileinput_model.get_campionamento_path_valid();

                        if campionamento_valid {
                            eprintln!("FileInputController:  HFBI - L'utente ha validato campionamento");
                            eprintln!("FileInputController:  Let's update current view and go to SelezioneInfoAggiuntive.");
                            main_state.set_current_view(CurrentView::SelezioneInfoAggiuntive);
                        }
                    }
                }
            },
            _ => {},
        }
    }

    pub fn get_state(&self) -> FileInputModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.clone();
    }

    pub fn get_current_index(&self) -> Option<Indice> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.indice_model.get_selected_index();
    }

    pub fn get_riferimento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_riferimento_path();
    }

    pub fn set_riferimento_path(&self, riferimento_path: Option<PathBuf>) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_riferimento_path(riferimento_path);
        state.fileinput_model.set_riferimento_path_valid(false); // Refresh the validity
    }

    pub fn get_riferimento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_riferimento_path_valid();
    }

    pub fn valida_riferimento_niseci_path(&self) {
        if let Some(path) = self.get_riferimento_path() {
            let csv_check = check_riferimento_niseci_path(path);

            match csv_check {
                Ok(_records) => {
                    //TODO: implement post-csv check step.
                    eprintln!("TODO: implement post-csv check step to ensure the records are valid.");
                    let records_check = false;

                    if records_check {
                        let mut state = GLOBAL_STATE.lock().unwrap();
                        state.fileinput_model.set_riferimento_path_valid(records_check);
                    }
                }
                Err(_errors) => {
                    //TODO: handle displaying the errors?
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    return;
                }
            }
        }
    }

    pub fn get_campionamento_path(&self) -> Option<PathBuf> {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_campionamento_path();
    }

    pub fn set_campionamento_path(&self, campionamento_path: Option<PathBuf>) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.set_campionamento_path(campionamento_path);
        state.fileinput_model.set_campionamento_path_valid(false); // Refresh the validity
    }

    pub fn get_campionamento_path_valid(&self) -> bool {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.get_campionamento_path_valid();
    }

    pub fn valida_campionamento_niseci_path(&self) {
        if let Some(path) = self.get_campionamento_path() {
            let csv_check = check_campionamento_niseci_path(path);

            match csv_check {
                Ok(_records) => {
                    //TODO: implement post-csv check step.
                    eprintln!("TODO: implement post-csv check step to ensure the records are valid.");
                    let records_check = false;

                    if records_check {
                        let mut state = GLOBAL_STATE.lock().unwrap();
                        state.fileinput_model.set_campionamento_path_valid(records_check);
                    }
                }
                Err(_errors) => {
                    //TODO: handle displaying the errors?
                    /*
                    for err in errors {
                        eprintln!("FileInputController:  {err}");
                    }
                    */
                    return;
                }
            }
        }
    }
}

pub struct InfoAggiuntiveController;

impl InfoAggiuntiveController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, _main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.infoaggiuntive_model.increment_frame_counter();
    }

    pub fn get_state(&self) -> InfoAggiuntiveModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.infoaggiuntive_model.clone();
    }
}

pub struct OutputController;

impl OutputController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle, _main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.output_model.increment_frame_counter();
    }

    pub fn get_state(&self) -> OutputModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.output_model.clone();
    }
}

pub struct LogController;

impl LogController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl: &RaylibHandle) {
        //let mut state = GLOBAL_STATE.lock().unwrap();
        //state.second_model.set_name("Updated".to_string());
    }
}

pub struct ConsoleController;

impl ConsoleController {

    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, rl : &mut RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        // Handle input
        if rl.is_key_pressed(KEY_UP) {
            state.console_model.console.scroll_up(1);
        }
        if rl.is_key_pressed(KEY_DOWN) {
            state.console_model.console.scroll_down(1);
        }

        // Detect and pass keys to the console
        while let Some(c) = rl.get_char_pressed() {
            state.console_model.console.handle_input(rl, Some(c), 20, false, false);
        }

        // Check for Enter key press
        if rl.is_key_pressed(KEY_ENTER) {
            state.console_model.console.handle_input(rl, None, 20, true, false);
        }

        // Check for Backspace key press
        if rl.is_key_pressed(KEY_BACKSPACE) {
            state.console_model.console.handle_input(rl, None, 20, false, true);
        }
        state.console_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> ConsoleModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.console_model.clone();
    }
}
