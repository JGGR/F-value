use super::index::Indice;
use std::path::PathBuf;


// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct HomeModel {
    value: i32,
    name: String,
}

impl HomeModel {
    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}


// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct SecondModel {
    value: i32,
    name: String,
}

impl SecondModel {
    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    pub fn set_value(&mut self, val: i32) {
        self.value = val;
    }
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct IndiceModel {
    frame_counter: u32,
    selected_index: Option<Indice>
}

impl IndiceModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }

    pub fn set_selected_index(&mut self, index: Indice) -> () {
        self.selected_index = Some(index);
    }

    pub fn get_selected_index(&self) -> Option<Indice> {
        return self.selected_index;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct FileInputModel {
    frame_counter: u32,
    riferimento_path: Option<PathBuf>,
    campionamento_path: Option<PathBuf>
}

impl FileInputModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
    pub fn set_riferimento_path(&mut self, path: Option<PathBuf>) {
        self.riferimento_path = path;
    }

    pub fn get_riferimento_path(&self) -> Option<PathBuf> {
        return self.riferimento_path.clone();
    }

    pub fn set_campionamento_path(&mut self, path: Option<PathBuf>) {
        self.campionamento_path = path;
    }

    pub fn get_campionamento_path(&self) -> Option<PathBuf> {
        return self.campionamento_path.clone();
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct InfoAggiuntiveModel {
    frame_counter: u32,
}

impl InfoAggiuntiveModel {
    pub fn get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct OutputModel {
    frame_counter: u32,
}

impl OutputModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
}

#[derive(Clone)]
pub struct Model {
    pub home_model: HomeModel,
    pub second_model: SecondModel,
    pub indice_model: IndiceModel,
    pub fileinput_model: FileInputModel,
    pub infoaggiuntive_model: InfoAggiuntiveModel,
    pub output_model: OutputModel
}

impl Model {
    pub fn new() -> Self {
        Self {
            home_model: HomeModel {
                value: 0,
                name: "Initial".to_string(),
            },
            second_model: SecondModel {
                value: 1,
                name: "Initial".to_string(),
            },
            indice_model: IndiceModel {
                frame_counter: 0,
                selected_index: None
            },
            fileinput_model: FileInputModel {
                frame_counter: 0,
                riferimento_path: None,
                campionamento_path: None,
            },
            infoaggiuntive_model: InfoAggiuntiveModel {
                frame_counter: 0,
            },
            output_model: OutputModel {
                frame_counter: 0,
            }
        }
    }
}
