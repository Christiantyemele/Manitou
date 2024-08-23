use std::sync::mpsc;

use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, EventType, InputEvent, RelativeAxisType};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::fs::File;
use tokio::io::{self, Result};
use tokio::net::TcpStream;
use tokio::spawn;

pub async fn handle_device_request(stream: TcpStream) -> Result<()> {
    // spilt the stream into reader and writer
    let mut stream = stream;
    let (_rd, mut wr) = stream.split();

    // sample
    let _inputevent = InputEvent::new(EventType::UINPUT, 0xff67, 0);

    let (tx, rx) = mpsc::channel();

    // Create virtual device on client
    let mut device = VirtualDeviceBuilder::new()
        .unwrap()
        .name("mouse")
        .with_relative_axes(&AttributeSet::from_iter([
            RelativeAxisType::REL_X,
            RelativeAxisType::REL_Y,
            RelativeAxisType::REL_WHEEL,
            RelativeAxisType::REL_HWHEEL,
        ]))
        .unwrap()
        .build()
        .unwrap();

    // move to syspath of device file in /dev/input/?
    spawn(async move {
        loop {
            for path in device.enumerate_dev_nodes_blocking().unwrap() {
                let path = path.unwrap();

                let mut watcher = RecommendedWatcher::new(tx.clone(), Config::default()).unwrap();
                watcher.watch(&path, RecursiveMode::Recursive).unwrap();
            }
        }
    });
    // creating a task which will continuously check for new events in a non-blocking fashion
    spawn(async move {
        for ev in rx {
            match ev {
                // collect binary events from server, convert them to `hexadecimal code`, `bringout the i32 value`, `bring out the u16 type`
                Ok(_) => {
                    
                }
                Err(e) => {
                    panic!("Error Reading Device File {e}")
                }
            }
        }
    });


    // call th new method with the three inputs then emit them to the virtual device created on the client
    let mut _opened_file = File::open("/dev/input/event16").await?;

    // asynchronoulsy writing to writer in real time
    io::copy(&mut _opened_file, &mut wr).await?;

    Ok(())
}
