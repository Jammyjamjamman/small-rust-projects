use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Debug)]
enum MyMessage {
    Message(String),
    Exit
}

fn handle_client(mut stream: TcpStream, stdin_rx: std::sync::mpsc::Receiver<String>, tcp_open: Arc<Mutex<bool>>) {
    let mut write_stream = stream.try_clone().unwrap();

    let tcp_open_read = tcp_open.clone();
    let read_thread = thread::spawn(move || {
        let mut data = [0 as u8; 2000]; // using 2000 byte buffer
        while { 
            let val = tcp_open_read.lock().unwrap();
            *val 
        } 
        && 
        match bincode::deserialize_from(stream.try_clone().unwrap()) {
            Ok(MyMessage::Message(msg_str)) => {
                    println!("Msg: {}", msg_str.trim_end());
                    true
            },
            Ok(MyMessage::Exit) => {
                println!("Chat closed.");
                false
            },

            Err(e) => {
                {
                    let end = tcp_open_read.lock().unwrap();
                    if !*end {
                        println!("An error occurred: {}", e);
                    }
                }
                println!("Terminating connection with {}.", stream.peer_addr().unwrap());
                false
            }
        } {}
        {
            let mut end = tcp_open_read.lock().unwrap();
            *end = false;
        }
        match stream.shutdown(Shutdown::Both) {
            Err(e) => println!("error when shutting down connection: {}", e),
            _ => ()
        }
    });

    let write_thread = thread::spawn(move || {
        while { 
            let val = tcp_open.lock().unwrap();
            *val 
        } 
        && {
            match stdin_rx.recv() {
                Ok(msg_str) => {
                    let msg = input_message(msg_str);
                    match msg {
                        MyMessage::Exit => {
                            match write_stream.write(&bincode::serialize(&msg).unwrap()) {
                                _ => false
                            }
                        }
                        _ => {
                            match write_stream.write(&bincode::serialize(&msg).unwrap()) {
                                Ok(_) => true,
                                _ => false
                            }
                        }
                    }
                },
                _ => false
            }
        } {}
        println!("Connection input closed.");
        {
            let mut end = tcp_open.lock().unwrap();
            *end = false;
        }
    });
    read_thread.join().unwrap();
    write_thread.join().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:61357").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 61357");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tcp_open = Arc::new(Mutex::new(true));
                let (tx_stdin, rx_stdin) = mpsc::channel();
                let tcp_open_stdin = tcp_open.clone();
                
                thread::spawn(move || {
                    while { 
                        let val = tcp_open_stdin.lock().unwrap();
                        *val } 
                        && {
                        let cli_input = read_input();
                        if cli_input.trim_end() == "/exit" {
                            tx_stdin.send(cli_input).unwrap();
                            false
                        }
                        else {
                            tx_stdin.send(cli_input).unwrap();
                            true
                        }
                    } {}
                });
                
                let tcp_net = tcp_open.clone();
                println!("New connection: {}", stream.peer_addr().unwrap());
                let conn_thread = thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream, rx_stdin, tcp_net);
                });
                conn_thread.join().unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

fn input_message(input: String) -> MyMessage {
    if input.trim_end() == "/exit" {
        MyMessage::Exit
    }
    else {
        MyMessage::Message(input)
    }
}

fn read_input() -> String {
    use std::io;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input
        }
        Err(_) => "error".to_owned(),
    }
}