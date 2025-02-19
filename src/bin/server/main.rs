
use std::io::{Read, Write};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::borrow::Cow;
use serde::Deserialize;
use serde_json;
mod utils;

use std::thread;
use uuid::Uuid;

use utils::{load_server_config_file, ServerConfig, get_input};

fn main() {
    Server::start();
}

pub type PlayerCollection = HashMap<Uuid, String>;

#[derive(Debug, PartialEq)]
pub enum ServerStatus {
    Starting,
    WaitingForPlayers,
    Busy,
    Running,
    Waiting,
    Inactive,
}

#[derive(Deserialize)]
enum Directive {
    Proceed,
}
#[derive(Deserialize)]
pub struct Order {
    directive: Directive,
    arguments: Vec<String>,     // {directive: proceed, value: tre}          // the action ... collect{}, trade{}, proceed, delay, repair{}, build{}, fight{}, scout{}
    // assignment: String,             // person, party or empty in cases like the captain's orders
}

#[derive(Deserialize)]
pub struct Orders {
    convoy: String,
    orders: Vec<Order>,
}

pub struct Server {
    status: ServerStatus,
    players: PlayerCollection,
    number_of_players: u16,
    port: u16,
    listener: TcpListener,
    orders_list: Vec<Orders>,    
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
            orders_list: Vec::new(),
        };
        println!("Server is listening on port {}" , game_server.port);
        game_server.status = ServerStatus::WaitingForPlayers;
        game_server.run();
    }

    fn run(&mut self) {
         println!(
             "[SERVER] Waiting for {} players to connect on port {}...",
             self.number_of_players, self.port
         );

        // KILL SWITCH - thread listens for the termination command
        let running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        let running_clone: Arc<Mutex<bool>> = Arc::clone(&running);
        thread::spawn(move || {
            loop {
                let user_input = get_input();
                match user_input.as_str() {
                    "shutdown" => {
                        println!("Shutting down server");
                        let mut running_clone_lock = running_clone.lock().unwrap();
                        *running_clone_lock = false;
                        drop(running_clone_lock);
                    },
                    _ => {
                        println!("{}", user_input);
                    }
                }
            }
            // let mut running = running_clone.lock().unwrap();
            // *running = false;
        });

        for stream in self.listener.incoming() {
            {
                let running = running.lock().unwrap();
                if !*running {
                    println!("Server is shutting down");
                    break;
                }
            }
    
            match stream {
                Ok(stream) => {
                    match self.players.len() as usize {
                        len if len <= self.number_of_players as usize => {
                            let id = Uuid::new_v4();
                            self.players.insert(id, String::from(""));
                            // spawn a new thread here
                            thread::spawn(move || {
                                handle_client(id, stream);
                            });
                            if len == self.number_of_players as usize {
                                println!("[SERVER] All players connected.");        
                            }
                        },
                        _ => {
                            send_to_latecomer(stream);
                        }   
                    }
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }

        fn handle_client(id: Uuid, mut stream: TcpStream)  {
            stream.write_all(b"Trail Server > ").expect("Could not write to stream");
            stream.write_all(b"").expect("cant");
            // let mut buffer = [0; 512];
            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            println!("Client disconnected");
                            break;
                        }
                        let msg: Cow<'_, str> = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("from {} > {}", id, msg);

                        let orders: Orders = serde_json::from_slice(&buffer[..bytes_read]).expect("Failed to deserialize JSON");                  
                        


                        // do this only if using a terminal - the client isn't looking for a return
                        // stream.write_all(b"Trail Server > ").expect("Could not write to stream");
                    }
                    Err(e) => {
                        println!("Failed to read from stream: {}", e);
                        break;
                    }
                }
            }
        }

        // acceessible from the engine
        fn send_to_client(id:Uuid, message: String) {
        }

        fn send_to_latecomer(mut stream: TcpStream) {
            stream.write_all(b"Trail Server Game in Progress. Go Away.\n").expect("Could not write to stream");
        }

        // self.status = ServerStatus::Running;
        // Set up the GameConfig, save it to a file, then start up the engine?

    }

}

