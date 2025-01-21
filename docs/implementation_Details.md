# LINUX
the initial target of manitou is for linux computers to be able to share peripheral devices among themselves (mouse, keyboard and screen) just as `lan mouse`, `any desk` or `team viewer`

# Prerequistics
- Linux filesystem and mostly /dev directory
- Linux event system
- Networking of wireless devices
- https://wayland.freedesktop.org/libinput/doc/latest/what-is-libinput.html#what-is-libinput

# Resources
- https://docs.kernel.org/input/index.html
- https://www.kernel.org/doc/html/v4.12/input/index.html
- https://www.infradead.org/~mchehab/rst_features/input/input.html

- https://www.baeldung.com/linux/dev-directory
- https://tldp.org/LDP/sag/html/dev-fs.html
# Steps To Initiate Communication Between Devices

- should be connected to the same LAN or WAN

# Implementation Logic Behind Manitou
From Linux event system when a devices in connected to a computer posted hosting the linux system the devices is read and recorded in /dev or /dev/input for input devices, this is where all the input event are buffered as binaries Manitou then collects this event and processes them at real-time before the buffer is cleared, the collected are then send over TCP connection to the other device, 
Device two has to process this events such that computer two behaves as if it was the on which generated the events, to do so;

device 2 creates a virtual device of the peripheral in questions and passes the events to that virt devices which then renders the events as if they directly came from the device.

# Points To Consider And Some Questions
##  Questions
1. how does device2 know what peripheral event is send given it is expecting from mouse, keyboard and screen 
2. 

## Points To Consider
1. The process of collecting and transferring the events is carried in an async fashion since we need a multi-threaded situation for multitasking

# Flow
Collect event ---> send event ---> process event ---> dispatch event


# wayland
We have a wayland client on the sharing computer which collects input events from the wayland compositor and sends it to the wayland compositor of the other computer
through wifi
the events should be sent accross clients on some certain conditions, e.g for a mouse event the event is send to a particular client depending on the predefined position of the pointer on the focus screen

## Steps
- have a wayland client collecting events from the server