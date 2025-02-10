mod utils;
mod structs;

use crate::structs::Client;

fn main() {
    println!("Welcome to the game!");
    
    Client::start();
    
    // if response == "h" {

    //     // Host
    //     let server_status_clone: Arc<Mutex<ServerStatus>> = Arc::clone(&server_status_main);
    //     let server_status_client: Arc<Mutex<ServerStatus>> = Arc::clone(&server_status_main);

    //     // how many player?
    //     let number_of_players: u16 = prompt_user("How many players?")
    //         .parse()
    //         .expect("Failed to parse string to u16");

    //     // game_engine started on new thread
    //     thread::spawn(move || GameServer::start(server_status_clone, number_of_players, PORT));

    //     println!("[CLIENT] Setting up game...");
    //     sleep(Duration::from_secs(5));

    //     while *server_status_client.lock().unwrap() != ServerStatus::WaitingForHost {
    //         print!("...");
    //         sleep(Duration::from_secs(2));
    //     }
    //     println!("{:?}", server_status_client.lock().unwrap());

    //     println!("Game is ready, establishing connection...");

    //     if *server_status_client.lock().unwrap() == ServerStatus::WaitingForHost {
    //         println!("You are the host player.");
    //         tcp_connection =
    //             Some(TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to host"));
    //     }

    //     client_status = ClientStatus::Waiting;

    //     {
    //         let server_status = server_status_client.lock().unwrap();
    //         match *server_status {
    //             ServerStatus::WaitingForPlayers => {
    //                 println!("Waiting for players to join...");
    //                 while client_status == ClientStatus::Waiting {
    //                     print!(".");
    //                     sleep(Duration::from_secs(2));
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // } else {
    //     let host_addr = prompt_user("Enter Host IP");
    //     tcp_connection = Some(TcpStream::connect(host_addr).expect("Failed to connect to host"));
    // }

}
