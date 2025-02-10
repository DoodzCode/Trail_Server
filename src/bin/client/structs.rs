use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::{self, BufReader, Write, stdout, stdin};
use std::fs::{remove_file, File};

const PORT: u16 = 5000;


#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
  name: String,
  id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ClientStatus {
  Connecting,
  Waiting,
  IssuingTasksOrders,
  IssuingCaptainsOrders,
  Inactive,
}

pub struct Client {
  status: ClientStatus,
  player_profile: PlayerProfile,
  // tcp_stream: TcpStream,
  // host_addr: String,
}

impl Client {

  pub fn start() {
    let player_profile: PlayerProfile = load_player_from_file("src/config/profile_ian.json")
      .expect("Failed to load player profile");
    
    // let host_ip: String = prompt_user("What is the host address?");
    // let host_addr = format!("{}:{}", host_ip, PORT);

    // println!("Connecting to host: {}...", host_addr);
    // let tcp_stream: TcpStream = TcpStream::connect(&host_addr).expect("Failed to connect to host");
    // println!("Connected to host: {}", host_addr);

    let mut client: Client = Self {
      status: ClientStatus::Waiting,
      player_profile,
      // tcp_stream,
      // host_addr,
    };

    client.run();
  }

  fn run(&mut self) {
    loop {
      match self.status {
        ClientStatus::Waiting => {
          // Waiting for signal from server
          self.handle_waiting_commands();
          // self.status = ClientStatus::IssuingTasksOrders;
        },
        ClientStatus::IssuingTasksOrders => {
          self.status = ClientStatus::IssuingCaptainsOrders;
        },
        ClientStatus::IssuingCaptainsOrders => {
          self.status = ClientStatus::Inactive;
        },
        ClientStatus::Inactive => {
          break;
        },
        _ => {},
      }

    }
  }

  fn print_hud(&self) {
    println!("");
    println!("--- HUD ---");
    println!("Player: {}", self.player_profile.name);
    println!("ID: {}", self.player_profile.id);
    println!("Status: {:?}", self.status);
    println!("");
  }

  fn print_status(&self) {
    println!("Status: {:?}", self.status);
  }

  fn handle_waiting_commands(&mut self) {
    self.print_hud();
    let user_input = prompt_user("What would you like to do?");

    match user_input.as_str() {
      "exit" => {
        self.status = ClientStatus::Inactive;
      },
      "status" => {
        self.print_status();
      },
      _ => {
        println!("Invalid command.");
      },
    }
  }

}

fn load_player_from_file(filename: &str) -> serde_json::Result<PlayerProfile> {
  let file: File = File::open(filename).map_err(serde_json::Error::io)?;
  let reader: BufReader<File> = BufReader::new(file);
  let player_profile: PlayerProfile = serde_json::from_reader(reader)?;
  Ok(player_profile)
}

fn prompt_user(prompt: &str) -> String {
  print!("{}: ", prompt);
  stdout().flush().unwrap();
  let mut response: String = String::new();
  stdin().read_line(&mut response).unwrap();
  response.trim().to_string()
}