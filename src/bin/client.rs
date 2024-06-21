use std::borrow::Borrow;
use tokio::io;
use tokio::{fs::File, net::TcpStream};
use tokio::io::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let mut address = std::env::args()
        .nth(1)
        .expect("expects a valid IP address as first argument");
    address.push_str(":6935").borrow();
    // connecting to the listening socket
    let mut socket = TcpStream::connect(address).await?;
    let (mut rd, mut _wr) = socket.split();
    let mut client_device_file = File::open("/dev/input/event4").await?;

    io::copy(&mut rd, &mut client_device_file).await?;
    Ok(())
}
