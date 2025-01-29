
# Resources
- https://wayland-book.com/libwayland

# wayland
The wayland protocol works by issuing `request` and `events` that act on `objects`. Each object has an interface which defines
what requests and events are possible, and the signature of each (e.g one event the server can send regarding a wl_surface is `enter`, which it sends when that surface is being displayed on a specific output)













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