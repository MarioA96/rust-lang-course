use std::{
    io::prelude::*, 
    net::{TcpStream, SocketAddr}
};

use std::io;

fn main() -> std::io::Result<()> {
    let addrs = [
        SocketAddr::from(([127,0,0,1], 80)),
        SocketAddr::from(([127,0,0,1], 4343)),
    ];

    let mut stream = TcpStream::connect(&addrs[..])?;

    // Set the TTL value
    stream.set_ttl(64).expect("Failed to set TTL");

    // Optionally, you can check the TTL value
    let ttl = stream.ttl().expect("Failed to get TTL");
    println!("Current TTL: {}", ttl);

    // We set the current connection to nonblocking status
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let mut buf: Vec<u8> = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
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
    
    Ok(())
}

fn read_data(buf: Vec<u8>){
    println!("bytes: {buf:?}");

    let ascii_table_upper = vec![
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    ];
    let ascii_table_lower = vec![
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    ];
    for b in buf {
        if b == 32{
            print!(" ")
        }else if b >= 65 && b <= 90{
            let val = b - 65 as u8;
            print!("{}", ascii_table_upper[ val as usize ]);
        } else if b >= 97 && b <= 122 {
            let val = b - 97 as u8;
            print!("{}", ascii_table_lower[ val as usize ]);
        }
    }
    println!("");
}