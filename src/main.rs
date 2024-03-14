use std::net::TcpListener;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Server failed to start");
    for stream in listener.incoming() {
        match stream{
            Ok(_stream) =>{
                println!("Server working");
            }
            Err(_e) => {
                println!("Something went wrong couldnt match");
            }
        }
        println!("TCP listener started listening");
    }
}
