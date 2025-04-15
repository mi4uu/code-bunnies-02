# Crate Documentation

**Version:** 0.7.14

**Format Version:** 43

# Module `tokio_util`

Utilities for working with Tokio.

This crate is not versioned in lockstep with the core
[`tokio`] crate. However, `tokio-util` _will_ respect Rust's
semantic versioning policy, especially with regard to breaking changes.

[`tokio`]: https://docs.rs/tokio

## Modules

## Module `codec`

**Attributes:**

- `#[<cfg>(feature = "codec")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "codec")))]`

 Adaptors from `AsyncRead`/`AsyncWrite` to Stream/Sink

 Raw I/O objects work with byte sequences, but higher-level code usually
 wants to batch these into meaningful chunks, called "frames".

 This module contains adapters to go from streams of bytes, [`AsyncRead`] and
 [`AsyncWrite`], to framed streams implementing [`Sink`] and [`Stream`].
 Framed streams are also known as transports.

 # Example encoding using `LinesCodec`

 The following example demonstrates how to use a codec such as [`LinesCodec`] to
 write framed data. [`FramedWrite`] can be used to achieve this. Data sent to
 [`FramedWrite`] are first framed according to a specific codec, and then sent to
 an implementor of [`AsyncWrite`].

 ```
 use futures::sink::SinkExt;
 use tokio_util::codec::LinesCodec;
 use tokio_util::codec::FramedWrite;

 #[tokio::main]
 async fn main() {
     let buffer = Vec::new();
     let messages = vec!["Hello", "World"];
     let encoder = LinesCodec::new();

     // FramedWrite is a sink which means you can send values into it
     // asynchronously.
     let mut writer = FramedWrite::new(buffer, encoder);

     // To be able to send values into a FramedWrite, you need to bring the
     // `SinkExt` trait into scope.
     writer.send(messages[0]).await.unwrap();
     writer.send(messages[1]).await.unwrap();

     let buffer = writer.get_ref();

     assert_eq!(buffer.as_slice(), "Hello\nWorld\n".as_bytes());
 }
```

 # Example decoding using `LinesCodec`
 The following example demonstrates how to use a codec such as [`LinesCodec`] to
 read a stream of framed data. [`FramedRead`] can be used to achieve this. [`FramedRead`]
 will keep reading from an [`AsyncRead`] implementor until a whole frame, according to a codec,
 can be parsed.

