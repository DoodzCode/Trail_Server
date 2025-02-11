use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::str;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2925").expect("Could not bind");
    let running = Arc::new(Mutex::new(true));

    println!("Server is listening on port 2925");

    let running_clone = Arc::clone(&running);
    thread::spawn(move || {
        // This thread listens for the termination command
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
        let mut running = running_clone.lock().unwrap();
        *running = false;
    });

    for stream in listener.incoming() {
        {
            let running = running.lock().unwrap();
            if !*running {
                println!("Server is shutting down");
                break;
            }
        }

        match stream {
            Ok(mut stream) => {
                println!("Client connected");
                let mut buffer = [0; 512];
                loop {
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                println!("Client disconnected");
                                break;
                            }

                            match str::from_utf8(&buffer[..bytes_read]) {
                                Ok(client_message) => {
                                    println!("Client says: {}", client_message);
                                }
                                Err(_) => {
                                    println!("Received invalid UTF-8 message: {:?}", &buffer[..bytes_read]);
                                }
                            }

                            stream.write_all(b"Trail Server > ").expect("Could not write to stream");
                        }
                        Err(e) => {
                            println!("Failed to read from stream: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
