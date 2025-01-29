use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{
        wl_compositor, wl_keyboard::{self, WlKeyboard}, wl_output::{self, WlOutput}, wl_pointer::{self, WlPointer}, wl_registry, wl_seat::{self}
    },
    Connection, Dispatch, QueueHandle, WEnum,
};
struct Globals {
    outputs: Vec<WlOutput>,
}

struct State {
    pointer: Option<WlPointer>,
    keyboard: Option<WlKeyboard>,
    g: Globals,
}
#[allow(unused)]
impl Dispatch<wl_pointer::WlPointer, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_pointer::WlPointer,
        event: <wl_pointer::WlPointer as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            // enter event indicating pointer has enterred a surface and its image in undefined at this point
            // therefore should respond to this by setting and appropriate pointer image with `set_cursor` request
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            } => {
                println!("Pointer has entered a surface with serial {}", serial)
            }
            _ => {
                println!("This is not and Enter Event")
            }
        }
    }
}

#[allow(unused)]
impl Dispatch<WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Enter {
                serial,
                surface,
                keys,
            } => {
                println!("Keyboard has enterred a surface ")
            }
            _ => {
                println!("Other events")
            }
        }
    }
}

/// implementation to connect to seats by getting the capability event sent by the server
#[allow(unused)]
impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: <wl_seat::WlSeat as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Pointer) {
                if let Some(p) = state.pointer.take() {
                    // releasing the pointer before making a new request to the compositor
                    p.release();
                }
                // getting a seat to pointer object
                state.pointer.replace(seat.get_pointer(qh, ()));
            }
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                if let Some(k) = state.keyboard.take() {
                    // releasing keyboard
                    k.release(); // check if this is neccessary
                }
                seat.get_keyboard(qh, ());
            }
        }
    }
}

// binds global registry to app state
#[allow(unused)]
impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                if interface.as_str() == "wl_output" {
                    println!("wl_output global");
                    state
                        .g
                        .outputs
                        .push(registry.bind::<WlOutput, _, _>(name, 4, qh, ()));
                }
            }
            wl_registry::Event::GlobalRemove { .. } => {}
            _ => {}
        }
    }
}

#[allow(unused)]
impl Dispatch<WlOutput, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: <WlOutput as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_output::Event::Done = event {
            println!("New window event")
        }
    }
}

// delegate wl_registry events to App itself
#[allow(unused)]
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _state: &mut Self,
        _proxy: &wl_registry::WlRegistry,
        _event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

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
    // let pointer = display.send_request(req)
    let (g, mut queue) = registry_queue_init::<State>(&conn).unwrap();

    let compositor: wl_compositor::WlCompositor = g
    .bind(&qh, 4..=5, ())
    .unwrap();
compositor.create_surface(qh, udata)
}