```
 use tokio_stream::StreamExt;
 use tokio_util::codec::LinesCodec;
 use tokio_util::codec::FramedRead;

 #[tokio::main]
 async fn main() {
     let message = "Hello\nWorld".as_bytes();
     let decoder = LinesCodec::new();

     // FramedRead can be used to read a stream of values that are framed according to
     // a codec. FramedRead will read from its input (here `buffer`) until a whole frame
     // can be parsed.
     let mut reader = FramedRead::new(message, decoder);

     // To read values from a FramedRead, you need to bring the
     // `StreamExt` trait into scope.
     let frame1 = reader.next().await.unwrap().unwrap();
     let frame2 = reader.next().await.unwrap().unwrap();

     assert!(reader.next().await.is_none());
     assert_eq!(frame1, "Hello");
     assert_eq!(frame2, "World");
 }
 ```

 # The Decoder trait

 A [`Decoder`] is used together with [`FramedRead`] or [`Framed`] to turn an
 [`AsyncRead`] into a [`Stream`]. The job of the decoder trait is to specify
 how sequences of bytes are turned into a sequence of frames, and to
 determine where the boundaries between frames are.  The job of the
 `FramedRead` is to repeatedly switch between reading more data from the IO
 resource, and asking the decoder whether we have received enough data to
 decode another frame of data.

 The main method on the `Decoder` trait is the [`decode`] method. This method
 takes as argument the data that has been read so far, and when it is called,
 it will be in one of the following situations:

  1. The buffer contains less than a full frame.
  2. The buffer contains exactly a full frame.
  3. The buffer contains more than a full frame.

 In the first situation, the decoder should return `Ok(None)`.

 In the second situation, the decoder should clear the provided buffer and
 return `Ok(Some(the_decoded_frame))`.

 In the third situation, the decoder should use a method such as [`split_to`]
 or [`advance`] to modify the buffer such that the frame is removed from the
 buffer, but any data in the buffer after that frame should still remain in
 the buffer. The decoder should also return `Ok(Some(the_decoded_frame))` in
 this case.

 Finally the decoder may return an error if the data is invalid in some way.
 The decoder should _not_ return an error just because it has yet to receive
 a full frame.

 It is guaranteed that, from one call to `decode` to another, the provided
 buffer will contain the exact same data as before, except that if more data
 has arrived through the IO resource, that data will have been appended to
 the buffer.  This means that reading frames from a `FramedRead` is
 essentially equivalent to the following loop:

 ```no_run
 use tokio::io::AsyncReadExt;
 # // This uses async_stream to create an example that compiles.
 # fn foo() -> impl futures_core::Stream<Item = std::io::Result<bytes::BytesMut>> { async_stream::try_stream! {
 # use tokio_util::codec::Decoder;
 # let mut decoder = tokio_util::codec::BytesCodec::new();
 # let io_resource = &mut &[0u8, 1, 2, 3][..];

 let mut buf = bytes::BytesMut::new();
 loop {
     // The read_buf call will append to buf rather than overwrite existing data.
     let len = io_resource.read_buf(&mut buf).await?;

     if len == 0 {
         while let Some(frame) = decoder.decode_eof(&mut buf)? {
             yield frame;
         }
         break;
     }

     while let Some(frame) = decoder.decode(&mut buf)? {
         yield frame;
     }
 }
 # }}
 ```
 The example above uses `yield` whenever the `Stream` produces an item.

 ## Example decoder

 As an example, consider a protocol that can be used to send strings where
 each frame is a four byte integer that contains the length of the frame,
 followed by that many bytes of string data. The decoder fails with an error
 if the string data is not valid utf-8 or too long.

 Such a decoder can be written like this:
 ```
 use tokio_util::codec::Decoder;
 use bytes::{BytesMut, Buf};

 struct MyStringDecoder {}

 const MAX: usize = 8 * 1024 * 1024;

 impl Decoder for MyStringDecoder {
     type Item = String;
     type Error = std::io::Error;

     fn decode(
         &mut self,
         src: &mut BytesMut
     ) -> Result<Option<Self::Item>, Self::Error> {
         if src.len() < 4 {
             // Not enough data to read length marker.
             return Ok(None);
         }

         // Read length marker.
         let mut length_bytes = [0u8; 4];
         length_bytes.copy_from_slice(&src[..4]);
         let length = u32::from_le_bytes(length_bytes) as usize;

         // Check that the length is not too large to avoid a denial of
         // service attack where the server runs out of memory.
         if length > MAX {
             return Err(std::io::Error::new(
                 std::io::ErrorKind::InvalidData,
                 format!("Frame of length {} is too large.", length)
             ));
         }

         if src.len() < 4 + length {
             // The full string has not yet arrived.
             //
             // We reserve more space in the buffer. This is not strictly
             // necessary, but is a good idea performance-wise.
             src.reserve(4 + length - src.len());

             // We inform the Framed that we need more bytes to form the next
             // frame.
             return Ok(None);
         }

         // Use advance to modify src such that it no longer contains
         // this frame.
         let data = src[4..4 + length].to_vec();
         src.advance(4 + length);

         // Convert the data to a string, or fail if it is not valid utf-8.
         match String::from_utf8(data) {
             Ok(string) => Ok(Some(string)),
             Err(utf8_error) => {
                 Err(std::io::Error::new(
                     std::io::ErrorKind::InvalidData,
                     utf8_error.utf8_error(),
                 ))
             },
         }
     }
 }
 ```

 # The Encoder trait

 An [`Encoder`] is used together with [`FramedWrite`] or [`Framed`] to turn
 an [`AsyncWrite`] into a [`Sink`]. The job of the encoder trait is to
 specify how frames are turned into a sequences of bytes.  The job of the
 `FramedWrite` is to take the resulting sequence of bytes and write it to the
 IO resource.

 The main method on the `Encoder` trait is the [`encode`] method. This method
 takes an item that is being written, and a buffer to write the item to. The
 buffer may already contain data, and in this case, the encoder should append
 the new frame to the buffer rather than overwrite the existing data.

 It is guaranteed that, from one call to `encode` to another, the provided
 buffer will contain the exact same data as before, except that some of the
 data may have been removed from the front of the buffer. Writing to a
 `FramedWrite` is essentially equivalent to the following loop:

 ```no_run
 use tokio::io::AsyncWriteExt;
 use bytes::Buf; // for advance
 # use tokio_util::codec::Encoder;
 # async fn next_frame() -> bytes::Bytes { bytes::Bytes::new() }
 # async fn no_more_frames() { }
 # #[tokio::main] async fn main() -> std::io::Result<()> {
 # let mut io_resource = tokio::io::sink();
 # let mut encoder = tokio_util::codec::BytesCodec::new();

 const MAX: usize = 8192;

 let mut buf = bytes::BytesMut::new();
 loop {
     tokio::select! {
         num_written = io_resource.write(&buf), if !buf.is_empty() => {
             buf.advance(num_written?);
         },
         frame = next_frame(), if buf.len() < MAX => {
             encoder.encode(frame, &mut buf)?;
         },
         _ = no_more_frames() => {
             io_resource.write_all(&buf).await?;
             io_resource.shutdown().await?;
             return Ok(());
         },
     }
 }
 # }
 ```
 Here the `next_frame` method corresponds to any frames you write to the
 `FramedWrite`. The `no_more_frames` method corresponds to closing the
 `FramedWrite` with [`SinkExt::close`].

 ## Example encoder

 As an example, consider a protocol that can be used to send strings where
 each frame is a four byte integer that contains the length of the frame,
 followed by that many bytes of string data. The encoder will fail if the
 string is too long.

 Such an encoder can be written like this:
 ```
 use tokio_util::codec::Encoder;
 use bytes::BytesMut;

 struct MyStringEncoder {}

 const MAX: usize = 8 * 1024 * 1024;

 impl Encoder<String> for MyStringEncoder {
     type Error = std::io::Error;

     fn encode(&mut self, item: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
         // Don't send a string if it is longer than the other end will
         // accept.
         if item.len() > MAX {
             return Err(std::io::Error::new(
                 std::io::ErrorKind::InvalidData,
                 format!("Frame of length {} is too large.", item.len())
             ));
         }

         // Convert the length into a byte array.
         // The cast to u32 cannot overflow due to the length check above.
         let len_slice = u32::to_le_bytes(item.len() as u32);

         // Reserve space in the buffer.
         dst.reserve(4 + item.len());

         // Write the length and string to the buffer.
         dst.extend_from_slice(&len_slice);
         dst.extend_from_slice(item.as_bytes());
         Ok(())
     }
 }
 ```

 [`AsyncRead`]: tokio::io::AsyncRead
 [`AsyncWrite`]: tokio::io::AsyncWrite
 [`Stream`]: futures_core::Stream
 [`Sink`]: futures_sink::Sink
 [`SinkExt`]: futures::sink::SinkExt
 [`SinkExt::close`]: https://docs.rs/futures/0.3/futures/sink/trait.SinkExt.html#method.close
 [`FramedRead`]: struct@crate::codec::FramedRead
 [`FramedWrite`]: struct@crate::codec::FramedWrite
 [`Framed`]: struct@crate::codec::Framed
 [`Decoder`]: trait@crate::codec::Decoder
 [`decode`]: fn@crate::codec::Decoder::decode
 [`encode`]: fn@crate::codec::Encoder::encode
 [`split_to`]: fn@bytes::BytesMut::split_to
 [`advance`]: fn@bytes::Buf::advance

