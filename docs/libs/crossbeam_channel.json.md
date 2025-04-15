# Crate Documentation

**Version:** 0.5.15

**Format Version:** 43

# Module `crossbeam_channel`

Multi-producer multi-consumer channels for message passing.

This crate is an alternative to [`std::sync::mpsc`] with more features and better performance.

# Hello, world!

```
use crossbeam_channel::unbounded;

// Create a channel of unbounded capacity.
let (s, r) = unbounded();

// Send a message into the channel.
s.send("Hello, world!").unwrap();

// Receive the message from the channel.
assert_eq!(r.recv(), Ok("Hello, world!"));
```

# Channel types

Channels can be created using two functions:

* [`bounded`] creates a channel of bounded capacity, i.e. there is a limit to how many messages
  it can hold at a time.

* [`unbounded`] creates a channel of unbounded capacity, i.e. it can hold any number of
  messages at a time.

Both functions return a [`Sender`] and a [`Receiver`], which represent the two opposite sides
of a channel.

Creating a bounded channel:

```
use crossbeam_channel::bounded;

// Create a channel that can hold at most 5 messages at a time.
let (s, r) = bounded(5);

// Can send only 5 messages without blocking.
for i in 0..5 {
    s.send(i).unwrap();
}

// Another call to `send` would block because the channel is full.
// s.send(5).unwrap();
```

Creating an unbounded channel:

```
use crossbeam_channel::unbounded;

// Create an unbounded channel.
let (s, r) = unbounded();

// Can send any number of messages into the channel without blocking.
for i in 0..1000 {
    s.send(i).unwrap();
}
```

A special case is zero-capacity channel, which cannot hold any messages. Instead, send and
receive operations must appear at the same time in order to pair up and pass the message over:

```
use std::thread;
use crossbeam_channel::bounded;

// Create a zero-capacity channel.
let (s, r) = bounded(0);

// Sending blocks until a receive operation appears on the other side.
thread::spawn(move || s.send("Hi!").unwrap());

// Receiving blocks until a send operation appears on the other side.
assert_eq!(r.recv(), Ok("Hi!"));
```

# Sharing channels

Senders and receivers can be cloned and sent to other threads:

```
use std::thread;
use crossbeam_channel::bounded;

let (s1, r1) = bounded(0);
let (s2, r2) = (s1.clone(), r1.clone());

// Spawn a thread that receives a message and then sends one.
thread::spawn(move || {
    r2.recv().unwrap();
    s2.send(2).unwrap();
});

// Send a message and then receive one.
s1.send(1).unwrap();
r1.recv().unwrap();
```

Note that cloning only creates a new handle to the same sending or receiving side. It does not
create a separate stream of messages in any way:

```
use crossbeam_channel::unbounded;

let (s1, r1) = unbounded();
let (s2, r2) = (s1.clone(), r1.clone());
let (s3, r3) = (s2.clone(), r2.clone());

s1.send(10).unwrap();
s2.send(20).unwrap();
s3.send(30).unwrap();

assert_eq!(r3.recv(), Ok(10));
assert_eq!(r1.recv(), Ok(20));
assert_eq!(r2.recv(), Ok(30));
```

It's also possible to share senders and receivers by reference:

```
use crossbeam_channel::bounded;
use crossbeam_utils::thread::scope;

let (s, r) = bounded(0);

scope(|scope| {
    // Spawn a thread that receives a message and then sends one.
    scope.spawn(|_| {
        r.recv().unwrap();
        s.send(2).unwrap();
    });

    // Send a message and then receive one.
    s.send(1).unwrap();
    r.recv().unwrap();
}).unwrap();
```

# Disconnection

When all senders or all receivers associated with a channel get dropped, the channel becomes
disconnected. No more messages can be sent, but any remaining messages can still be received.
Send and receive operations on a disconnected channel never block.

```
use crossbeam_channel::{unbounded, RecvError};

let (s, r) = unbounded();
s.send(1).unwrap();
s.send(2).unwrap();
s.send(3).unwrap();

// The only sender is dropped, disconnecting the channel.
drop(s);

// The remaining messages can be received.
assert_eq!(r.recv(), Ok(1));
assert_eq!(r.recv(), Ok(2));
assert_eq!(r.recv(), Ok(3));

// There are no more messages in the channel.
assert!(r.is_empty());

// Note that calling `r.recv()` does not block.
// Instead, `Err(RecvError)` is returned immediately.
assert_eq!(r.recv(), Err(RecvError));
```

