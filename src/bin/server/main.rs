use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

mod utils;

use utils::{load_server_config_file, ServerConfig};

fn main() {
    Server::start();
}

pub type PlayerCollection = HashMap<SocketAddr, TcpStream>;

#[derive(Debug, PartialEq)]
pub enum ServerStatus {
    Starting,
    WaitingForPlayers,
    HostingGame,    // Just sits until something needs to be sent/received?
    Busy,
    Waiting,
    Inactive,
}

pub struct Server {
    status: ServerStatus,
    players: PlayerCollection,
    number_of_players: u16,
    port: u16,
    listener: TcpListener,
}

impl Server {
    pub fn start() {
        let server_config: ServerConfig = load_server_config_file("src/config/server.json").unwrap();

        let mut game_server: Server = Server {
            status: ServerStatus::Starting,
            players: HashMap::new(),
            number_of_players: server_config.number_of_players,
            port: server_config.port,
            listener: TcpListener::bind(("0.0.0.0", server_config.port))
                .expect("Could not bind to port"),
        };

        game_server.run();
    }

    fn run(&mut self) {
        self.wait_for_players();

        // Set up the GameConfig, save it to a file, then start up the engine?
    }

    fn wait_for_players(&mut self) {
        self.status = ServerStatus::WaitingForPlayers;

        println!(
            "[SERVER] Waiting for {} players to connect on port {}...",
            self.number_of_players, self.port
        );

        while self.players.len() < self.number_of_players as usize {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    println!("[PLAYER CONNECTED] {}", addr);
                    self.players.insert(addr, stream);
                    println!(
                        "[PLAYERS CONNECTED] {}/{}",
                        self.players.len(),
                        self.number_of_players
                    );
                }
                Err(e) => {
                    println!("[ERROR] Could not accept connection: {}", e);
                }
            }
        }

        println!("[SERVER] All players connected.");
        self.status = ServerStatus::Busy;
    }
}
