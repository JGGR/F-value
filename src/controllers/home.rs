// Controller to update and access the state
pub(crate) struct HomeController;

use super::{Controller, HomeModel, CurrentView};
use raylib::RaylibHandle;
use crate::MainState;
use crate::state::GLOBAL_STATE;
use crate::app::model::SubModel;

impl Controller for HomeController {
    type SubModel = HomeModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.increment_frame_counter();
        if state.home_model.get_user_continued() {
            eprintln!("HomeController:  L'utente ha premuto Continua");
            eprintln!("HomeController:  Let's update current view and go to SelezioneIndice.");
            main_state.set_current_view(CurrentView::SelezioneIndice)
        }
        if main_state.should_reset {
            eprintln!("HomeController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.console_model.reset();
        }
    }

    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.home_model.clone()
    }
}

impl HomeController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_user_continued(&self, val: bool) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.home_model.set_user_continued(val);
    }
}
