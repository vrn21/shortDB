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

fn handle_connection(stream:  TcpStream){
    println!("Handling connection");
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();
    let data = b"+PONG\r\n"; 
    let mut stream = &stream;
    stream.write(data).expect("Couldn't write into the stream");
}