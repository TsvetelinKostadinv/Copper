use std::io::stdin;

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
    println!("[P]arent mode (server)");
    println!("[C]hild mode (node)");
    let mut choice = String::new();
    let mut chosen = false;
    while !chosen {
        let _ = stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice from stdin!");
        chosen = match choice.trim() {
            "parent" | "p" | "Parent" | "P" => {
                todo!();
                true
            }
            "child" | "c" | "Child" | "C" => {
                child::main();
                true
            }
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
}
