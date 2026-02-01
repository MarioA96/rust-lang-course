use std::{
    io::prelude::*, 
    net::{TcpStream, SocketAddr},
    thread
};

use std::io;
use std::io::{BufReader};

fn main() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];

    let stream = TcpStream::connect(&addrs[..])?;

    // Set the TTL value
    stream.set_ttl(64).expect("Failed to set TTL");

    // Optionally, you can check the TTL value
    let ttl = stream.ttl().expect("Failed to get TTL");
    println!("Current TTL: {}", ttl);

    // We set the current connection to nonblocking status
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let mut reader = BufReader::new(stream.try_clone()?);

    thread::spawn(move || {
        let mut buf = String::new();
        loop {
            buf.clear();
            match reader.read_line(&mut buf) {
                // Server is disconnected
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    read_data(buf);
                    break;
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP\\
                    eprintln!("An exception has occured while attempting to resolve nonblocking status");
                    break;
                }
                Err(e) => panic!("encountered IO error: {e}"),
            };
        };
    });

    // Main thread para enviar mensajes
    // let stdin = io::stdin();

    // for line in stdin.lock().lines() {
    //     let msg = line?;
    //     writeln!(stream, "{}", msg)?;
    // }
    
    Ok(())
}

fn read_data(buf: String) {
    println!("bytes: {buf:?}");
}
