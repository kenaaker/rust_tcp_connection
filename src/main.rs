use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.3.25:8154")?;
    let a_value: u32 = 4;
    let a_bytes = a_value.to_be_bytes();

    println!("Connected to the light controller.");
    let stream_result = stream.write(&a_bytes);
    println!("Tried to stream a host unsigned integer to the TCP socket, stream_result = {stream_result:?}");
    // stream.write(&[1])?;
    // println!("Tried to write data to the light controller.");

    // println!("Going to try to read from the light controller.");
    stream.read(&mut [0; 128])?;
    // println!("Back from trying to read a response from the light controller.");
    Ok(())
}
