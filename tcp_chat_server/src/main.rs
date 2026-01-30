// TCP Chat Application: Build a multi-client chat server using TCP sockets. Clients connect and broadcast messages. 
// Learn: Networking with std::net, threads for concurrency, and mutexes for shared state.

use std::{
    io::prelude::*, 
    net::{SocketAddr, TcpListener, TcpStream}
};

fn handle_connection(stream: TcpStream){
    // let mut buf = [0; 10];
    // let len = stream.peek(&mut buf).expect("peek failed");
    // println!("len: {}", len);
    let mut s = &stream;

    println!("Connected to the server!");
    
    let mut buffer = [0; 128];
    let printable = s.read(&mut buffer).expect("Couldnt resolve"); // Read response
    println!("printable: {}", printable);
}

fn main() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];
    
    let listener = TcpListener::bind(&addrs[..])?;

    for stream in listener.incoming() {
        match stream{
            Ok(stream) => {
                handle_connection(stream)
            },
            Err(e) => println!("Couldn't get client: {e:?}"),
        }
    }
    
    Ok(())
}