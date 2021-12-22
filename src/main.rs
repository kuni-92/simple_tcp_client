use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::io::Write;
use std::thread;
use std::time;

fn main() {
    let stream = connect("127.0.0.1:60080");

    for _ in 0..2 {
        send(&stream, "Hello, World!\n");
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn connect<A>(address: A) -> TcpStream
    where A:ToSocketAddrs
{
    match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(e) => panic!("{:?}", e),
    }
}

fn send(mut stream: &TcpStream, text: &str) {
    stream.write(text.as_bytes()).unwrap();
    stream.flush().unwrap();
}
