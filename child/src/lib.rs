use std::env;
use std::io::{Write, Read};
use std::net::{Shutdown, TcpStream};
use serde::{Deserialize, Serialize};

const INTER_FLAG: &'static str = "-i";

const PORT: usize = 8080;
const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

pub fn main() {
    println!("You have started a child node!");
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    let _callsite = args.pop();
    match args.pop() {
        Some(command) => {
            if command == INTER_FLAG {
                interactive_session()
            }
        }
        None => automatic_session(),
    }
}

fn automatic_session() {
    println!("Copper Child node started in automatic mode (for interactive mode supply flag \"-i\")");
    let mut port = PORT;
    let mut host_addr = LOCALHOST;
    let mut bound = false;
    while !bound {
        let addr = format!("{}:{}", host_addr, port);
        let stream = TcpStream::connect(addr);
        match stream {
            Ok(mut conn) => {
                bound = true;
                run_client(conn);
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
    println!("Child exited!");
}

fn next_port(port: usize) -> usize {
    port + 1
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a>
{
    msg:&'a str,
    num: usize,
}

fn run_client(mut conn: TcpStream) {
    println!("Client node connected to {}", conn.peer_addr().unwrap());
    conn.set_nonblocking(true)
        .expect("Failed to initialize nonblocking");
    let msg = Message{
        msg: "Hello from the other side!",
        num: 42,
    };
    conn.write( serde_json::to_string(&msg).expect("Unable to serialize message").as_bytes());
    // let mut buf = [0; MAX_MESSAGE_SIZE];
    // let read_res = conn.read(&mut buf);
    // match read_res
    // {
    //     Ok(_) => {
    //     },
    //     Err(err) => println!("Encountered error!"),
    // }

    conn.shutdown(Shutdown::Both);
}

fn interactive_session() {}
