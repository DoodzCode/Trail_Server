
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};


fn main() {
        
}

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
        // game_loop(&self.number_of_players, &mut self.players);
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
