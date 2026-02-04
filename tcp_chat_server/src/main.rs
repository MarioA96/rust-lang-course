// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::io;
use std::io::BufReader;
use std::{
    io::prelude::*, 
    net::{SocketAddr, TcpListener, TcpStream},
};
use std::thread;

fn handle_connection(mut stream: TcpStream){
    // We set the current connection to nonblocking status
    // we want to handle multiple clients without blocking the server
    // and also to listen to messages from clients asynchronously and sending responses
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    // We receive the local address of the incoming connection
    let addr = stream.peer_addr().expect("Couldnt resolve for incoming address <ScoketAddr>");
    println!("Incoming connection from: {}:{}", addr.ip(), addr.port());

    // We send a response to the client to confirm the connection
    let welcome_message = format!("Connected! Your address is {}:{}\n", addr.ip(), addr.port());
    stream.write(welcome_message.as_bytes()).expect("Couldnt send welcome message");

    // Then we create a BufReader to read new incoming messages from the client
    let mut reader = BufReader::new(stream.try_clone().expect("Error while attempting to read stream"));
    // We spawn a new thread to handle incoming messages from the client in order to not block the main server thread
    thread::spawn(move || {
        let mut buf = String::new();
        loop {
            buf.clear();
            match reader.read_line(&mut buf) {
                // Client is disconnected
                Ok(0) => {
                    println!("Client {}:{} disconnected", addr.ip(), addr.port());
                    break;
                },

                Ok(_) => print!("Message from {}:{}> {}", addr.ip(), addr.port(), buf),

                // We handle the WouldBlock error to continue the loop
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    // Here we just continue the loop
                    continue;
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
    
    //After we bind to the address we create a listener in that socket
    let listener = TcpListener::bind(&addrs[..])?;

    //We just print the data of wich socket are we binding the server
    let addr = listener.local_addr().expect("Couldn't bind");
    let ip = addr.ip().to_string();
    let port = addr.port().to_string();
    println!("Server up at: {}:{}", ip, port);

    //Returns an Iterator and so it acts as an ::connect()
    //For every incoming connection we spawn a new thread to handle it
    for stream in listener.incoming() {
        match stream {
            // If we get a stream we handle the connection in a new thread
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