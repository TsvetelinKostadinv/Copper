use rand::Rng;
use serde::{Deserialize, Serialize};

use std::time::Duration;

use crate::common::Executable;

mod util;

const PORT: usize = 8080;

#[derive(Serialize, Deserialize, Debug)]
struct Task;

const SAMPLE_SIZE: usize = 5000;

impl Executable<(), String> for Task {
    fn exec(&self, _args: ()) -> String {
        println!("I am serializable!");
        std::thread::sleep(Duration::from_secs(1));

        let mut rng = rand::thread_rng();

        let mut sum: usize = 0;
        for _ in 0..SAMPLE_SIZE {
            sum += (rng.gen::<u8>() % 2) as usize;
        }
        if sum < SAMPLE_SIZE/2 as usize
        {
            String::from("0")
        }else{
            String::from("1")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Aggregator;

impl Executable<Vec<String>, String> for Aggregator {
    fn exec(&self, args: Vec<String>) -> String {
        println!("I am aggregating!");
        let mut ones: usize = 0;
        for res in args.iter() {
            ones += match res.as_str()
            {
                "0" => 0,
                "1" => 1,
                _ => unreachable!(),
            }
        }
        if ones > args.len() /2 as usize
        {
            "The ones are more".into()
        }else{
            
            "The zeroes are more".into()
        }
        
    }
}

pub fn main() {
    println!("You have started a server node!");
    let mut server =
        util::Server::new::<Task, Aggregator>(PORT, Box::new(Task), Box::new(Aggregator));
    while server.repl() {}

    println!("Server exited!");
}
