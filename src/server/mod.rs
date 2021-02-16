use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use crate::common::{deserialize, serialize, Executable, Msg, Type};

const PORT: usize = 8080;
const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

pub fn main() {
    println!("You have started a server node!");
    let mut port = PORT;
    let host_addr = LOCALHOST;
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
struct Task;

impl Executable<(), String> for Task {
    fn exec(&self, _args: ()) -> String {
        println!("I am serializable!");
        String::from("test")
    }
}

fn run_server(conn: TcpListener) {
    println!(
        "Copper server node started on {}",
        conn.local_addr().unwrap()
    );
    // conn.set_nonblocking(true)
    //     .expect("Could not move to non-blocking mode!");

    let mut clients = Vec::new();

    for stream in conn.incoming() {
        if let Ok(mut socket) = stream {
            println!("Client connected: {}", socket.peer_addr().unwrap());
            clients.push(socket.try_clone().expect("Could not clone client socket"));

            let task = Task;
            let send_msg = Msg {
                type_msg: Type::Task,
                res: "".to_string(),
                func: Box::new(task),
            };

            socket
                .write(serialize(send_msg).as_bytes())
                .expect("Could not write to the client");

            socket.flush().expect("Could not flush to the client");

            println!(
                "Sent the client at {} their task",
                socket
                    .peer_addr()
                    .expect("Unable to take the address of the client")
            );

            thread::spawn(move || loop {
                // TODO implement client structs and set the "should check" flag to false
                // if(socket.)
                let mut buf = [0; MAX_MESSAGE_SIZE];
                let _ = socket.peek(&mut buf).expect("Cannot peek in the buffer");
                let mut msg = buf.to_vec();

                msg.retain(|&el| el != 0);
                let msg = std::str::from_utf8(&msg).expect("Could not parse uft8 string");

                let msg = deserialize(msg.into());
                println!(
                    "Received a message from the client at {}",
                    socket.peer_addr().unwrap()
                );
                println!("{} : {}", socket.peer_addr().unwrap(), msg.res);
                let _ = socket.read(&mut buf);
                println!("Thanking the client...");
                let thank_you_msg = Msg {
                    type_msg: Type::ThankYou,
                    res: "".to_string(),
                    func: Box::new(Task),
                };
                socket
                    .write(serialize(thank_you_msg).as_bytes())
                    .expect("Could not write thank you to client!");

                println!("Thank you sent...");
                break;
                // println!("Received: |{:?}|", msg);

                // match serde_json::from_str::<Msg>(msg) {
                //     Ok(msg) => {
                //         println!(
                //             "Received a message from the client at {}",
                //             socket.peer_addr().unwrap()
                //         );
                //         println!("{} : {}", socket.peer_addr().unwrap(), msg.res);
                //         let _ = socket.read_exact(&mut buf);
                //         println!("Thanking the client...");
                //         let thank_you_msg = Msg {
                //             type_msg: Type::ThankYou,
                //             res: "".to_string(),
                //             func: Box::new(Task),
                //         };

                //         socket
                //             .write(serialize(thank_you_msg).as_bytes())
                //             .expect("Could not write thank you to client!");

                //         println!("Thank you sent...");
                //         // socket
                //         //     .shutdown(Shutdown::Both)
                //         //     .expect("Could not end connection with client");
                //     }
                //     Err(err) => {
                //         println!("{:?}", err);
                //         println!("{:?}", msg);
                //         continue;
                //     }
                // }
            });
        }
    }
}
