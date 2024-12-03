use crate::model::core::*;
use crate::core::MainState;
use crate::state::GLOBAL_STATE;
use raylib::RaylibHandle;

// Controller to update and access the state
pub struct HomeController;

impl HomeController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl : &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> HomeModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.home_model.clone();
    }

    pub fn set_name(&self, name : String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_name(name);
    }

    pub fn set_value(&self, val : i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_value(val);
    }
}

pub struct SecondController;

impl SecondController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl : &RaylibHandle) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name("Updated".to_string());
    }

    pub fn get_state(&self) -> SecondModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.second_model.clone();
    }

    pub fn set_name(&self, name : String) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_name(name);
    }

    pub fn set_value(&self, val : i32) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.second_model.set_value(val);
    }
}

pub struct IndiceController;

impl IndiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl : &RaylibHandle, _main_state : &mut MainState) {
        //let mut state = GLOBAL_STATE.lock().unwrap();
        //state.second_model.set_name("Updated".to_string());
    }
}

pub struct FileInputController;

impl FileInputController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl : &RaylibHandle, _main_state : &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.fileinput_model.increment_frame_counter();
    }

    pub fn get_state(&self) -> FileInputModel {
        let state = GLOBAL_STATE.lock().unwrap();
        return state.fileinput_model.clone();
    }
}

pub struct InfoAggiuntiveController;

impl InfoAggiuntiveController {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, _rl : &RaylibHandle, _main_state : &mut MainState) {
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

    pub fn update(&self, _rl : &RaylibHandle, _main_state : &mut MainState) {
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

    pub fn update(&self, _rl : &RaylibHandle) {
        //let mut state = GLOBAL_STATE.lock().unwrap();
        //state.second_model.set_name("Updated".to_string());
    }
}
