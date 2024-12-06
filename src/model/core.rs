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
}

impl IndiceModel {
    pub fn _get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
    }
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct FileInputModel {
    frame_counter: u32,
}

impl FileInputModel {
    pub fn get_frame_counter(&self) -> u32 {
        return self.frame_counter;
    }
    pub fn increment_frame_counter(&mut self) {
        self.frame_counter += 1;
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
    pub fn get_frame_counter(&self) -> u32 {
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
            },
            fileinput_model: FileInputModel {
                frame_counter: 0,
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
