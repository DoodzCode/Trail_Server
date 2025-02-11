// Anything that handles inputs from various sources?
// Listening for inputs and directing inputs to appropriate places

use crate::structs::party::Party;
use crate::structs::game_state::{GameState, ActionType, Message};
use crate::utils;
use std::collections::HashMap;
use std::io::{self, Write};

/// collect initial data to start game
pub fn get_initial_data() {
  // println!("how many players");
  // add player
  // name of party
  // manifest
}

// collect data to present

// prompt for player input
pub fn party_to_proceed(party: &mut Party) {
  println!("{:?} decides to proceed. \n", party.name);
  party.increment_position(80);
  // party.position
}

pub fn party_to_delay(party: &mut Party) {
  println!("{:?} decides to delay.", party.name)
}

// prompt for player input
pub fn captains_orders(game_state: &mut GameState) {
  let mut scores = HashMap::new();

  for party in &mut game_state.parties {
    print!("{:?} do you want to 1. proceed or 2. delay?", &party.name);
    io::stdout().flush().unwrap();
    
    let cmd: String = utils::get_input();     
    let party_name = String::from(&party.name);
    match cmd.as_str() {
        "1" => party_to_proceed(party), 
        
        "2" => party_to_delay(party),

        _ => println!("Invalid Response")
    } 
    scores.insert(party_name, party.position);
  }   
  game_state.score = scores;
  
}

