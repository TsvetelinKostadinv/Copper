use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use std::thread::JoinHandle;

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::{deserialize, serialize, Dummy, Executable, Msg, Type};

mod structs;

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

#[derive(Serialize, Deserialize, Debug)]
struct Task;

impl Executable<(), String> for Task {
    fn exec(&self, _args: ()) -> String {
        println!("I am serializable!");
        std::thread::sleep(Duration::from_secs(1));
        String::from("test")
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Aggregator;

impl Executable<Vec<String>, String> for Aggregator {
    fn exec(&self, args: Vec<String>) -> String {
        println!("I am aggregating!");
        for res in args {
            println!("{}", res);
        }
        "Done the work".into()
    }
}

pub fn main() {
    println!("You have started a server node!");
    let mut server = structs::Server::new::<Task, Aggregator>(8080, Box::new(Task), Box::new(Aggregator));
    while server.repl() {}

    println!("Server exited!");
}
