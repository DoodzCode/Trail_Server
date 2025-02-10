use rand::Rng;
use serde::Serialize;
use std::fs::{File, remove_file};
use std::io::{Write, BufReader, stdin};
use std::path::Path;
use crate::structs::game_state::GameState;

pub fn d20() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..=20)
}

pub fn save_to_file<T: Serialize>(data: &T, filename: &str) -> std::io::Result<()> {
  let path: std::path::PathBuf = Path::new("src/config").join(filename);
  let json: String = serde_json::to_string(data)?;
  let mut file: File = File::create(path)?;
  file.write_all(json.as_bytes())?;
  Ok(())
}

pub fn load_game_from_file(filename: &str) -> serde_json::Result<GameState> {
  let file = File::open(filename).map_err(serde_json::Error::io)?;
  let reader = BufReader::new(file);
  let game_state: GameState = serde_json::from_reader(reader)?;
  Ok(game_state)
}


pub fn line_break() {
  println!(" - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - ");
  println!();
}
