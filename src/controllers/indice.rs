pub(crate) struct IndiceController;
use crate::controllers::{Controller, IndiceModel, CurrentView};
use raylib::RaylibHandle;
use crate::MainState;
use crate::state::GLOBAL_STATE;
use crate::domain::index::Indice;
use crate::app::model::SubModel;

impl Controller for IndiceController {
    type SubModel = IndiceModel;

    fn update(&self, _rl: &mut RaylibHandle, main_state: &mut MainState) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.increment_frame_counter();

        if let Some(index) = state.indice_model.get_selected_index() {
            eprintln!("IndiceController:  L'utente ha selezionato indice {index}");
            eprintln!("IndiceController:  Let's update current view and go to SelezioneFileInput.");
            main_state.set_current_view(CurrentView::SelezioneFileInput)
        }
        if main_state.should_reset {
            eprintln!("IndiceController: Resetting");
            main_state.should_reset = false;
            state.home_model.reset();
            state.second_model.reset();
            state.indice_model.reset();
            state.console_model.reset();
            main_state.set_current_view(CurrentView::Home);
            return;
        }
    }
    fn get_state(&self) -> Self::SubModel {
        let state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.clone()
    }
}

impl IndiceController {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn set_indice_corrente(&self, index: Indice) {
        let mut state = GLOBAL_STATE.lock().unwrap();
        state.indice_model.set_selected_index(Some(index));
    }
}
