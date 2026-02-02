// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::io;
use std::io::BufReader;
use std::{
    io::prelude::*, 
    net::{SocketAddr, TcpListener, TcpStream},
};
use std::thread;

fn handle_connection(stream: TcpStream){
    let mut s = &stream;
    let local = s.peer_addr().expect("Couldnt resolve for incoming address <ScoketAddr>");
    println!("Incoming local addr: {}:{}", local.ip(), local.port());

    let message = String::from("hOlA dEsDe SeRvEr");
    s.write(message.as_bytes()).expect("Couldnt resolve");

    let mut reader = BufReader::new(stream.try_clone().expect("Error while attempting to read stream"));

    thread::spawn(move || {
        let mut buf = String::new();
        loop {
            buf.clear();
            match reader.read_line(&mut buf) {
                // Server is disconnected
                Ok(0) => {
                    break;
                },

                Ok(_) => println!("{buf:?}"),

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP\\
                    eprintln!("An exception has occured while attempting to resolve nonblocking status");
                    break;
                },

                Err(e) => panic!("encountered IO error: {e}"),
            };
        };
    });
}

fn main() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([192,168,1,72], 8080)),
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];
    
    let listener = TcpListener::bind(&addrs[..])?;

    let addr = listener.local_addr().expect("Couldn't bind");
    let ip = addr.ip().to_string();
    let port = addr.port().to_string();
    println!("Server up at: {}:{}", ip, port);

    for stream in listener.incoming() { //Returns an Iterator and so it acts as an ::connect()
        match stream {
            Ok(stream) => {
                let stream = stream;

                thread::spawn( move || {
                    handle_connection(stream)
                });
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
    
    Ok(())
}