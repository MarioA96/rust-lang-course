// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::collections::HashSet;
use std::io;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::{
    io::prelude::*, 
    net::{SocketAddr, TcpListener, TcpStream},
};
use std::thread;

fn handle_connection(mut stream: TcpStream, clients: Arc<Mutex<HashSet<SocketAddr>>>) {
    // We set the current connection to nonblocking status
    // we want to handle multiple clients without blocking the server
    // and also to listen to messages from clients asynchronously and sending responses
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    // We receive the local address of the incoming connection
    let client_addr = stream.peer_addr().expect("Couldnt resolve for incoming address <ScoketAddr>");
    println!("Incoming connection from: {}:{}", client_addr.ip(), client_addr.port());

    // We add the new client to the shared state
    clients.lock().unwrap().insert(client_addr);
    println!("Current connected clients: {:?}", clients.lock().unwrap());

    // We send a response to the client to confirm the connection
    let welcome_message = format!("Connected! Your address is {}:{}\n", client_addr.ip(), client_addr.port());
    stream.write(welcome_message.as_bytes()).expect("Couldnt send welcome message");


    let mut reader = BufReader::new(stream.try_clone().expect("Error while attempting to read stream"));
    let mut buf = String::new();
    loop {
        buf.clear();
        match reader.read_line(&mut buf) {
            // Client is disconnected
            Ok(0) => {
                println!("Client {}:{} disconnected", client_addr.ip(), client_addr.port());

                clients.lock().unwrap().remove(&client_addr);
                println!("Current connected clients: {:?}", clients.lock().unwrap());

                break;
            },
            Ok(_) => {
                print!("Message from {}:{}> {}", client_addr.ip(), client_addr.port(), buf);

                // Broadcast the message to all connected clients
                let clients_guard = clients.lock().unwrap();

                // Iterate over all clients and send the message
                for &addr in clients_guard.iter() {
                    if addr != client_addr {
                        if let Ok(mut client_stream) = TcpStream::connect(addr) {
                            let message_with_newline = format!("Message from {}:{}> {}", client_addr.ip(), client_addr.port(), buf);
                            client_stream.write(message_with_newline.as_bytes()).expect("Couldnt send message to client");
                        }
                    }
                }
            },
            // We handle the WouldBlock error to continue the loop
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => panic!("encountered IO error: {e}"),
        };
    }

}

fn main() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([192,168,1,72], 8080)),
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];
    
    //After we bind to the address we create a listener in that socket
    let listener = TcpListener::bind(&addrs[..])?;

    //We create a thread-safe shared state to keep track of connected clients
    let clients = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));

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
                let clients_clone = Arc::clone(&clients);

                thread::spawn( move || {
                    handle_connection(stream, clients_clone)
                });
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
    
    Ok(())
}