```rust
pub mod codec { /* ... */ }
```

### Modules

## Module `length_delimited`

Frame a stream of bytes based on a length prefix

Many protocols delimit their frames by prefacing frame data with a
frame head that specifies the length of the frame. The
`length_delimited` module provides utilities for handling the length
based framing. This allows the consumer to work with entire frames
without having to worry about buffering or other framing logic.

# Getting started

If implementing a protocol from scratch, using length delimited framing
is an easy way to get started. [`LengthDelimitedCodec::new()`] will
return a length delimited codec using default configuration values.
This can then be used to construct a framer to adapt a full-duplex
byte stream into a stream of frames.

```
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

fn bind_transport<T: AsyncRead + AsyncWrite>(io: T)
    -> Framed<T, LengthDelimitedCodec>
{
    Framed::new(io, LengthDelimitedCodec::new())
}
# pub fn main() {}
```

The returned transport implements `Sink + Stream` for `BytesMut`. It
encodes the frame with a big-endian `u32` header denoting the frame
payload length:

```text
+----------+--------------------------------+
| len: u32 |          frame payload         |
+----------+--------------------------------+
```

Specifically, given the following:

```
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use futures::SinkExt;
use bytes::Bytes;

async fn write_frame<T>(io: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let mut transport = Framed::new(io, LengthDelimitedCodec::new());
    let frame = Bytes::from("hello world");

    transport.send(frame).await?;
    Ok(())
}
```