# Blocking operations

Send and receive operations come in three flavors:

* Non-blocking (returns immediately with success or failure).
* Blocking (waits until the operation succeeds or the channel becomes disconnected).
* Blocking with a timeout (blocks only for a certain duration of time).

A simple example showing the difference between non-blocking and blocking operations:

```
use crossbeam_channel::{bounded, RecvError, TryRecvError};

let (s, r) = bounded(1);

// Send a message into the channel.
s.send("foo").unwrap();

// This call would block because the channel is full.
// s.send("bar").unwrap();

// Receive the message.
assert_eq!(r.recv(), Ok("foo"));

// This call would block because the channel is empty.
// r.recv();

// Try receiving a message without blocking.
assert_eq!(r.try_recv(), Err(TryRecvError::Empty));

// Disconnect the channel.
drop(s);

// This call doesn't block because the channel is now disconnected.
assert_eq!(r.recv(), Err(RecvError));
```

# Iteration

Receivers can be used as iterators. For example, method [`iter`] creates an iterator that
receives messages until the channel becomes empty and disconnected. Note that iteration may
block waiting for next message to arrive.

```
use std::thread;
use crossbeam_channel::unbounded;

let (s, r) = unbounded();

thread::spawn(move || {
    s.send(1).unwrap();
    s.send(2).unwrap();
    s.send(3).unwrap();
    drop(s); // Disconnect the channel.
});

// Collect all messages from the channel.
// Note that the call to `collect` blocks until the sender is dropped.
let v: Vec<_> = r.iter().collect();

assert_eq!(v, [1, 2, 3]);
```

A non-blocking iterator can be created using [`try_iter`], which receives all available
messages without blocking:

```
use crossbeam_channel::unbounded;

let (s, r) = unbounded();
s.send(1).unwrap();
s.send(2).unwrap();
s.send(3).unwrap();
// No need to drop the sender.

// Receive all messages currently in the channel.
let v: Vec<_> = r.try_iter().collect();

assert_eq!(v, [1, 2, 3]);
```

# Selection

The [`select!`] macro allows you to define a set of channel operations, wait until any one of
them becomes ready, and finally execute it. If multiple operations are ready at the same time,
a random one among them is selected.

It is also possible to define a `default` case that gets executed if none of the operations are
ready, either right away or for a certain duration of time.

An operation is considered to be ready if it doesn't have to block. Note that it is ready even
when it will simply return an error because the channel is disconnected.

An example of receiving a message from two channels:

```
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thread::spawn(move || s1.send(10).unwrap());
thread::spawn(move || s2.send(20).unwrap());

// At most one of these two receive operations will be executed.
select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    recv(r2) -> msg => assert_eq!(msg, Ok(20)),
    default(Duration::from_secs(1)) => println!("timed out"),
}
```

If you need to select over a dynamically created list of channel operations, use [`Select`]
instead. The [`select!`] macro is just a convenience wrapper around [`Select`].

# Extra channels

Three functions can create special kinds of channels, all of which return just a [`Receiver`]
handle:

* [`after`] creates a channel that delivers a single message after a certain duration of time.
* [`tick`] creates a channel that delivers messages periodically.
* [`never`](never()) creates a channel that never delivers messages.

These channels are very efficient because messages get lazily generated on receive operations.

An example that prints elapsed time every 50 milliseconds for the duration of 1 second:

```
use std::time::{Duration, Instant};
use crossbeam_channel::{after, select, tick};

let start = Instant::now();
let ticker = tick(Duration::from_millis(50));
let timeout = after(Duration::from_secs(1));

loop {
    select! {
        recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
        recv(timeout) -> _ => break,
    }
}
```

[`send`]: Sender::send
[`recv`]: Receiver::recv
[`iter`]: Receiver::iter
[`try_iter`]: Receiver::try_iter

## Macros

### Macro `select`

**Attributes:**

- `#[macro_export]`

Selects from a set of channel operations.

This macro allows you to define a set of channel operations, wait until any one of them becomes
ready, and finally execute it. If multiple operations are ready at the same time, a random one
among them is selected (i.e. the unbiased selection). Use `select_biased!` for the biased
selection.

