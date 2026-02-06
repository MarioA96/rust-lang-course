use std::{
    io::prelude::*, 
    net::{TcpStream, SocketAddr},
    thread
};

use std::io;
use std::io::{BufReader};


fn handle_server_communication(mut stream: TcpStream) -> io::Result<()> {
    // stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let server_ip = stream.peer_addr().expect("Couldnt resolve peer addr").ip();
    let server_port = stream.peer_addr().expect("Couldnt resolve peer addr").port();
    let client_ip = stream.local_addr().expect("Couldnt resolve local addr").ip();
    let client_port = stream.local_addr().expect("Couldnt resolve local addr").port();
    println!("Connected to server {}:{} from: {}:{}", server_ip, server_port, client_ip, client_port);

    // Then we create a BufReader to read new incoming messages from the server
    let mut reader = BufReader::new(stream.try_clone().expect("Error while attempting to read stream"));

    // We spawn a new thread to handle incoming messages from the server in order to not block the main client thread
    thread::spawn(move || {
        let mut buf = String::new();
        loop {
            buf.clear();
            match reader.read_line(&mut buf) {
                // Server is disconnected
                Ok(0) => {
                    println!("Server disconnected");
                    break;
                },

                Ok(_) => print!("Message from server> {}", buf),

                // We handle the WouldBlock error to continue the loop
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                },

                Err(e) => panic!("encountered IO error: {e}"),
            };
        };
    });

    println!("Type your messages below:\n");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let message = line?;
        let message_with_newline = format!("{}\n", message);
        stream.write(message_with_newline.as_bytes())?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // Define the server addresses to connect to
    let addrs = [
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];

    // Attempt to connect to the server addresses
    let stream = TcpStream::connect(&addrs[..])?;

    match stream.local_addr() {
        Ok(_) => {
            // Once we stablish the connection, we can proceed to handle communication
            handle_server_communication(stream)?;
        },
        Err(e) => {
            eprintln!("Could not get local address: {}", e);
        }
    }
    
    Ok(())
}
