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

#[derive(Clone)]
pub struct Model {
    pub home_model : HomeModel,
    pub second_model : SecondModel,
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
            }
        }
    }
}
