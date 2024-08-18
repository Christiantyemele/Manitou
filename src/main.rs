use manitou::handle_device_request;
use std::borrow::Borrow;
use tokio::io::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // collecting the ip address for the communication as first passed argument
    let mut address = std::env::args().nth(1).expect("expects a valid IP address");
    let mode = std::env::args()
        .nth(2)
        .expect("second argument should specify if you are server or client e.g server or client");
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
