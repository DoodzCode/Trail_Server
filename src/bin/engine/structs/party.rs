use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Party {
    pub name: String,
    pub position: u16,
    pub head_count: u16,
    // pub wagons: Vec<Wagon>,
}

impl Party {
    pub fn create(name: &str) -> Party {
        Party {
            name: String::from(name),
            position: 0,
            head_count: 140,
        }
    }

    pub fn increment_position(&mut self, distance: u16) {
        self.position = self.position + distance;
    }

    pub fn decrement_position(&mut self, distance: u16) {
        self.position = self.position - distance;
    }

    pub fn increment_head_count(&mut self, amount: u16) {
        self.head_count = self.head_count + amount;
    }

    pub fn decrement_head_count(&mut self, amount: u16) {
        self.head_count = self.head_count - amount;
    }

    pub fn give_position(&self) -> &u16 {
        &self.position
    }
}
