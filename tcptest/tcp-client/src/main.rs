use std::net::{TcpStream, IpAddr, Ipv4Addr, SocketAddr, Shutdown};
use std::io::{Read, Write};
use std::env;
use std::time::Duration;
use std::thread;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

#[derive(Serialize, Deserialize, Debug)]
enum MyMessage {
    Message(String),
    Exit
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let ip_addr: &str;
    if args.len() > 1 {
        ip_addr = &args[1];
    } else {
        ip_addr = "0.0.0.0"
    }

    println!("Attempting connection to {}...", ip_addr);
    match TcpStream::connect(format!("{}:{}", ip_addr, 61357)) {
        Ok(mut stream) => {
            let mut write_stream = stream.try_clone().unwrap();
            println!("Successfully connected to server in port 61357");

            let (tx_write, rx_write) = mpsc::channel();
            let (tx_read, rx_read) = mpsc::channel();

            thread::spawn(move || {
                let mut data = [0 as u8; 2000]; // using 50 byte buffer
                while match stream.read(&mut data) {
                    Ok(size) => {
                        if size != 0 {
                            let msg: MyMessage = bincode::deserialize(&data).unwrap();
                            match msg {
                                MyMessage::Message(msg_str) => {
                                    println!("Msg: {}", msg_str.trim_end());
                                    true
                                }
                                MyMessage::Exit => {
                                    tx_read.send(false).unwrap();
                                    stream.shutdown(Shutdown::Both).unwrap();
                                    false
                                }
                            }
                        }
                        else { true }
                        
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        false
                    }
                } {}
            });

            thread::spawn(move || {
                loop {
                    let msg = input_message();
                    match msg {
                        MyMessage::Exit => {
                            // write_stream.shutdown(Shutdown::Both).unwrap();
                            write_stream.write(&bincode::serialize(&msg).unwrap()).unwrap();
                            tx_write.send(false).unwrap();
                            break;
                        }
                        _ => {
                            write_stream.write(&bincode::serialize(&msg).unwrap()).unwrap();
                        }
                    }
                }
            });

            let exit_thread = thread::spawn(move || {
                while match rx_write.recv_timeout(Duration::from_secs(1)) {
                    Ok(val) => val,
                    _ => true,
                } && match rx_read.recv_timeout(Duration::from_secs(1)) {
                    Ok(val) => val,
                    _ => true,
                } {}
            });
            exit_thread.join().unwrap();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn input_message() -> MyMessage {
    let input = read_input();
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