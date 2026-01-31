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
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    let mut buf: Vec<u8> = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP\\
            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
    };

    println!("bytes: {buf:?}");

    let ascii_table_upper = vec![
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','R','S','T','U','V','W','X','Y','Z',
    ];
    let ascii_table_lower = vec![
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','r','s','t','u','v','w','x','y','z',
    ];
    for b in buf {
        if b >= 65 && b <= 90{
            let val = b - 65 as u8;
            print!("{}", ascii_table_upper[ val as usize ]);
        } else if b >= 97 && b <= 122 {
            let val = b - 97 as u8;
            print!("{}", ascii_table_lower[ val as usize ]);
        }
    }
    println!("");

    // let mut buffer = String::new();
    // let printable = stream.read_to_string(&mut buffer)?; // Read response

    // println!("printable: {}", printable);


    // println!("Introduce the message: ");

    // let mut message = String::new();
    // io::stdin()
    //     .read_line(&mut message)
    //     .expect("Couldnt read line");

    // stream.write(message.as_bytes())?; // Send data
    

    Ok(())
}