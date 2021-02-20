use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::common::Executable;

mod structs;

const PORT: usize = 8080;

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
    let mut server = structs::Server::new::<Task, Aggregator>(PORT, Box::new(Task), Box::new(Aggregator));
    while server.repl() {}

    println!("Server exited!");
}