The encoded frame will look like this:

```text
+---- len: u32 ----+---- data ----+
| \x00\x00\x00\x0b |  hello world |
+------------------+--------------+
```

# Decoding

[`FramedRead`] adapts an [`AsyncRead`] into a `Stream` of [`BytesMut`],
such that each yielded [`BytesMut`] value contains the contents of an
entire frame. There are many configuration parameters enabling
[`FramedRead`] to handle a wide range of protocols. Here are some
examples that will cover the various options at a high level.

## Example 1

The following will parse a `u16` length field at offset 0, omitting the
frame head in the yielded `BytesMut`.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\x00\x0BHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(0) // default value
    .length_field_type::<u16>()
    .length_adjustment(0)   // default value
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"Hello world");
# }
```

The following frame will be decoded as such:

```text
         INPUT                        DECODED
+-- len ---+--- Payload ---+     +--- Payload ---+
| \x00\x0B |  Hello world  | --> |  Hello world  |
+----------+---------------+     +---------------+
```

The value of the length field is 11 (`\x0B`) which represents the length
of the payload, `hello world`. By default, [`FramedRead`] assumes that
the length field represents the number of bytes that **follows** the
length field. Thus, the entire frame has a length of 13: 2 bytes for the
frame head + 11 bytes for the payload.

## Example 2

The following will parse a `u16` length field at offset 0, including the
frame head in the yielded `BytesMut`.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\x00\x0BHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(0) // default value
    .length_field_type::<u16>()
    .length_adjustment(2)   // Add head size to length
    .num_skip(0)            // Do NOT skip the head
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"\x00\x0BHello world");
# }
```

The following frame will be decoded as such:

```text
         INPUT                           DECODED
+-- len ---+--- Payload ---+     +-- len ---+--- Payload ---+
| \x00\x0B |  Hello world  | --> | \x00\x0B |  Hello world  |
+----------+---------------+     +----------+---------------+
```

This is similar to the first example, the only difference is that the
frame head is **included** in the yielded `BytesMut` value. To achieve
this, we need to add the header size to the length with `length_adjustment`,
and set `num_skip` to `0` to prevent skipping the head.

## Example 3

The following will parse a `u16` length field at offset 0, omitting the
frame head in the yielded `BytesMut`. In this case, the length field
**includes** the frame head length.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\x00\x0DHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(0) // default value
    .length_field_type::<u16>()
    .length_adjustment(-2)  // size of head
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"Hello world");
# }
```

The following frame will be decoded as such:

```text
         INPUT                           DECODED
