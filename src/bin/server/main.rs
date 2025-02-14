
use std::io::{Read, Write};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
mod utils;
use std::thread;
use uuid::Uuid;


use utils::{load_server_config_file, ServerConfig};

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

pub struct Server {
    status: ServerStatus,
    // players: Arc<Mutex<PlayerCollection>>,
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
        println!("Server is listening on port {}" , game_server.port);

        //game_server.wait_for_players();
        game_server.status = ServerStatus::WaitingForPlayers;
        game_server.run();
    }

    fn run(&mut self) {

         println!(
             "[SERVER] Waiting for {} players to connect on port {}...",
             self.number_of_players, self.port
         );

        // This thread listens for the termination command...

        // let running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
        // let running_clone = Arc::clone(&running);
        // thread::spawn(move || {
        //     let mut buffer = String::new();
        //     std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
        //     let mut running = running_clone.lock().unwrap();
        //     *running = false;
        // });

   
        for stream in self.listener.incoming() {
            // {
            //     let running = running.lock().unwrap();
            //     if !*running {
            //         println!("Server is shutting down");
            //         break;
            //     }
            // }
    
            match stream {
            //match self.listener.accept() {
                // Ok((stream, addr)) => {
                //     println!("Client connected");
                //     let mut connections = self.players.lock().unwrap();
                //     connections.insert(addr, stream.try_clone().expect("Failed to clone stream"));
                //     let connections_clone = Arc::clone(&self.players);
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
            let mut buffer = [0; 512];
            loop {
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            println!("Client disconnected");
                            break;
                        }
                        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("player {} says: {}", id, msg);
                        
    
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

    // fn wait_for_players(&mut self) {
    //     self.status = ServerStatus::WaitingForPlayers;

    //     println!(
    //         "[SERVER] Waiting for {} players to connect on port {}...",
    //         self.number_of_players, self.port
    //     );

    //     while self.players.len() < self.number_of_players as usize {
    //         match self.listener.accept() {
    //             Ok((stream, addr)) => {
    //                 println!("[PLAYER CONNECTED] {}", addr);
    //                 self.players.insert(addr, stream);
    //                 println!(
    //                     "[PLAYERS CONNECTED] {}/{}",
    //                     self.players.len(),
    //                     self.number_of_players
    //                 );
    //             }
    //             Err(e) => {
    //                 println!("[ERROR] Could not accept connection: {}", e);
    //             }
    //         }
    //     }

    //     println!("[SERVER] All players connected.");
    // }

