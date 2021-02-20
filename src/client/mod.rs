use std::io::stdin;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use crate::common::{deserialize, serialize, Dummy, Msg, Type};

// const INTER_FLAG: &'static str = "-i";

const PORT: usize = 8080;
const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

pub fn main() {
    println!("You have started a child node!");
    println!("On what IP would you like to connect");
    let mut choice = String::new();
    let _ = stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice from stdin!");
    let host = match choice.as_str().trim() {
        "" => LOCALHOST,
        _ => choice.trim(),
    };
    let mut port = PORT;
    let mut str_port = String::new();
    let mut done = false;
    while !done {
        str_port.clear();
        println!("Input port to connect to:");
        let _ = stdin()
            .read_line(&mut str_port)
            .expect("Failed to read choice from stdin!");
        println!("For the port read: {}", str_port);
        match str_port.trim().parse::<usize>() {
            Ok(input) => {
                port = input;
                done = true;
            }
            _ => {}
        };
    }

    automatic_session(host, port);
}

fn automatic_session(host: &str, mut port: usize) {
    println!("Copper Child node started in automatic mode");
    let mut bound = false;
    while !bound {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr);
        match stream {
            Ok(conn) => {
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
    println!("Child node exited!");
}

fn next_port(port: usize) -> usize {
    port + 1
}

fn run_client(mut conn: TcpStream) {
    println!("Child node connected to {}", conn.peer_addr().unwrap());

    let handle = std::thread::spawn(move || {
        let mut buf = [0; MAX_MESSAGE_SIZE];
        let _ = conn.peek(&mut buf).expect("Cannot peek in the buffer");
        let mut msg = buf.to_vec();

        msg.retain(|&el| el != 0);
        let msg = std::str::from_utf8(&msg).expect("Could not parse uft8 string");
        let task = deserialize(msg.into());

        let res = task.func.exec(()); // piece de resistance of the whole project...

        // let mut read = String::new();
        let _ = conn
            .read(&mut buf)
            .expect("Could not read the peeked bytes!");

        let end_msg = Msg {
            type_msg: Type::Result,
            res,
            func: Box::new(Dummy),
        };

        conn.write(serialize(end_msg).as_bytes())
            .expect("Could not write to the server!");

        conn.flush().expect("Could not flush to server!");

        println!("Wrote to server!");

        loop {
            let _ = conn.peek(&mut buf).expect("Cannot peek in the buffer");
            let mut msg = buf.to_vec();
            msg.retain(|&el| el != 0);
            let msg = std::str::from_utf8(&msg).expect("Could not parse uft8 string");
            let end_msg = deserialize(msg.into());
            if end_msg.type_msg == Type::ThankYou {
                break;
            }
        }

        conn.shutdown(Shutdown::Both)
            .expect("Could not close the connection");
    });

    let _ = handle.join();
}