+-- len ---+--- Payload ---+     +--- Payload ---+
| \x00\x0D |  Hello world  | --> |  Hello world  |
+----------+---------------+     +---------------+
```

In most cases, the length field represents the length of the payload
only, as shown in the previous examples. However, in some protocols the
length field represents the length of the whole frame, including the
head. In such cases, we specify a negative `length_adjustment` to adjust
the value provided in the frame head to represent the payload length.

## Example 4

The following will parse a 3 byte length field at offset 0 in a 5 byte
frame head, including the frame head in the yielded `BytesMut`.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\x00\x00\x0B\xCA\xFEHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(0) // default value
    .length_field_length(3)
    .length_adjustment(3 + 2)  // len field and remaining head
    .num_skip(0)
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"\x00\x00\x0B\xCA\xFEHello world");
# }
```

The following frame will be decoded as such:

```text
                 INPUT
+---- len -----+- head -+--- Payload ---+
| \x00\x00\x0B | \xCAFE |  Hello world  |
+--------------+--------+---------------+

                 DECODED
+---- len -----+- head -+--- Payload ---+
| \x00\x00\x0B | \xCAFE |  Hello world  |
+--------------+--------+---------------+
```

A more advanced example that shows a case where there is extra frame
head data between the length field and the payload. In such cases, it is
usually desirable to include the frame head as part of the yielded
`BytesMut`. This lets consumers of the length delimited framer to
process the frame head as needed.

The positive `length_adjustment` value lets `FramedRead` factor in the
additional head into the frame length calculation.

## Example 5

The following will parse a `u16` length field at offset 1 of a 4 byte
frame head. The first byte and the length field will be omitted from the
yielded `BytesMut`, but the trailing 2 bytes of the frame head will be
included.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\xCA\x00\x0B\xFEHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(1) // length of hdr1
    .length_field_type::<u16>()
    .length_adjustment(1)  // length of hdr2
    .num_skip(3) // length of hdr1 + LEN
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"\xFEHello world");
# }
```

The following frame will be decoded as such:

```text
                 INPUT
+- hdr1 -+-- len ---+- hdr2 -+--- Payload ---+
|  \xCA  | \x00\x0B |  \xFE  |  Hello world  |
+--------+----------+--------+---------------+

         DECODED
+- hdr2 -+--- Payload ---+
|  \xFE  |  Hello world  |
+--------+---------------+
```

The length field is situated in the middle of the frame head. In this
case, the first byte in the frame head could be a version or some other
identifier that is not needed for processing. On the other hand, the
second half of the head is needed.

`length_field_offset` indicates how many bytes to skip before starting
to read the length field.  `length_adjustment` is the number of bytes to
skip starting at the end of the length field. In this case, it is the
second half of the head.

## Example 6

The following will parse a `u16` length field at offset 1 of a 4 byte
frame head. The first byte and the length field will be omitted from the
yielded `BytesMut`, but the trailing 2 bytes of the frame head will be
included. In this case, the length field **includes** the frame head
length.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\xCA\x00\x0F\xFEHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(1) // length of hdr1
    .length_field_type::<u16>()
    .length_adjustment(-3)  // length of hdr1 + LEN, negative
    .num_skip(3)
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"\xFEHello world");
# }
```

The following frame will be decoded as such:

```text
                 INPUT
+- hdr1 -+-- len ---+- hdr2 -+--- Payload ---+
|  \xCA  | \x00\x0F |  \xFE  |  Hello world  |
+--------+----------+--------+---------------+

         DECODED
+- hdr2 -+--- Payload ---+
|  \xFE  |  Hello world  |
+--------+---------------+
```

Similar to the example above, the difference is that the length field
represents the length of the entire frame instead of just the payload.
The length of `hdr1` and `len` must be counted in `length_adjustment`.
Note that the length of `hdr2` does **not** need to be explicitly set
anywhere because it already is factored into the total frame length that
is read from the byte stream.

## Example 7

The following will parse a 3 byte length field at offset 0 in a 4 byte
frame head, excluding the 4th byte from the yielded `BytesMut`.

```
# use tokio_stream::StreamExt;
# use tokio_util::codec::LengthDelimitedCodec;
# #[tokio::main]
# async fn main() {
# let io: &[u8] = b"\x00\x00\x0B\xFFHello world";
let mut reader = LengthDelimitedCodec::builder()
    .length_field_offset(0) // default value
    .length_field_length(3)
    .length_adjustment(0)  // default value
    .num_skip(4) // skip the first 4 bytes
    .new_read(io);
# let res = reader.next().await.unwrap().unwrap().to_vec();
# assert_eq!(res, b"Hello world");
# }
```

