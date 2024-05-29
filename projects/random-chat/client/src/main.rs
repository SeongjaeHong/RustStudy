use message::{Message, MSG_SIZE};
use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect.");
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking.");

    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg_socket: Message = bincode::deserialize(&buff[..]).unwrap();
                println!("[{}]: {}", msg_socket.addr, msg_socket.msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with the server is closed.");
                break;
            }
        }

        match rx.try_recv() {
            Ok(buff) => {
                let mut msg_socket: Vec<u8> = bincode::serialize(&buff).unwrap();
                let mut len_socket: Vec<u8> = vec![msg_socket.len() as u8];
                len_socket.append(&mut msg_socket);
                client
                    .write_all(&len_socket)
                    // .write_all(&msg_socket)
                    .expect("Writing to the socket failed.");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    loop {
        println!("Write a Message:");
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Reading from stdin failed.");

        let msg_socket = Message {
            addr: LOCAL.parse().unwrap(),
            msg: buff.trim().to_string(),
        };

        if msg_socket.msg == ":q" {
            break;
        }

        if tx.send(msg_socket).is_err() {
            break;
        }
    }
    println!("Bye bye {}", LOCAL);
}
