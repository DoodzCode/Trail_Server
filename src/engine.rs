use std::io::Write;
use serde_json;
use log::{info, warn, error};
use crate::processors::{
    report_processor::status_report,
    conditions_processor::cycle_conditions
};
use crate::controllers::decision_controller::{
    self, 
    party_to_delay, 
    party_to_proceed
};
use crate::server::{PlayerCollection, ServerStatus};
use crate::structs::game_state::GameState;
use crate::utils::{
    load_game_from_file,
    line_break
};



pub fn game_loop(number_of_players: &u16, players: &mut PlayerCollection) {

    // startup
    env_logger::init();
    
    info!("\n\nsetup:\n");
    // println!();
    // println!("setup:");
    // line_break();
    
    let mut game_state: GameState = load_game_from_file("src/config/game_state.json").expect("Failed to load game data");
    status_report(&mut game_state);
    // println!("{:#?}", &game_state);
    info!("\n{:#?}", &game_state);

    // Serialize the game_state
    let serialized_game_state = serde_json::to_string(&game_state).expect("Failed to serialize game state");

    // Broadcast the serialized game_state to all players
    for (_, player) in players.iter_mut() {
        let _ = player.write_all(serialized_game_state.as_bytes());
    }
    

    // main loop
    loop {
        if game_state.game_date.week_number > game_state.g_duration - 1 { break; }

        // Broadcast message to all players
        for (_, player) in players.iter_mut() {
            let _ = player.write_all("[SERVER BROADCAST] Test Message".as_bytes());
        }
        
        //* conditions_processor -  cycle conditions
        cycle_conditions(&mut game_state);

        //* decision_controller - user prompt for commands
        //* actions_processor - cycle actions    
        //* decision_controller - user prompt - go or no go.

        decision_controller::captains_orders(&mut game_state);

        /*
        for party in &mut game_state.parties {
            print!("{:?} do you want to 1. proceed or 2. delay?", party.name);
            let cmd: String = get_input();     
            let party_name = party.name.clone();
            match cmd.as_str() {
                "1" => party_to_proceed(party), 
                
                "2" => party_to_delay(party),

                _ => println!("Invalid Response")
            } 
            scores.insert(party_name, party.position);
        }   
        */

        // decision_controller();
        // Global Report

        println!("");
        for (key, value) in game_state.score.clone() {
            println!("{}: {}", key, value);
        }
        line_break();    
    }
    // shutdown

}