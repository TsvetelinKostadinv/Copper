use std::io::stdin;

mod server;
mod client;
mod common;

fn print_ascii_art() {
    println!(
        r#"
_________                                    
\_   ___ \  ____ ______ ______   ___________ 
/    \  \/ /  _ \\____ \\____ \_/ __ \_  __ \
\     \___(  <_> )  |_> >  |_> >  ___/|  | \/
 \______  /\____/|   __/|   __/ \___  >__|   
        \/       |__|   |__|        \/       
        "#
    );
}

fn main() {
    print_ascii_art();

    println!("Hello, welcome to copper!");
    println!("What mode shall we start?: ");
    println!("[S]erver mode (server)");
    println!("[C]hild mode (node)");
    println!("[E]xit");
    let mut choice = String::new();
    let mut chosen = false;
    while !chosen {
        choice.clear();
        let read = stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice from stdin!");
        if read == 0 {
            continue;
        }
        chosen = match choice.trim() {
            "server" | "s" | "Server" | "S" => {
                server::main();
                true
            }
            "child" | "c" | "Child" | "C" => {
                client::main();
                true
            }
            "exit" | "e" | "Exit" | "E" => true,
            _ => {
                println!(
                    r#"Didn't get that
                        try with one of these: 
                        - <parent>, <Parent>, <p> or <P> for server mode
                        - <child>, <Child>, <c> or <C> for child mode"#
                );
                false
            }
        }
    }
    println!("Done the work, exited.");
}
