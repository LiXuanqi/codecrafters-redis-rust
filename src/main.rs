// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        thread::spawn(move || {
            match stream {
                Ok(mut _stream) => {
                    loop {
                        let mut buffer = [0; 512];
                        _stream.read(&mut buffer).unwrap();

                        // let result = String::from_utf8_lossy(&buffer[..]);
                        // println!("{}", result);
                        
                        _stream.write("+PONG\r\n".as_bytes()).unwrap();
                        _stream.flush().unwrap();
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}
