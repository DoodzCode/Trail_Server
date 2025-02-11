use serde::{Serialize, Deserialize};
use core::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    id: String,
    name: String,
    l_type: String,
    elevation: u32,
    mile_marker: u32,
}

impl Location {

    pub fn create(id: &str, name: &str, l_type: &str, elevation: u32, mile_marker: u32) -> Location {
        Location {
            id: String::from(id),
            name: String::from(name),
            l_type: String::from(l_type),
            elevation,
            mile_marker,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Location.name: {} \n Location.type: {} \n elevation: {} \n", 
        self.name, self.l_type, self.elevation)
    }
}