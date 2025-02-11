use crate::structs::game_state::{GameState, ActionType, Message};

// Update Game Date
// let current_date = start_date + (i * 7);
// let current_date = start_date + "00".to_string();

pub fn cycle_conditions(game_state: &mut GameState) {
    
    // calandar
    game_state.change_state(Message{
        action: ActionType::IncWeek,
    });
    game_state.game_date.day_of_year += 7;

    // let mut week: u8 = game_state::GameDate::increment_week(&mut self);
    println!("Week # {} | DOY {} \n", game_state.game_date.week_number, game_state.game_date.day_of_year);


    // biomes




    // determine which biomes have player in them

    // loop through active biomes

    // output: chance_of_snow = snow_factor();
    // input: d20
    // input: is date in winter

    // legs

    // cycle through the parties
    fn cycle_parties(party_count: u32) {}
}
