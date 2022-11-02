# XCB
This document covers some basics of XCB.
This contains strengths and weaknesses and other aspects worth mentioning when trying to decide between
it and Xlib.

All information was pulled from [the XCB tutorial](https://xcb.freedesktop.org/tutorial/).

## What is XCB

It is a alternative to the X-server interface Xlib.
Both offer the ability to communicate with a systems x-server,
which is a crucially important aspect of a window manager.

XCB eliminates the need for programs to implement the X protocol layer by offering low-level access to X-servers.
Since the protocol is standardized, it is possible to talk to any X-server with XCB.

## What does XCB provide over Xlib

- Toolkit implementation
- Direct protocol programming (see section *Why not to use XCB*)
- Leightweight emulation of commonly used portions of the Xlib API
- XCB does not lock itself while waiting for a response it send to a x-server instance like Xlib.
  This avoids needless stalling while a request is processed.
  Instead, XCB binds a cookie to a request which can then be used to ask for a pointer to the corresponding reply.
  This not only enables reading of the reply only when it is required, but also is ~5 times faster then locking while waiting for a reply.


## Latency comparison

- Request: `W`
- Reply: `R`
- No action: `-`
- Amount of send requests: `N`

### Xlib
Due to how Xlib works, a request-reply cycle works like this:
```
W-----RW-----RW-----RW-----R
```
The total time is `N * (T_write + T_round_trip + T_read)`.

### XCB
XCBs request-reply cycle looks like this:
```
WWWW--RRRR
```
The total time is `N * T_write + max(0, T_round_trip - (N-1) * T_write) + N * T_read`.

### Conclusion

XCB offers considerably faster event handling
The tutorial linked at the top of this document also proiveds the source code and results of a benchmarking which lead to the same results.

## Why to not use Xlib

**Xlib is quite big:**
Xlib is bigger then XCB and can therefor not be used with minimalistic systems.
However, the target groups of this project are (semi-)well equiped computers,
which makes this not as much of an advantage.

**Latency:**
Xlib manages events synchronously, with the principle of *first in, first out (fifo)*.
This can cause delays when dealing with a bigger amount of events within a short notice.

**Multithreading:**
XCB appears to support this feature.
Xlib can to some degree work with multiple threads too but its API
was not designed for this purpose which makes it difficult to work with
as well as error-prone.

## Why not to use XCB

**Direct protocol access:**
This can be good or not depending on the system an application will run on.
Xlib performs chaching, layering and other optimizations by itself.
XCB does not provide this feature.

## Summary

XCB seems to be a lighter version of Xlib that also solves issues like multithreading while losing some optimizations
such as caching in the process.
However, XCB offers a considerably quicker request-reply method, which also allows reading of replies when necessary and does not force an aplication to do so when the response becomes
available.
