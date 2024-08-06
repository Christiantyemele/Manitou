use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::AttributeSet;
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

    // loop through listener for incoming connections.
    loop {
        // for each received connection we create a task which consumes it.

        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            // handle file request
            handle_device_request(stream).await.unwrap();
        });
        ()
    }
}
async fn handle_device_request(stream: TcpStream) -> Result<()> {
    // spilt the stream into reader and writer
    let mut stream = stream;
    let (_rd, mut wr) = stream.split();

    // create a virtual device on client
    // let mut device = VirtualDeviceBuilder::new().unwrap()
    // .name("mouse")
    // .with_relative_axes(&AttributeSet::from_iter([
    //     RelativeAxisCode::REL_X
    // ])).unwrap()

    let mut _opened_file = File::open("/dev/input/event16").await?;

    // asynchronoulsy writing to writer in real time
    io::copy(&mut _opened_file, &mut wr).await?;

    Ok(())
}
