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

    pub fn set_value(&mut self, val : i32) {
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

    pub fn set_value(&mut self, val : i32) {
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
    value: i32,
    name: String,
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct FileInputModel {
    value: i32,
    name: String,
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct InfoAggiuntiveModel {
    value: i32,
    name: String,
}

// State struct holding non-`Copy` types
#[derive(Clone)]
pub struct OutputModel {
    value: i32,
    name: String,
}

#[derive(Clone)]
pub struct Model {
    pub home_model : HomeModel,
    pub second_model : SecondModel,
    pub indice_model : IndiceModel,
    pub fileinput_model : FileInputModel,
    pub infoaggiuntive_model : InfoAggiuntiveModel,
    pub output_model : OutputModel
}

impl Model {
    pub fn new() -> Self {
        Self {
            home_model : HomeModel {
                value : 0,
                name : "Initial".to_string(),
            },
            second_model : SecondModel {
                value : 1,
                name : "Initial".to_string(),
            },
            indice_model : IndiceModel {
                value : 2,
                name : "Initial".to_string(),
            },
            fileinput_model : FileInputModel {
                value : 3,
                name : "Initial".to_string(),
            },
            infoaggiuntive_model : InfoAggiuntiveModel {
                value : 4,
                name : "Initial".to_string(),
            },
            output_model : OutputModel {
                value : 5,
                name : "Initial".to_string(),
            }
        }
    }
}
