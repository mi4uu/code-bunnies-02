# Crate Documentation

**Version:** 0.3.31

**Format Version:** 43

# Module `futures_sink`

Asynchronous sinks

This crate contains the `Sink` trait which allows values to be sent
asynchronously.

## Traits

### Trait `Sink`

**Attributes:**

- `#[must_use = "sinks do nothing unless polled"]`

A `Sink` is a value into which other values can be sent, asynchronously.

Basic examples of sinks include the sending side of:

- Channels
- Sockets
- Pipes

In addition to such "primitive" sinks, it's typical to layer additional
functionality, such as buffering, on top of an existing sink.

Sending to a sink is "asynchronous" in the sense that the value may not be
sent in its entirety immediately. Instead, values are sent in a two-phase
way: first by initiating a send, and then by polling for completion. This
two-phase setup is analogous to buffered writing in synchronous code, where
writes often succeed immediately, but internally are buffered and are
*actually* written only upon flushing.

In addition, the `Sink` may be *full*, in which case it is not even possible
to start the sending process.

As with `Future` and `Stream`, the `Sink` trait is built from a few core
required methods, and a host of default methods for working in a
higher-level way. The `Sink::send_all` combinator is of particular
importance: you can use it to send an entire stream to a sink, which is
the simplest way to ultimately consume a stream.

```rust
pub trait Sink<Item> {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Error`: The type of value produced by the sink when an error occurs.

##### Required Methods

- `poll_ready`: Attempts to prepare the `Sink` to receive a value.
- `start_send`: Begin the process of sending a value to the sink.
- `poll_flush`: Flush any remaining output from this sink.
- `poll_close`: Flush any remaining output and close this sink, if necessary.

#### Implementations

This trait is implemented for the following types:

- `&mut S` with <S: ?Sized + Sink<Item> + Unpin, Item>
- `core::pin::Pin<P>` with <P, Item>
- `alloc::vec::Vec<T>` with <T>
- `alloc::collections::VecDeque<T>` with <T>
- `alloc::boxed::Box<S>` with <S: ?Sized + Sink<Item> + Unpin, Item>

