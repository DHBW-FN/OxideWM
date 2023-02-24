# IPC

During a first discussion it was decided that *Oxide* should be controllable via an ipc mechanism.
This functionality will be inspired by i3-msg.

## Description
An IPC mechanism for the window manager is required.
This is neccessary for:

* Taskbar
* External libraries
* Command line utility

## Feature list
The following list includes currently proposed features

* everything that is achievable via the keyboard (kill, move, launch...)
* current state of the window manager including e.g. layout or windows


## IPC integration solution

Since there are two types of events that have to be handled, there
needs to be some separation between them.

One type are `xevents`, received from the `X11` instance, and the other one
is custom events created by the user, received over `zbus`.

For this reason, each type of event will get its own loop on its own thread,
which will await them and push them into a list shared between them.
The events in this list will be taken care of by the window manager,
who will execute the correct action based on event type and content.

![IPC-QUEUE](ipc-queue.png)

## Technical solution
The following sections describe the argument for the different ipc-mechanisms and libraries.

### Requirements
As for the aforementioned use cases it will not be required to send large amounts of data. 
Only short messages will be exchanged between the clients. Also it is not expected that the ipc performance will have a significant impact on the usability of the system.
Therefore some ipc options such as shared memory and semaphores will not be regarded as these options are not as easy to use and do not offer any significant advantages.

### Possible IPC mechanisms
There are multiple different ways of implementing ipc on posix systems.

#### FIFO
[Named Pipes Wikipedia](https://en.wikipedia.org/wiki/Named_pipe)
* Work like normal pipes, but are a permanent file on the system
* Fasted regarded option
* Good library support

#### Unix Sockets
[Unix Domain Sockets Wikipedia](https://de.wikipedia.org/wiki/Unix_Domain_Socket)
* Work like tcp sockets
* Very fast ipc mechanism
* Easy to use and inbuilt library support

#### D-Bus
[D-Bus Wikipedia](https://en.wikipedia.org/wiki/D-Bus)
[D-Bus documentation Rust](https://docs.rs/dbus/latest/dbus/)
[D-Bus create from freedesktop.org](https://dbus.pages.freedesktop.org/zbus/)
[D-Bus interface for Rust](https://github.com/diwic/dbus-rs)
* High level ipc mechanism
* Based on unix sockets
* Widely used in projects such as Gnome and KDE
* Offers message queing, tow way communication and is supposed to offer a easy to use interface
* Comparetively slow compared to FIFO or UNIX sockets

### Key Takeaways

[Discussion about ipc on Stackoverflow](https://stackoverflow.com/questions/1235958/ipc-performance-named-pipe-vs-socket)
[Stackoverflow Comparison D-Bus vs Unix Sockets](https://stackoverflow.com/questions/33887063/difference-between-dbus-and-other-interprocess-communications-method)
[Practical uses of D-Bus](https://unix.stackexchange.com/questions/604258/what-is-d-bus-practically-useful-for)

* TCP Sockets are only about 16% slower compared to FIFO
* IPC performance is in most cases not the bottleneck
* Sockets allow for two way communication
* Sockets are more widely supported
* IPC interface should be abstracted, so that the ipc mechanism can be changed in a later stage
* D-Bus should offer a high level, easy to use ipc mechanism


### Conclusion

After a technical discussion with our team, we came to the conclusion that **D-Bus is most suitable**.
The **performance is deemed non critical** in our use case and the ease of use will be benefitial for the project.
None the less, the ipc interface should be **created in an abstract manner** allowing for a possible replacement of the underlying ipc mechanism.  

### Implementation
#### Available libraries
There seem to be two main projects striving to provide dbus support for rust.

[Zbus project repository](https://gitlab.freedesktop.org/dbus/zbus/-/tree/main)
[Zbus crate](https://crates.io/crates/zbus)
[Zbus documentation](https://dbus.pages.freedesktop.org/zbus/)
* Official dbus rust implementation by the freedesktop.org foundation
* Pure rust implementation
* Extensive documentation
* Examples

[dbus-rs repository](https://github.com/diwic/dbus-rs)
[dbus crate](https://crates.io/crates/dbus)
* Wrapper library for libdbus -> libdbus dependency
* Examples

#### Conclusion
Zbus seems to have some advantages over dbus-rs, mainly:
* official freedesktop.org library
* pure rust -> no ibdbus dependency
* Extensive documentation
* Due to being an official library, maintenance is most likely certain

Therefore we came to the conclusion **to use zbus** as our ipc library.

## Conclusion
As IPC-mechanism **dbus** was chosen as most suitable.
The rust library **zbus** has been chosen as implementation.


