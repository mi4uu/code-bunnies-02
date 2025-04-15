# Crate Documentation

**Version:** 1.0.3

**Format Version:** 43

# Module `mio`

Mio is a fast, low-level I/O library for Rust focusing on non-blocking APIs
and event notification for building high performance I/O apps with as little
overhead as possible over the OS abstractions.

# Usage

Using Mio starts by creating a [`Poll`], which reads events from the OS and
puts them into [`Events`]. You can handle I/O events from the OS with it.

For more detail, see [`Poll`].

[`Poll`]: ../mio/struct.Poll.html
[`Events`]: ../mio/event/struct.Events.html

## Examples

Examples can found in the `examples` directory of the source code, or [on
GitHub].

[on GitHub]: https://github.com/tokio-rs/mio/tree/master/examples

## Guide

A getting started guide is available in the [`guide`] module.

## Available features

The available features are described in the [`features`] module.

## Modules

## Module `event`

Readiness event types and utilities.

```rust
pub mod event { /* ... */ }
```

### Re-exports

#### Re-export `Event`

```rust
pub use self::event::Event;
```

#### Re-export `Events`

```rust
pub use self::events::Events;
```

#### Re-export `Iter`

```rust
pub use self::events::Iter;
```

#### Re-export `Source`

```rust
pub use self::source::Source;
```

## Module `net`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

Networking primitives.

The types provided in this module are non-blocking by default and are
designed to be portable across all supported Mio platforms. As long as the
[portability guidelines] are followed, the behavior should be identical no
matter the target platform.

[portability guidelines]: ../struct.Poll.html#portability

# Notes

When using a datagram based socket, i.e. [`UdpSocket`] or [`UnixDatagram`],
its only possible to receive a packet once. This means that if you provide a
buffer that is too small you won't be able to receive the data anymore. How
OSs deal with this situation is different for each OS:
 * Unixes, such as Linux, FreeBSD and macOS, will simply fill the buffer and
   return the amount of bytes written. This means that if the returned value
   is equal to the size of the buffer it may have only written a part of the
   packet (or the packet has the same size as the buffer).
 * Windows returns an `WSAEMSGSIZE` error.

Mio does not change the value (either ok or error) returned by the OS, it's
up to the user handle this. How to deal with these difference is still up
for debate, specifically in
<https://github.com/rust-lang/rust/issues/55794>. The best advice we can
give is to always call receive with a large enough buffer.

```rust
pub mod net { /* ... */ }
```

### Re-exports

#### Re-export `TcpListener`

```rust
pub use self::tcp::TcpListener;
```

#### Re-export `TcpStream`

```rust
pub use self::tcp::TcpStream;
```

#### Re-export `UdpSocket`

**Attributes:**

- `#[<cfg>(not(target_os = "wasi"))]`

```rust
pub use self::udp::UdpSocket;
```

#### Re-export `UnixDatagram`

**Attributes:**

- `#[<cfg>(unix)]`

```rust
pub use self::uds::UnixDatagram;
```

#### Re-export `UnixListener`

**Attributes:**

- `#[<cfg>(unix)]`

```rust
pub use self::uds::UnixListener;
```

#### Re-export `UnixStream`

**Attributes:**

- `#[<cfg>(unix)]`

```rust
pub use self::uds::UnixStream;
```

## Module `unix`

**Attributes:**

- `#[<cfg>(all(unix, feature = "os-ext"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "os-ext"))))]`

Unix only extensions.

```rust
pub mod unix { /* ... */ }
```

### Modules

## Module `pipe`

Unix pipe.

See the [`new`] function for documentation.

```rust
pub mod pipe { /* ... */ }
```

### Re-exports

#### Re-export `new`

```rust
pub use crate::sys::pipe::new;
```

#### Re-export `Receiver`

```rust
pub use crate::sys::pipe::Receiver;
```

#### Re-export `Sender`

```rust
pub use crate::sys::pipe::Sender;
```

### Re-exports

#### Re-export `SourceFd`

```rust
pub use crate::sys::SourceFd;
```

## Module `features`

**Attributes:**

- `#[<cfg_attr>(feature = "os-poll", doc = "## `os-poll` (enabled)")]`
- `#[<cfg_attr>(not(feature = "os-poll"), doc = "## `os-poll` (disabled)")]`
- `#[<cfg_attr>(feature = "os-ext", doc = "## `os-ext` (enabled)")]`
- `#[<cfg_attr>(not(feature = "os-ext"), doc = "## `os-ext` (disabled)")]`
- `#[<cfg_attr>(feature = "net", doc = "## Network types (enabled)")]`
- `#[<cfg_attr>(not(feature = "net"), doc = "## Network types (disabled)")]`

# Mio's optional features.

This document describes the available features in Mio.

## `os-poll` (enabled)

Mio by default provides only a shell implementation that `panic!`s the
moment it is actually run. To run it requires OS support, this is
enabled by activating the `os-poll` feature.

This makes `Poll`, `Registry` and `Waker` functional.

## `os-ext` (enabled)

`os-ext` enables additional OS specific facilities. These facilities can
be found in the `unix` and `windows` module.

## Network types (enabled)

The `net` feature enables networking primitives in the `net` module.

```rust
pub mod features { /* ... */ }
```

