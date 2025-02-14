use std::fs::File;
use std::io::BufReader;
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


fn parse_to_command(input: String) -> (String, String) {
  match input.find(" ") {
    Some(index)  => {
      let ( cmd_name, cmd_args ) = input.split_at(index);
      (String::from(cmd_name), String::from(cmd_args))
    },
    None => (input, String::from("")),
  }
}
