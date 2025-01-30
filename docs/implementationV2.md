
# Resources
- https://wayland-book.com/libwayland

# wayland
## Overview
The wayland protocol works by issuing `request` and `events` that act on `objects`. Each object has an interface which defines
what requests and events are possible, and the signature of each (e.g one event the server can send regarding a wl_surface is `enter`, which it sends when that surface is being displayed on a specific output). Upon POLLIN events call `wl_display_dispatch` to process incomming events. To flush outgoing requests, call `wl_display_flush`. using the `wl_compositor` you may send the server your windows for presentation, a wayland surface has a rectangular area which may be displayed on zero or more ouputs, present buffers, receive user input, and define a local coordinate system. To obtain a surface we first bind to the wl_compositor global, but before presenting the surface we must attach a source of pixel to it using `wl_buffer`, shared memory buffers is the simplest means of getting pixels from client to compositor and the only one enshrined in handling this `wl_shm` it simply allowsyou to transfer a file descriptor for the compositor to mmap(memory map) with MAP_SHARED, then share pixel buffers out of this pool. Add some simple synchronization primitives to keep everone from fighting over each buffer






# Flow
- Connecting to a wayland server and creating a `wl_display` to manage the connection's state
- Create global objects and bind to each interfaces
- Create a surface using compositor for rendering images from a shared memory buffer






We have a wayland client on the sharing computer which collects input events from the wayland compositor and sends it to the wayland compositor of the other computer
through wifi
the events should be sent accross clients on some certain conditions, e.g for a mouse event the event is send to a particular client depending on the predefined position of the pointer on the focus screen

## Steps
- have a wayland client collecting events from the server



# usefull points
- Buffers & surfaces
Apparently, the whole point of this system is to display information to users and receive their feedback for additional processing. In this chapter, we'll explore the first of these tasks: showing pixels on the screen.

There are two primitives which are used for this purpose: buffers and surfaces, governed respectively by the wl_buffer and wl_surface interfaces. Buffers act as an opaque container for some underlying pixel storage, and are supplied by clients with a number of methods — shared memory buffers and GPU handles being the most common.

- The compositor has two jobs: the creation of surfaces and regions.