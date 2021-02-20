use std::io::{stdin, Read, Write};
use std::net::{TcpListener, TcpStream};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use std::thread::JoinHandle;

use crate::common::{deserialize, serialize, Dummy, Executable, Msg, Type};

const LOCALHOST: &'static str = "127.0.0.1";

const MAX_MESSAGE_SIZE: usize = 4 * 1024;

#[allow(dead_code)]
pub struct Client {
    stream: Arc<TcpStream>,
    listening_thread_join_handle: JoinHandle<()>,
    listening: Arc<AtomicBool>,
    active: Arc<AtomicBool>,
}

#[allow(dead_code)]
pub struct Server {
    remote_exec_msg: Arc<Vec<u8>>,
    // listener: Arc<Mutex<TcpListener>>,
    clients: Arc<Mutex<Vec<Client>>>,
    aggregation_buffer: Arc<Mutex<Vec<String>>>,
    // accepting_thread_handle: Option<JoinHandle<()>>,
    accepting: Arc<AtomicBool>,
    aggregation_func: Box<dyn Executable<Vec<String>, String>>,
}

impl Server {
    fn open_tcp_listener(mut port: usize) -> (TcpListener, usize) {
        let host_addr = LOCALHOST;
        loop {
            let addr = format!("{}:{}", host_addr, port);
            let listener = TcpListener::bind(addr);
            match listener {
                Ok(tcp_listener) => {
                    return (tcp_listener, port);
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
    }

    fn open_accepting_thread(
        listener: &Arc<Mutex<TcpListener>>,
        accepting: &Arc<AtomicBool>,
        clients: &Arc<Mutex<Vec<Client>>>,
        remote_exec_msg: &Arc<Vec<u8>>,
        aggregation_buffer: &Arc<Mutex<Vec<String>>>,
    ) -> JoinHandle<()> {
        std::thread::spawn({
            let listener = listener.clone();
            let accepting = accepting.clone();
            let clients = clients.clone();
            let aggregation_buffer = aggregation_buffer.clone();
            let remote_exec_msg = remote_exec_msg.clone();
            move || {
                let listener = listener.lock().expect("Could not lock the tcp listener!");
                for stream in listener.incoming() {
                    if !accepting.load(Ordering::Relaxed) {
                        println!("Stopping accepting clients");
                        break;
                    }
                    if let Ok(mut socket) = stream {
                        println!("Client connected: {}", socket.peer_addr().unwrap());
                        socket
                            .write(&remote_exec_msg)
                            .expect("Could not write to the client");

                        socket.flush().expect("Could not flush to the client");

                        println!(
                            "Sent the client at {} their task",
                            socket
                                .peer_addr()
                                .expect("Unable to take the address of the client")
                        );

                        let listening = Arc::new(AtomicBool::new(true));
                        let active = Arc::new(AtomicBool::new(true));
                        let stream = Arc::new(socket.try_clone().expect("Could not clone socket"));

                        let clone = listening.clone();
                        let stream_clone = stream.clone();
                        let active_clone = active.clone();
                        let aggregation_buffer = aggregation_buffer.clone();
                        let listening_thread_join_handle = std::thread::spawn(move || {
                            while clone.load(Ordering::Relaxed) {
                                let mut buf = [0; MAX_MESSAGE_SIZE];
                                let peek_res = stream_clone.peek(&mut buf); //.expect("Cannot peek in the buffer");
                                match peek_res {
                                    Err(_) => {
                                        println!("Error while peeking in buffer of client at {}, disconnecting them.", stream_clone.peer_addr().unwrap());
                                        active_clone.store(false, Ordering::Relaxed);
                                        break;
                                    }
                                    _ => {}
                                };
                                let mut msg = buf.to_vec();
                                msg.retain(|&el| el != 0);
                                let msg =
                                    std::str::from_utf8(&msg).expect("Could not parse uft8 string");
                                // println!("Message is {}", msg);
                                let msg = deserialize(msg.into());
                                println!(
                                    "Received a message from the client at {}",
                                    stream_clone.peer_addr().unwrap()
                                );
                                // Insert in the aggregation queue
                                let mut agg_buf = aggregation_buffer
                                    .lock()
                                    .expect("Could not lock the aggregation buffer");
                                agg_buf.push(msg.res.clone());

                                println!("{} : {}", stream_clone.peer_addr().unwrap(), msg.res);
                                let _ = socket.read(&mut buf);
                                println!("Thanking the client...");
                                let thank_you_msg = Msg {
                                    type_msg: Type::ThankYou,
                                    res: "".to_string(),
                                    func: Box::new(Dummy),
                                };
                                socket
                                    .write(serialize(thank_you_msg).as_bytes())
                                    .expect("Could not write thank you to client!");
                                println!("Thank you sent...");
                            }
                        });
                        println!("Spawned listening thread.");
                        let new_client = Client {
                            stream,
                            listening_thread_join_handle,
                            listening,
                            active,
                        };
                        let mut client_handles = clients.lock().unwrap();
                        client_handles.push(new_client);
                    }
                }
            }
        })
    }

    pub fn new<T, U>(port: usize, remote_exec_func: Box<T>, aggregation_func: Box<U>) -> Self
    where
        T: Executable<(), String> + 'static,
        U: Executable<Vec<String>, String> + 'static,
    {
        let bytes: Vec<u8> = serialize(Msg {
            type_msg: Type::Task,
            res: "".into(),
            func: remote_exec_func,
        })
        .as_bytes()
        .iter()
        .map(|&el| el)
        .collect();

        let remote_exec_msg = Arc::new(bytes);
        let (listener, port) = Server::open_tcp_listener(port);
        let listener = Arc::new(Mutex::new(listener));
        let clients = Arc::new(Mutex::new(Vec::new()));
        let aggregation_buffer = Arc::new(Mutex::new(Vec::new()));
        let accepting = Arc::new(AtomicBool::new(true));
        /* let accepting_thread_handle =*/
        Some(Server::open_accepting_thread(
            &listener,
            &accepting,
            &clients,
            &remote_exec_msg,
            &aggregation_buffer,
        ));

        println!("Copper server node started on port {}", port);

        Server {
            remote_exec_msg,
            // listener,
            clients,
            aggregation_buffer,
            // accepting_thread_handle,
            accepting,
            aggregation_func,
        }
    }
    pub fn shutdown(&mut self) {
        self.accepting.store(false, Ordering::Relaxed);
        // match &self.accepting_thread_handle {
        //     Some(handle) => {
        //         handle.join();
        //     }
        //     None => {}
        // };
        // self.accepting_thread_handle.join();
    }

    pub fn list_clients(&self) {
        let mut at_least_one = false;
        for client in self.clients.lock().unwrap().iter() {
            if client.active.load(Ordering::Relaxed) {
                println!("Client at {}", client.stream.peer_addr().unwrap());
                at_least_one = true;
            }
        }
        if !at_least_one {
            println!("No active clients connectd");
        }
    }

    const HELP_LINES: &'static [&'static str] = &[
        "Help manual for Copper server",
        "Any letter encased in square brackets -> [] is accepted standalone, upper- and lowercase and will trigger the respective command",
        "[H]elp   - list available commands",
        "[L]ist   - lists all clients and their IP-s",
        "[S]tart  - starts the aggregation",
        "[B]uffer - prints the buffer",
        "[E]xit   - shuts down the server",
    ];

    pub fn print_help(&self) {
        for line in Server::HELP_LINES {
            println!("{}", line);
        }
    }

    pub fn start_aggregation(&self) {
        println!("Starting aggregation");
        let aggreagation_buffer = self
            .aggregation_buffer
            .lock()
            .expect("Could not lock aggregation buffer");
        let res = self.aggregation_func.exec((*aggreagation_buffer).clone());
        println!("After aggregation got: {}", res);
    }

    pub fn print_buffer(&self) {
        let mut at_least_one = false;
        for line in self
            .aggregation_buffer
            .lock()
            .expect("Could not lock aggregation buffer")
            .iter()
        {
            println!("{}", line);
            at_least_one = true;
        }
        if !at_least_one {
            println!("Nothing in the buffer");
        }
    }

    pub fn repl(&mut self) -> bool {
        println!("");
        self.print_help();
        let mut choice = String::new();
        choice.clear();
        let _ = stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice from stdin!");
        match choice.trim() {
            "h" | "help" | "H" | "Help" => {
                self.print_help();
            }
            "l" | "list" | "L" | "List" => {
                self.list_clients();
            }
            "s" | "start" | "S" | "Start" => {
                self.start_aggregation();
            }
            "b" | "buffer" | "B" | "Buffer" => {
                self.print_buffer();
            }
            "e" | "exit" | "E" | "Exit" => {
                self.shutdown();
            }
            _ => println!("Unrecognised command, try \"help\" for more information"),
        }
        self.accepting.load(Ordering::Relaxed)
    }
}

fn next_port(port: usize) -> usize {
    port + 1
}