The following frame will be decoded as such:

```text
                 INPUT                       DECODED
+------- len ------+--- Payload ---+    +--- Payload ---+
| \x00\x00\x0B\xFF |  Hello world  | => |  Hello world  |
+------------------+---------------+    +---------------+
```

A simple example where there are unused bytes between the length field
and the payload.

# Encoding

[`FramedWrite`] adapts an [`AsyncWrite`] into a `Sink` of [`BytesMut`],
such that each submitted [`BytesMut`] is prefaced by a length field.
There are fewer configuration options than [`FramedRead`]. Given
protocols that have more complex frame heads, an encoder should probably
be written by hand using [`Encoder`].

Here is a simple example, given a `FramedWrite` with the following
configuration:

```
# use tokio::io::AsyncWrite;
# use tokio_util::codec::LengthDelimitedCodec;
# fn write_frame<T: AsyncWrite>(io: T) {
# let _ =
LengthDelimitedCodec::builder()
    .length_field_type::<u16>()
    .new_write(io);
# }
# pub fn main() {}
```

A payload of `hello world` will be encoded as:

```text
+- len: u16 -+---- data ----+
|  \x00\x0b  |  hello world |
+------------+--------------+
```

[`LengthDelimitedCodec::new()`]: method@LengthDelimitedCodec::new
[`FramedRead`]: struct@FramedRead
[`FramedWrite`]: struct@FramedWrite
[`AsyncRead`]: trait@tokio::io::AsyncRead
[`AsyncWrite`]: trait@tokio::io::AsyncWrite
[`Encoder`]: trait@Encoder
[`BytesMut`]: bytes::BytesMut

```rust
pub mod length_delimited { /* ... */ }
```

### Types

#### Struct `Builder`

Configure length delimited `LengthDelimitedCodec`s.

`Builder` enables constructing configured length delimited codecs. Note
that not all configuration settings apply to both encoding and decoding. See
the documentation for specific methods for more detail.

Note that the if the value of [`Builder::max_frame_length`] becomes larger than
what can actually fit in [`Builder::length_field_length`], it will be clipped to
the maximum value that can fit.

```rust
pub struct Builder {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Builder { /* ... */ }
  ```
  Creates a new length delimited codec builder with default configuration

