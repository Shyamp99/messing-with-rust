use std::{io,thread};
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    match TcpStream::connect("127.0.0.1:8888"){
        Ok(mut listener) => {
            // let mut input = String::new();

            println!("Server listening on port 8888");

            // let msg = b"Hello!";

            // listener.write(msg).unwrap();
            // println!("Sent Hello, awaiting reply...");
            loop{
                let mut input = String::new();
                print!("Input you command: ");
                io::Write::flush(&mut io::stdout()).expect("flush failed!");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                println!("sending: {}", input);
                listener.write(input.as_bytes()).unwrap();
            };
            // for stream in listener.incoming() {
            //     let stream = stream.unwrap();
            //     io::stdin().read_line(&mut input).expect("Failed to read line");
            //     println!("{}",input);

            //     stream.write()
            // }
            // drop(listener);
        }
        Err(e) =>{
            println!("Connection failed: {}", e);
        }
    }
}

// fn spawn_connection(mut stream: TcpStream){

// }