## Module `guide`

**Attributes:**

- `#[<cfg_attr>(feature = "os-poll", doc = "```")]`
- `#[<cfg_attr>(not(feature = "os-poll"), doc = "```ignore")]`
- `#[<cfg_attr>(all(feature = "os-poll", feature = "net"), doc = "```")]`
- `#[<cfg_attr>(not(all(feature = "os-poll", feature = "net")), doc =
"```ignore")]`
- `#[<cfg_attr>(all(feature = "os-poll", feature = "net"), doc = "```")]`
- `#[<cfg_attr>(not(all(feature = "os-poll", feature = "net")), doc =
"```ignore")]`

# Getting started guide.

In this guide we'll do the following:

1. Create a [`Poll`] instance (and learn what it is).
2. Register an [event source].
3. Create an event loop.

At the end you'll have a very small (but quick) TCP server that accepts
connections and then drops (disconnects) them.

## 1. Creating a `Poll` instance

Using Mio starts by creating a [`Poll`] instance, which monitors events
from the OS and puts them into [`Events`]. This allows us to execute I/O
operations based on what operations are ready.

[`Poll`]: ../struct.Poll.html
[`Events`]: ../event/struct.Events.html

```
# use mio::{Poll, Events};
# fn main() -> std::io::Result<()> {
// `Poll` allows for polling of readiness events.
let poll = Poll::new()?;
// `Events` is collection of readiness `Event`s and can be filled by
// calling `Poll::poll`.
let events = Events::with_capacity(128);
# drop((poll, events));
# Ok(())
# }
```

For example if we're using a [`TcpListener`],  we'll only want to
attempt to accept an incoming connection *iff* any connections are
queued and ready to be accepted. We don't want to waste our time if no
connections are ready.

[`TcpListener`]: ../net/struct.TcpListener.html

## 2. Registering event source

After we've created a [`Poll`] instance that monitors events from the OS
for us, we need to provide it with a source of events. This is done by
registering an [event source]. As the name “event source” suggests it is
a source of events which can be polled using a `Poll` instance. On Unix
systems this is usually a file descriptor, or a socket/handle on
Windows.

In the example below we'll use a [`TcpListener`] for which we'll receive
an event (from [`Poll`]) once a connection is ready to be accepted.

[event source]: ../event/trait.Source.html

```
# use mio::net::TcpListener;
# use mio::{Poll, Token, Interest};
# fn main() -> std::io::Result<()> {
# let poll = Poll::new()?;
# let address = "127.0.0.1:0".parse().unwrap();
// Create a `TcpListener`, binding it to `address`.
let mut listener = TcpListener::bind(address)?;

// Next we register it with `Poll` to receive events for it. The `SERVER`
// `Token` is used to determine that we received an event for the listener
// later on.
const SERVER: Token = Token(0);
poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;
# Ok(())
# }
```

Multiple event sources can be [registered] (concurrently), so we can
monitor multiple sources at a time.

[registered]: ../struct.Registry.html#method.register

## 3. Creating the event loop

After we've created a [`Poll`] instance and registered one or more
[event sources] with it, we can [poll] it for events. Polling for events
is simple, we need a container to store the events: [`Events`] and need
to do something based on the polled events (this part is up to you, we
can't do it all!). If we do this in a loop we've got ourselves an event
loop.

The example below shows the event loop in action, completing our small
TCP server.

[poll]: ../struct.Poll.html#method.poll
[event sources]: ../event/trait.Source.html

```
# use std::io;
# use std::time::Duration;
# use mio::net::TcpListener;
# use mio::{Poll, Token, Interest, Events};
# fn main() -> io::Result<()> {
# let mut poll = Poll::new()?;
# let mut events = Events::with_capacity(128);
# let address = "127.0.0.1:0".parse().unwrap();
# let mut listener = TcpListener::bind(address)?;
# const SERVER: Token = Token(0);
# poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;
// Start our event loop.
loop {
    // Poll the OS for events, waiting at most 100 milliseconds.
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;

    // Process each event.
    for event in events.iter() {
        // We can use the token we previously provided to `register` to
        // determine for which type the event is.
        match event.token() {
            SERVER => loop {
                // One or more connections are ready, so we'll attempt to
                // accept them (in a loop).
                match listener.accept() {
                    Ok((connection, address)) => {
                        println!("Got a connection from: {}", address);
#                       drop(connection);
                    },
                    // A "would block error" is returned if the operation
                    // is not ready, so we'll stop trying to accept
                    // connections.
                    Err(ref err) if would_block(err) => break,
                    Err(err) => return Err(err),
                }
            }
#           _ => unreachable!(),
        }
    }
#   return Ok(());
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}
# }
```

```rust
pub mod guide { /* ... */ }
```

## Re-exports

### Re-export `Events`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use event::Events;
```

### Re-export `Interest`

```rust
pub use interest::Interest;
```

### Re-export `Poll`

```rust
pub use poll::Poll;
```

### Re-export `Registry`

```rust
pub use poll::Registry;
```

### Re-export `Token`

```rust
pub use token::Token;
```

### Re-export `Waker`

**Attributes:**

- `#[<cfg>(not(target_os = "wasi"))]`

```rust
pub use waker::Waker;
```

