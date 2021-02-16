use std::net::TcpListener;
use std::thread;

const PORT: usize = 8080;
const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4096;

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

fn run_server(conn: TcpListener) {
    println!(
        "Copper server node started on {}",
        conn.local_addr().unwrap()
    );
    conn.set_nonblocking(true)
        .expect("Could not move to non-blocking mode!");

    let mut clients = Vec::new();

    for stream in conn.incoming() {
        if let Ok(socket) = stream {
            println!("Client connected: {}", socket.local_addr().unwrap());
            clients.push(socket.try_clone().expect("Could not clone client socket"));

            thread::spawn(move || loop {
                // TODO implement client structs and set the "should check" flag to false
                let mut buf = [0; MAX_MESSAGE_SIZE];
                let _ = socket.peek(&mut buf);
                let msg = std::str::from_utf8(&buf).expect("Could not parse utf8 message");
                match serde_json::from_str::<String>(msg) {
                    Ok(_) => 
                    {
                        println!("Received a message from the client at {}", socket.local_addr().unwrap())
                        todo!()
                    },
                    Err(_) => todo!(),
                }
            });

        }
    }
}