- ```rust
  pub fn big_endian(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Read the length field as a big endian integer

- ```rust
  pub fn little_endian(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Read the length field as a little endian integer

- ```rust
  pub fn native_endian(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Read the length field as a native endian integer

- ```rust
  pub fn max_frame_length(self: &mut Self, val: usize) -> &mut Self { /* ... */ }
  ```
  Sets the max frame length in bytes

- ```rust
  pub fn length_field_type<T: builder::LengthFieldType>(self: &mut Self) -> &mut Self { /* ... */ }
  ```
  Sets the unsigned integer type used to represent the length field.

- ```rust
  pub fn length_field_length(self: &mut Self, val: usize) -> &mut Self { /* ... */ }
  ```
  Sets the number of bytes used to represent the length field

- ```rust
  pub fn length_field_offset(self: &mut Self, val: usize) -> &mut Self { /* ... */ }
  ```
  Sets the number of bytes in the header before the length field

- ```rust
  pub fn length_adjustment(self: &mut Self, val: isize) -> &mut Self { /* ... */ }
  ```
  Delta between the payload length specified in the header and the real

- ```rust
  pub fn num_skip(self: &mut Self, val: usize) -> &mut Self { /* ... */ }
  ```
  Sets the number of bytes to skip before reading the payload

- ```rust
  pub fn new_codec(self: &Self) -> LengthDelimitedCodec { /* ... */ }
  ```
  Create a configured length delimited `LengthDelimitedCodec`

- ```rust
  pub fn new_read<T>(self: &Self, upstream: T) -> FramedRead<T, LengthDelimitedCodec>
where
    T: AsyncRead { /* ... */ }
  ```
  Create a configured length delimited `FramedRead`

- ```rust
  pub fn new_write<T>(self: &Self, inner: T) -> FramedWrite<T, LengthDelimitedCodec>
where
    T: AsyncWrite { /* ... */ }
  ```
  Create a configured length delimited `FramedWrite`

- ```rust
  pub fn new_framed<T>(self: &Self, inner: T) -> Framed<T, LengthDelimitedCodec>
where
    T: AsyncRead + AsyncWrite { /* ... */ }
  ```
  Create a configured length delimited `Framed`

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Builder { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `LengthDelimitedCodecError`

An error when the number of bytes read is more than max frame length.

```rust
pub struct LengthDelimitedCodecError {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Error**
#### Struct `LengthDelimitedCodec`

A codec for frames delimited by a frame head specifying their lengths.

This allows the consumer to work with entire frames without having to worry
about buffering or other framing logic.

See [module level] documentation for more detail.

[module level]: index.html

```rust
pub struct LengthDelimitedCodec {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new `LengthDelimitedCodec` with the default configuration values.

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new length delimited codec builder with default configuration

- ```rust
  pub fn max_frame_length(self: &Self) -> usize { /* ... */ }
  ```
  Returns the current max frame setting

- ```rust
  pub fn set_max_frame_length(self: &mut Self, val: usize) { /* ... */ }
  ```
  Updates the max frame setting.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LengthDelimitedCodec { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Decoder**
  - ```rust
    fn decode(self: &mut Self, src: &mut BytesMut) -> io::Result<Option<BytesMut>> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Encoder**
  - ```rust
    fn encode(self: &mut Self, data: Bytes, dst: &mut BytesMut) -> Result<(), io::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `BytesCodec`

```rust
pub use self::bytes_codec::BytesCodec;
```

#### Re-export `Decoder`

```rust
pub use self::decoder::Decoder;
```

#### Re-export `Encoder`

```rust
pub use self::encoder::Encoder;
```

#### Re-export `Framed`

```rust
pub use self::framed::Framed;
```

#### Re-export `FramedParts`

```rust
pub use self::framed::FramedParts;
```

#### Re-export `FramedRead`

```rust
pub use self::framed_read::FramedRead;
```

#### Re-export `FramedWrite`

```rust
pub use self::framed_write::FramedWrite;
```

#### Re-export `LengthDelimitedCodec`

```rust
pub use self::length_delimited::LengthDelimitedCodec;
```

#### Re-export `LengthDelimitedCodecError`

```rust
pub use self::length_delimited::LengthDelimitedCodecError;
```

#### Re-export `LinesCodec`

```rust
pub use self::lines_codec::LinesCodec;
```

#### Re-export `LinesCodecError`

```rust
pub use self::lines_codec::LinesCodecError;
```

#### Re-export `AnyDelimiterCodec`

```rust
pub use self::any_delimiter_codec::AnyDelimiterCodec;
```

#### Re-export `AnyDelimiterCodecError`

```rust
pub use self::any_delimiter_codec::AnyDelimiterCodecError;
```

## Module `io`

**Attributes:**

- `#[<cfg>(feature = "io")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io")))]`

Helpers for IO related tasks.

The stream types are often used in combination with hyper or reqwest, as they
allow converting between a hyper [`Body`] and [`AsyncRead`].

The [`SyncIoBridge`] type converts from the world of async I/O
to synchronous I/O; this may often come up when using synchronous APIs
inside [`tokio::task::spawn_blocking`].

[`Body`]: https://docs.rs/hyper/0.13/hyper/struct.Body.html
[`AsyncRead`]: tokio::io::AsyncRead

```rust
pub mod io { /* ... */ }
```

### Re-exports

#### Re-export `CopyToBytes`

```rust
pub use self::copy_to_bytes::CopyToBytes;
```

#### Re-export `InspectReader`

```rust
pub use self::inspect::InspectReader;
```

#### Re-export `InspectWriter`

```rust
pub use self::inspect::InspectWriter;
```

#### Re-export `read_buf`

```rust
pub use self::read_buf::read_buf;
```

#### Re-export `ReaderStream`

```rust
pub use self::reader_stream::ReaderStream;
```

#### Re-export `SinkWriter`

```rust
pub use self::sink_writer::SinkWriter;
```

#### Re-export `StreamReader`

```rust
pub use self::stream_reader::StreamReader;
```

#### Re-export `poll_read_buf`

```rust
pub use crate::util::poll_read_buf;
```

#### Re-export `poll_write_buf`

```rust
pub use crate::util::poll_write_buf;
```

## Module `sync`

Synchronization primitives

```rust
pub mod sync { /* ... */ }
```

### Re-exports

#### Re-export `DropGuard`

```rust
pub use cancellation_token::guard::DropGuard;
```

#### Re-export `CancellationToken`

```rust
pub use cancellation_token::CancellationToken;
```

#### Re-export `WaitForCancellationFuture`

```rust
pub use cancellation_token::WaitForCancellationFuture;
```

#### Re-export `WaitForCancellationFutureOwned`

```rust
pub use cancellation_token::WaitForCancellationFutureOwned;
```

#### Re-export `PollSendError`

```rust
pub use mpsc::PollSendError;
```

#### Re-export `PollSender`

```rust
pub use mpsc::PollSender;
```

#### Re-export `PollSemaphore`

```rust
pub use poll_semaphore::PollSemaphore;
```

#### Re-export `ReusableBoxFuture`

```rust
pub use reusable_box::ReusableBoxFuture;
```

## Module `either`

Module defining an Either type.

```rust
pub mod either { /* ... */ }
```

### Types

#### Enum `Either`

**Attributes:**

- `#[allow(missing_docs)]`

Combines two different futures, streams, or sinks having the same associated types into a single type.

This type implements common asynchronous traits such as [`Future`] and those in Tokio.

[`Future`]: std::future::Future

# Example

The following code will not work:

```compile_fail
# fn some_condition() -> bool { true }
# async fn some_async_function() -> u32 { 10 }
# async fn other_async_function() -> u32 { 20 }
#[tokio::main]
async fn main() {
    let result = if some_condition() {
        some_async_function()
    } else {
        other_async_function() // <- Will print: "`if` and `else` have incompatible types"
    };

    println!("Result is {}", result.await);
}
```


When the output type is the same, we can wrap each future in `Either` to avoid the
issue:

```
use tokio_util::either::Either;
# fn some_condition() -> bool { true }
# async fn some_async_function() -> u32 { 10 }
# async fn other_async_function() -> u32 { 20 }

#[tokio::main]
async fn main() {
    let result = if some_condition() {
        Either::Left(some_async_function())
    } else {
        Either::Right(other_async_function())
    };

    let value = result.await;
    println!("Result is {}", value);
    # assert_eq!(value, 10);
}
```

```rust
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
```

##### Variants

###### `Left`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `L` |  |

###### `Right`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `R` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **AsyncSeekExt**
- **Sink**
  - ```rust
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<std::result::Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn start_send(self: Pin<&mut Self>, item: Item) -> std::result::Result<(), <Self as >::Error> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<std::result::Result<(), <Self as >::Error>> { /* ... */ }
    ```

  - ```rust
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<std::result::Result<(), <Self as >::Error>> { /* ... */ }
    ```

- **AsyncWriteExt**
- **Unpin**
- **AsyncBufRead**
  - ```rust
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<&[u8]>> { /* ... */ }
    ```

  - ```rust
    fn consume(self: Pin<&mut Self>, amt: usize) { /* ... */ }
    ```

- **AsyncSeek**
  - ```rust
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Result<u64>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **Send**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Either<L, R> { /* ... */ }
    ```

- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut ReadBuf<''_>) -> Poll<Result<()>> { /* ... */ }
    ```

- **AsyncReadExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<tokio::io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<tokio::io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[std::io::IoSlice<''_>]) -> Poll<std::result::Result<usize, std::io::Error>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

- **Sync**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **AsyncBufReadExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Re-exports

### Re-export `bytes`

```rust
pub use bytes;
```

