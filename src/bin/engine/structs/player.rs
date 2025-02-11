
use serde::{Deserialize, Serialize};

pub struct Player {
  name: String,
  
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
  name: String,
  id: String,
}