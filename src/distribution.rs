use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Distribution {

    #[serde(default)]
    id: u32,
    name: Option<String>

}

#[allow(dead_code)]
impl Distribution {

    pub fn new() -> Self {
        Self {
            id: 0,
            name: None
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

}
