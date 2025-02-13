use std::fs::File;
use std::io::{BufReader, stdin, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
  pub port: u16,
  pub number_of_players: u16,
}

pub fn load_server_config_file(filename: &str) -> serde_json::Result<ServerConfig> {
  let file = File::open(filename).map_err(serde_json::Error::io)?;
  let reader = BufReader::new(file);
  let server_config: ServerConfig = serde_json::from_reader(reader)?;
  Ok(server_config)
}

pub fn get_input() -> String {
  let mut r_input: String = String::new();
  stdin().read_line(&mut r_input).unwrap();
  let input: &str = r_input.trim();
  String::from(input).to_lowercase()
}