use std::io::{BufRead, BufReader,Write};
use std::net::{TcpListener, TcpStream};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Server failed to start");
    for stream in listener.incoming() {
        match stream{
            Ok(stream) =>{
                println!("Server working");
                handle_connection(stream);
            }
            Err(_e) => {
                println!("Something went wrong couldnt match");
            }
        }
        println!("TCP listener waiting");
    }
}

fn handle_connection(mut stream: TcpStream){
    println!("Handling connection");
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();

    match lines.next() {
        Some(line) => {
            match line {
                Ok(request_line) => {
                    if request_line == "*1\r\n$4\r\nping\r\n" {
                        stream.write(b"+PONG\r\n").expect("Couldn't write into the stream");
                    }
                },
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    return;
                }
            }
        },
        None => {
            eprintln!("No lines to read from the stream");
            return;
        }
    }
}