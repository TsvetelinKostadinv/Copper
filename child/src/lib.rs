use std::env;

const INTER_FLAG: &'static str = "-i";

pub fn main() {
    println!("You have started a child node!");
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    let _callsite = args.pop();
    match args.pop() {
        Some(command) => 
        if command == INTER_FLAG
        {
            interactive_session()
        },
        None => automatic_session(),
    }
}

fn automatic_session() {
    // parse config file
}

fn interactive_session() {}
