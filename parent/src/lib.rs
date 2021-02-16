use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::thread;
use std::io::{Write, Read};

const PORT: usize = 8080;
const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

pub fn main() {
    println!("You have started a server node!");
    let mut port = PORT;
    let mut host_addr = LOCALHOST;
    let mut bound = false;
    let mut conn;
    while !bound {
        let addr = format!("{}:{}", host_addr, port);
        let listener = TcpListener::bind(addr);
        match listener {
            Ok(tcp_listener) => {
                bound = true;
                conn = tcp_listener;
                run_server(conn);
            }
            Err(_) => {
                println!(
                    "Could not bind in port {}, trying {}",
                    port,
                    next_port(port)
                );
                port = next_port(port);
            }
        }
    }
    println!("Server exited!");
}

fn next_port(port: usize) -> usize {
    port + 1
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    msg: &'a str,
    num: usize,
}

fn run_server(conn: TcpListener) {
    println!(
        "Copper server node started on {}",
        conn.local_addr().unwrap()
    );
    conn.set_nonblocking(true)
        .expect("Could not move to non-blocking mode!");

    let mut clients = Vec::new();

    for stream in conn.incoming() {
        if let Ok(mut socket) = stream {
            println!("Client connected: {}", socket.peer_addr().unwrap());
            clients.push(socket.try_clone().expect("Could not clone client socket"));

            thread::spawn(move || loop {
                // TODO implement client structs and set the "should check" flag to false
                // if(socket.)
                let mut buf = [0; MAX_MESSAGE_SIZE];
                let _ = socket.peek(&mut buf).expect("Cannot peek in the buffer");
                let mut msg = buf.to_vec();

                msg.retain(|&el| el != 0);
                let msg = std::str::from_utf8(&msg).expect("Could not parse uft8 string");

                // println!("Received: |{:?}|", msg);
                match serde_json::from_str::<Message>(msg) {
                    Ok(msg) => {
                        println!(
                            "Received a message from the client at {}",
                            socket.peer_addr().unwrap()
                        );
                        println!("{} : {:?}", socket.peer_addr().unwrap(), msg);
                        let _ = socket.read_exact(&mut buf);
                    }
                    Err(err) => {
                        continue;
                    }
                }
            });
        }
    }
}
