use std::borrow::Borrow;
use tokio::fs::File;
use tokio::io::{self, Result};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    // collecting the ip address for the communication as first passed argument
    let mut address = std::env::args().nth(1).expect("expects a valid IP address");

    address.push_str(":6935").borrow();
    // todo give permissions to file and try reading in real time

    let listener = TcpListener::bind(address).await?;

    // loop through listener for incomming connections.
    loop {
        // for each received connection we create a task which consumes it

        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            // handle file request
            handle_device_request(stream).await.unwrap();
        });
        ()
    }
}
async fn handle_device_request(stream: TcpStream) -> Result<()> {
    // spilt the stream into reader and writter
    let mut stream = stream;
    let (_rd, mut wr) = stream.split();

    let mut _opened_file = File::open("/dev/input/event16").await?;

    // asynchronoulsy writting to writter in real time
    io::copy(&mut _opened_file, &mut wr).await?;

    Ok(())
}
