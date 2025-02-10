use crate::structs::PlayerProfile;
use std::fs::{remove_file, File};
use std::io::{stdin, BufReader, Write};

pub fn load_player_from_file(filename: &str) -> serde_json::Result<PlayerProfile> {
    let file = File::open(filename).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    let player_profile: PlayerProfile = serde_json::from_reader(reader)?;
    Ok(player_profile)
}

pub fn get_input() -> String {
    let mut r_input: String = String::new();
    stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input).to_lowercase()
}

pub fn prompt_user(msg: &str) -> String {
    println!("{} ", msg);
    let mut r_input: String = String::new();
    stdin().read_line(&mut r_input).unwrap();
    let input: &str = r_input.trim();
    String::from(input).to_lowercase()
}
