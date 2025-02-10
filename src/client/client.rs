use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

use crate::server::{ServerStatus, GameServer};
use crate::structs::player::PlayerProfile;
use crate::utils::{load_player_from_file, prompt_user};

const PORT: u16 = 5000;

#[derive(Debug, Deserialize, Serialize)]
struct ClientState {
    player_name: String,
    player_id: u16,
    status: ClientStatus,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ClientStatus {
    Active,
    Inactive,
    Waiting,
    Busy,
    Idle,
}


pub fn game_client() {
    let mut client_status: ClientStatus = ClientStatus::Inactive;
    let mut tcp_connection: Option<TcpStream> = None; // Will eventually hold the TcpStream
    let server_status_main: Arc<Mutex<ServerStatus>> = Arc::new(Mutex::new(ServerStatus::Inactive));

    // welcome
    println!("Welcome to the game!");

    //Establish Player Identity
    //let player_profile: PlayerProfile = load_player_from_file("src/config/profile_ian.json")
    //    .expect("Failed to load player profile");

    // Host or Join
    let response = prompt_user("Do you want to host or join? ( h/j )");

    if response == "h" {
        // Host
        let server_status_clone: Arc<Mutex<ServerStatus>> = Arc::clone(&server_status_main);
        let server_status_client: Arc<Mutex<ServerStatus>> = Arc::clone(&server_status_main);

        // how many player?
        let number_of_players: u16 = prompt_user("How many players?")
            .parse()
            .expect("Failed to parse string to u16");

        // game_engine started on new thread
        thread::spawn(move || GameServer::start(server_status_clone, number_of_players, PORT));

        println!("[CLIENT] Setting up game...");
        sleep(Duration::from_secs(5));

        while *server_status_client.lock().unwrap() != ServerStatus::WaitingForHost {
            print!("...");
            sleep(Duration::from_secs(2));
        }
        println!("{:?}", server_status_client.lock().unwrap());

        println!("Game is ready, establishing connection...");

        if *server_status_client.lock().unwrap() == ServerStatus::WaitingForHost {
            println!("You are the host player.");
            tcp_connection =
                Some(TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to host"));
        }

        client_status = ClientStatus::Waiting;

        {
            let server_status = server_status_client.lock().unwrap();
            match *server_status {
                ServerStatus::WaitingForPlayers => {
                    println!("Waiting for players to join...");
                    while client_status == ClientStatus::Waiting {
                        print!(".");
                        sleep(Duration::from_secs(2));
                    }
                }
                _ => {}
            }
        }
    } else {
        let host_addr = prompt_user("Enter Host IP");
        tcp_connection = Some(TcpStream::connect(host_addr).expect("Failed to connect to host"));
    }

    while client_status != ClientStatus::Inactive {
        
        let response: String = prompt_user("Do you want to continue? (y/n)");
        if response == "n" {
            client_status = ClientStatus::Inactive;
        }

    }
}