It is also possible to define a `default` case that gets executed if none of the operations are
ready, either right away or for a certain duration of time.

An operation is considered to be ready if it doesn't have to block. Note that it is ready even
when it will simply return an error because the channel is disconnected.

The `select!` macro is a convenience wrapper around [`Select`]. However, it cannot select over a
dynamically created list of channel operations.

[`Select`]: super::Select

# Examples

Block until a send or a receive operation is selected:

```
use crossbeam_channel::{select, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();
s1.send(10).unwrap();

// Since both operations are initially ready, a random one will be executed.
select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    send(s2, 20) -> res => {
        assert_eq!(res, Ok(()));
        assert_eq!(r2.recv(), Ok(20));
    }
}
```

Select from a set of operations without blocking:

```
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thread::spawn(move || {
    thread::sleep(Duration::from_secs(1));
    s1.send(10).unwrap();
});
thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    s2.send(20).unwrap();
});

// None of the operations are initially ready.
select! {
    recv(r1) -> msg => panic!(),
    recv(r2) -> msg => panic!(),
    default => println!("not ready"),
}
```

Select over a set of operations with a timeout:

```
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thread::spawn(move || {
    thread::sleep(Duration::from_secs(1));
    s1.send(10).unwrap();
});
thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    s2.send(20).unwrap();
});

// None of the two operations will become ready within 100 milliseconds.
select! {
    recv(r1) -> msg => panic!(),
    recv(r2) -> msg => panic!(),
    default(Duration::from_millis(100)) => println!("timed out"),
}
```

Optionally add a receive operation to `select!` using [`never`]:

```
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, never, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thread::spawn(move || {
    thread::sleep(Duration::from_secs(1));
    s1.send(10).unwrap();
});
thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    s2.send(20).unwrap();
});

// This receiver can be a `Some` or a `None`.
let r2 = Some(&r2);

// None of the two operations will become ready within 100 milliseconds.
select! {
    recv(r1) -> msg => panic!(),
    recv(r2.unwrap_or(&never())) -> msg => assert_eq!(msg, Ok(20)),
}
```

To optionally add a timeout to `select!`, see the [example] for [`never`].

[`never`]: super::never
[example]: super::never#examples

```rust
pub macro_rules! select {
    /* macro_rules! select {
    ($($tokens:tt)*) => { ... };
} */
}
```

### Macro `select_biased`

**Attributes:**

- `#[macro_export]`

Selects from a set of channel operations.

This macro allows you to define a list of channel operations, wait until any one of them
becomes ready, and finally execute it. If multiple operations are ready at the same time, the
operation nearest to the front of the list is always selected (i.e. the biased selection). Use
[`select!`] for the unbiased selection.

Otherwise, this macro's functionality is identical to [`select!`]. Refer to it for the syntax.

```rust
pub macro_rules! select_biased {
    /* macro_rules! select_biased {
    ($($tokens:tt)*) => { ... };
} */
}
```

## Re-exports

### Re-export `after`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::after;
```

### Re-export `at`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::at;
```

### Re-export `bounded`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::bounded;
```

### Re-export `never`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::never;
```

### Re-export `tick`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::tick;
```

### Re-export `unbounded`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::unbounded;
```

### Re-export `IntoIter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::IntoIter;
```

### Re-export `Iter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::Iter;
```

### Re-export `Receiver`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::Receiver;
```

### Re-export `Sender`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::Sender;
```

### Re-export `TryIter`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::channel::TryIter;
```

### Re-export `ReadyTimeoutError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::ReadyTimeoutError;
```

### Re-export `RecvError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::RecvError;
```

### Re-export `RecvTimeoutError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::RecvTimeoutError;
```

### Re-export `SelectTimeoutError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::SelectTimeoutError;
```

### Re-export `SendError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::SendError;
```

### Re-export `SendTimeoutError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::SendTimeoutError;
```

### Re-export `TryReadyError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::TryReadyError;
```

### Re-export `TryRecvError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::TryRecvError;
```

### Re-export `TrySelectError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::TrySelectError;
```

### Re-export `TrySendError`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::err::TrySendError;
```

### Re-export `Select`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::select::Select;
```

### Re-export `SelectedOperation`

**Attributes:**

- `#[<cfg>(feature = "std")]`

```rust
pub use crate::select::SelectedOperation;
```

