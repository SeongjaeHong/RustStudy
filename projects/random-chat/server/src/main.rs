use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use message::{Message, MSG_SIZE};

const LOCAL: &str = "127.0.0.1:6000";

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}
fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<Message>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connectd", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone a client"));

            thread::spawn(move || loop {
                let mut buff = vec![0];
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let mut buff = vec![0; usize::from(buff[0])];
                        let _ = socket.read_exact(&mut buff);
                        let msg = buff
                            .into_iter()
                            .take_while(|&x| x != 60)
                            .collect::<Vec<_>>();
                        let msg_socket: Message = bincode::deserialize(&msg).unwrap();

                        println!("{}: {}", msg_socket.addr, msg_socket.msg);
                        tx.send(msg_socket).expect("Failed to send a msg to rx");
                    }
                    Err(err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {} ", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg_socket) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let msg_socket = bincode::serialize(&msg_socket).unwrap();
                    client.write_all(&msg_socket).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}
