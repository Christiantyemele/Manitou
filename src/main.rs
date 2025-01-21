use wayland_client::Connection;

#[tokio::main]
async fn main() {

    // create connection to wayland server
    let conn = Connection::connect_to_env().unwrap();

    // retrieve display to compositor
    let display = conn.display();

    // create event queuefor processing events
    let event_queue = conn.new_event_queue();

    // handle to event queue
    let qh = event_queue.handle();

    // start collecting events
 
}
