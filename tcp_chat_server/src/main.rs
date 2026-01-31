// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::{
    io::prelude::*, 
    net::{SocketAddr, TcpListener, TcpStream}
};

fn handle_connection(stream: TcpStream){
    let mut s = &stream;

    let message = String::from("HoLaDeSdEsErVeR");
    s.write(message.as_bytes()).expect("Couldnt resolve");

    // let mut buffer = String::new();
    // let printable = s.read_to_string(&mut buffer).expect("Couldnt resolve"); // Read response
    // println!("printable: {}", printable);
}

fn main() -> std::io::Result<()> {
    let addrs = [
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
                handle_connection(stream)
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
    
    Ok(())
}