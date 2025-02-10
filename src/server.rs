use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use crate::engine::game_loop;

pub type PlayerCollection = HashMap<SocketAddr, TcpStream>;

#[derive(Debug, PartialEq)]
pub enum ServerStatus {
    WaitingForHost,
    WaitingForPlayers,
    Busy,
    Idle,
    Active,
    Inactive,
}

pub struct GameServer {
    status: Arc<Mutex<ServerStatus>>,
    players: PlayerCollection,
    number_of_players: u16,
    port: u16,
}

impl GameServer {
    pub fn start(status: Arc<Mutex<ServerStatus>>, number_of_players: u16, port: u16) {
        
        let mut players: PlayerCollection = wait_for_players(number_of_players, port, &status);
        for (addr, _) in players.iter() {
            println!("[PLAYER] {} is ready!", addr);
        }
        
        {
            let mut server_status_lock = status.lock().unwrap();
            *server_status_lock = ServerStatus::WaitingForHost;
            drop(server_status_lock);
        }

        let mut game_server: GameServer = GameServer {
            status,
            players: HashMap::new(),
            number_of_players,
            port,
        };

        game_server.run();
    }

    fn run(&mut self) {
        game_loop(&self.number_of_players, &mut self.players);
    }
}



pub fn wait_for_players(number_of_players: u16, port: u16, server_status: &Arc<Mutex<ServerStatus>>) -> PlayerCollection {
    let listener: TcpListener = TcpListener::bind(("0.0.0.0", port)).expect("Could not bind to port");

    {
        let mut server_status_lock = server_status.lock().unwrap();
        *server_status_lock = ServerStatus::WaitingForHost;
        drop(server_status_lock);
    }

    let mut players: PlayerCollection = HashMap::new();
    let mut host_joined: bool = false;

    println!(
        "[SERVER] Waiting for {} players to connect on port {}...",
        number_of_players, port
    );

    // TODO: May need something about making sure the first connection comes from 127.0.0.1
    while players.len() < number_of_players as usize {
        
        match listener.accept() {

            Ok((stream, addr)) => {
                
                if !host_joined && addr.ip().to_string() == "127.0.0.1"{
                    println!("[HOST CONNECTED] {}", addr);
                    players.insert(addr, stream);
                    host_joined = true;
                    {
                        let mut server_status_lock = server_status.lock().unwrap();
                        *server_status_lock = ServerStatus::WaitingForPlayers;
                        drop(server_status_lock);
                    }

                } else if host_joined {
                    println!("[PLAYER CONNECTED] {}", addr);
                    players.insert(addr, stream);
                    println!(
                        "[PLAYERS CONNECTED] {}/{}",
                        &players.len(),
                        number_of_players
                    );

                } else {
                    println!("[ERROR] Host must connect first.");
                }
            }
            Err(e) => {
                println!("[ERROR] Could not accept connection: {}", e);
            }
        }
    }

    println!("[SERVER] All players connected.");

    players
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::thread;
//     use std::time::Duration;

//     #[test]
//     fn test_wait_for_players() {
//         let port = 12345;
//         let number_of_players = 2;

//         thread::spawn(move || {
//             let _ = wait_for_players(number_of_players, port);
//         });

//         thread::sleep(Duration::from_millis(100));

//         for _ in 0..number_of_players {
//             let _ = TcpStream::connect(("127.0.0.1", port)).expect("Could not connect to server");
//         }
//     }

//     #[test]
//     fn test_server_status_enum() {
//         assert_eq!(ServerStatus::WaitingForHost, ServerStatus::WaitingForHost);
//         assert_eq!(
//             ServerStatus::WaitingForPlayers,
//             ServerStatus::WaitingForPlayers
//         );
//         assert_eq!(ServerStatus::Busy, ServerStatus::Busy);
//         assert_eq!(ServerStatus::Idle, ServerStatus::Idle);
//         assert_eq!(ServerStatus::Active, ServerStatus::Active);
//     }

//     #[test]
//     fn test_player_collection() {
//         let mut players: PlayerCollection = HashMap::new();
//         let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//         let stream = TcpStream::connect(addr).expect("Could not connect to server");

//         players.insert(addr, stream);
//         assert_eq!(players.len(), 1);
//         assert!(players.contains_key(&addr));
//     }
// }
