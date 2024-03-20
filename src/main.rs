use std::io::{BufRead, BufReader,Write,Read};
use std::net::{TcpListener, TcpStream};

use bytes::buf;
fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Server failed to start");
    for stream in listener.incoming() {
        match stream{
            Ok(mut stream) =>{
                println!("Server working");
                let mut buf = [0; 256];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        let request = std::str::from_utf8(&buf).expect("Couldn't convert to String");
                        //if request.contains("*1\r\n$4\r\nping\r\n") {
                            let data = b"+PONG\r\n"; 
                            stream.write(data).expect("Couldn't write into the stream");
                        //}
                    },
                    Err(_e) => {
                        eprintln!("Couldn't read from the stream: {}", _e);
                    }
                }
                //loop {
                    //handle_connection(&mut stream);
                    // let data = b"+PONG\r\n"; 
                    // stream.write(data).expect("Couldn't write into the stream");
                    //let buf_reader = BufReader::new(&mut stream);
                    //let mut buf_reader = [0;256];
                    //let n = stream.read(&mut buf_reader).expect("Couldn't read from the stream");
                    //let buf_reader = &buf_reader[..n];
                    //let buf_reader = std::str::from_utf8(&buf_reader).expect("Couldnt convert to Stringx");
                    //println!("Read from stream {:?}", &buf_reader[..n]);

                //}
            }
            Err(_e) => {
                println!("Something went wrong couldnt match");
            }
        }
        println!("TCP listener waiting");
    }
}

// fn handle_connection(stream:  &mut TcpStream){
//     println!("Handling connection");
//     let buf_reader = BufReader::new(stream);
//     let mut lines = buf_reader.lines();
//     let data = b"+PONG\r\n"; 
//     let mut stream = &stream;
//     stream.write(data).expect("Couldn't write into the stream");
// }