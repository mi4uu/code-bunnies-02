# Crate Documentation

**Version:** 1.44.2

**Format Version:** 43

# Module `tokio`

A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous
applications with the Rust programming language. At a high level, it
provides a few major components:

* Tools for [working with asynchronous tasks][tasks], including
  [synchronization primitives and channels][sync] and [timeouts, sleeps, and
  intervals][time].
* APIs for [performing asynchronous I/O][io], including [TCP and UDP][net] sockets,
  [filesystem][fs] operations, and [process] and [signal] management.
* A [runtime] for executing asynchronous code, including a task scheduler,
  an I/O driver backed by the operating system's event queue (`epoll`, `kqueue`,
  `IOCP`, etc...), and a high performance timer.

Guide level documentation is found on the [website].

[tasks]: #working-with-tasks
[sync]: crate::sync
[time]: crate::time
[io]: #asynchronous-io
[net]: crate::net
[fs]: crate::fs
[process]: crate::process
[signal]: crate::signal
[fs]: crate::fs
[runtime]: crate::runtime
[website]: https://tokio.rs/tokio/tutorial

# A Tour of Tokio

Tokio consists of a number of modules that provide a range of functionality
essential for implementing asynchronous applications in Rust. In this
section, we will take a brief tour of Tokio, summarizing the major APIs and
their uses.

The easiest way to get started is to enable all features. Do this by
enabling the `full` feature flag:

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring applications

Tokio is great for writing applications and most users in this case shouldn't
worry too much about what features they should pick. If you're unsure, we suggest
going with `full` to ensure that you don't run into any road blocks while you're
building your application.

#### Example

This example shows the quickest way to get started with Tokio.

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring libraries

As a library author your goal should be to provide the lightest weight crate
that is based on Tokio. To achieve this you should ensure that you only enable
the features you need. This allows users to pick up your crate without having
to enable unnecessary features.

#### Example

This example shows how you may want to import features for a library that just
needs to `tokio::spawn` and use a `TcpStream`.

```toml
tokio = { version = "1", features = ["rt", "net"] }
```

## Working With Tasks

Asynchronous programs in Rust are based around lightweight, non-blocking
units of execution called [_tasks_][tasks]. The [`tokio::task`] module provides
important tools for working with tasks:

* The [`spawn`] function and [`JoinHandle`] type, for scheduling a new task
  on the Tokio runtime and awaiting the output of a spawned task, respectively,
* Functions for [running blocking operations][blocking] in an asynchronous
  task context.

The [`tokio::task`] module is present only when the "rt" feature flag
is enabled.

[tasks]: task/index.html#what-are-tasks
[`tokio::task`]: crate::task
[`spawn`]: crate::task::spawn()
[`JoinHandle`]: crate::task::JoinHandle
[blocking]: task/index.html#blocking-and-yielding

The [`tokio::sync`] module contains synchronization primitives to use when
needing to communicate or share data. These include:

* channels ([`oneshot`], [`mpsc`], [`watch`], and [`broadcast`]), for sending values
  between tasks,
* a non-blocking [`Mutex`], for controlling access to a shared, mutable
  value,
* an asynchronous [`Barrier`] type, for multiple tasks to synchronize before
  beginning a computation.

The `tokio::sync` module is present only when the "sync" feature flag is
enabled.

[`tokio::sync`]: crate::sync
[`Mutex`]: crate::sync::Mutex
[`Barrier`]: crate::sync::Barrier
[`oneshot`]: crate::sync::oneshot
[`mpsc`]: crate::sync::mpsc
[`watch`]: crate::sync::watch
[`broadcast`]: crate::sync::broadcast

The [`tokio::time`] module provides utilities for tracking time and
scheduling work. This includes functions for setting [timeouts][timeout] for
tasks, [sleeping][sleep] work to run in the future, or [repeating an operation at an
interval][interval].

In order to use `tokio::time`, the "time" feature flag must be enabled.

[`tokio::time`]: crate::time
[sleep]: crate::time::sleep()
[interval]: crate::time::interval()
[timeout]: crate::time::timeout()

Finally, Tokio provides a _runtime_ for executing asynchronous tasks. Most
applications can use the [`#[tokio::main]`][main] macro to run their code on the
Tokio runtime. However, this macro provides only basic configuration options. As
an alternative, the [`tokio::runtime`] module provides more powerful APIs for configuring
and managing runtimes. You should use that module if the `#[tokio::main]` macro doesn't
provide the functionality you need.

Using the runtime requires the "rt" or "rt-multi-thread" feature flags, to
enable the current-thread [single-threaded scheduler][rt] and the [multi-thread
scheduler][rt-multi-thread], respectively. See the [`runtime` module
documentation][rt-features] for details. In addition, the "macros" feature
flag enables the `#[tokio::main]` and `#[tokio::test]` attributes.

[main]: attr.main.html
[`tokio::runtime`]: crate::runtime
[`Builder`]: crate::runtime::Builder
[`Runtime`]: crate::runtime::Runtime
[rt]: runtime/index.html#current-thread-scheduler
[rt-multi-thread]: runtime/index.html#multi-thread-scheduler
[rt-features]: runtime/index.html#runtime-scheduler

## CPU-bound tasks and blocking code

Tokio is able to concurrently run many tasks on a few threads by repeatedly
swapping the currently running task on each thread. However, this kind of
swapping can only happen at `.await` points, so code that spends a long time
without reaching an `.await` will prevent other tasks from running. To
combat this, Tokio provides two kinds of threads: Core threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default
spawn one for each CPU core. You can use the environment variable `TOKIO_WORKER_THREADS`
to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code
that would otherwise block other tasks from running and are kept alive when
not used for a certain amount of time which can be configured with [`thread_keep_alive`].
Since it is not possible for Tokio to swap out blocking tasks, like it
can do with asynchronous code, the upper limit on the number of blocking
threads is very large. These limits can be configured on the [`Builder`].

To spawn a blocking task, you should use the [`spawn_blocking`] function.

[`Builder`]: crate::runtime::Builder
[`spawn_blocking`]: crate::task::spawn_blocking()
[`thread_keep_alive`]: crate::runtime::Builder::thread_keep_alive()

```
#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
```

If your code is CPU-bound and you wish to limit the number of threads used
to run it, you should use a separate thread pool dedicated to CPU bound tasks.
For example, you could consider using the [rayon] library for CPU-bound
tasks. It is also possible to create an extra Tokio runtime dedicated to
CPU-bound tasks, but if you do this, you should be careful that the extra
runtime runs _only_ CPU-bound tasks, as IO-bound tasks on that runtime
will behave poorly.

Hint: If using rayon, you can use a [`oneshot`] channel to send the result back
to Tokio when the rayon task finishes.

[rayon]: https://docs.rs/rayon
[`oneshot`]: crate::sync::oneshot

## Asynchronous IO

As well as scheduling and running tasks, Tokio provides everything you need
to perform input and output asynchronously.

The [`tokio::io`] module provides Tokio's asynchronous core I/O primitives,
the [`AsyncRead`], [`AsyncWrite`], and [`AsyncBufRead`] traits. In addition,
when the "io-util" feature flag is enabled, it also provides combinators and
functions for working with these traits, forming as an asynchronous
counterpart to [`std::io`].

Tokio also includes APIs for performing various kinds of I/O and interacting
with the operating system asynchronously. These include:

* [`tokio::net`], which contains non-blocking versions of [TCP], [UDP], and
  [Unix Domain Sockets][UDS] (enabled by the "net" feature flag),
* [`tokio::fs`], similar to [`std::fs`] but for performing filesystem I/O
  asynchronously (enabled by the "fs" feature flag),
* [`tokio::signal`], for asynchronously handling Unix and Windows OS signals
  (enabled by the "signal" feature flag),
* [`tokio::process`], for spawning and managing child processes (enabled by
  the "process" feature flag).

[`tokio::io`]: crate::io
[`AsyncRead`]: crate::io::AsyncRead
[`AsyncWrite`]: crate::io::AsyncWrite
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`std::io`]: std::io
[`tokio::net`]: crate::net
[TCP]: crate::net::tcp
[UDP]: crate::net::UdpSocket
[UDS]: crate::net::unix
[`tokio::fs`]: crate::fs
[`std::fs`]: std::fs
[`tokio::signal`]: crate::signal
[`tokio::process`]: crate::process

# Examples

A simple TCP echo server:

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

# Feature flags

Tokio uses a set of [feature flags] to reduce the amount of compiled code. It
is possible to just enable certain features over others. By default, Tokio
does not enable any features but allows one to enable a subset for their use
case. Below is a list of the available feature flags. You may also notice
above each function, struct and trait there is listed one or more feature flags
that are required for that item to be used. If you are new to Tokio it is
recommended that you use the `full` feature flag which will enable all public APIs.
Beware though that this will pull in many extra dependencies that you may not
need.

- `full`: Enables all features listed below except `test-util` and `tracing`.
- `rt`: Enables `tokio::spawn`, the current-thread scheduler,
        and non-scheduler utilities.
- `rt-multi-thread`: Enables the heavier, multi-threaded, work-stealing scheduler.
- `io-util`: Enables the IO based `Ext` traits.
- `io-std`: Enable `Stdout`, `Stdin` and `Stderr` types.
- `net`: Enables `tokio::net` types such as `TcpStream`, `UnixStream` and
         `UdpSocket`, as well as (on Unix-like systems) `AsyncFd` and (on
         FreeBSD) `PollAio`.
- `time`: Enables `tokio::time` types and allows the schedulers to enable
          the built in timer.
- `process`: Enables `tokio::process` types.
- `macros`: Enables `#[tokio::main]` and `#[tokio::test]` macros.
- `sync`: Enables all `tokio::sync` types.
- `signal`: Enables all `tokio::signal` types.
- `fs`: Enables `tokio::fs` types.
- `test-util`: Enables testing based infrastructure for the Tokio runtime.
- `parking_lot`: As a potential optimization, use the `_parking_lot_` crate's
                 synchronization primitives internally. Also, this
                 dependency is necessary to construct some of our primitives
                 in a `const` context. `MSRV` may increase according to the
                 `_parking_lot_` release in use.

_Note: `AsyncRead` and `AsyncWrite` traits do not require any features and are
always available._

## Unstable features

Some feature flags are only available when specifying the `tokio_unstable` flag:

- `tracing`: Enables tracing events.

Likewise, some parts of the API are only available with the same flag:

- [`task::Builder`]
- Some methods on [`task::JoinSet`]
- [`runtime::RuntimeMetrics`]
- [`runtime::Builder::on_task_spawn`]
- [`runtime::Builder::on_task_terminate`]
- [`runtime::Builder::unhandled_panic`]
- [`runtime::TaskMeta`]

This flag enables **unstable** features. The public API of these features
may break in 1.x releases. To enable these features, the `--cfg
tokio_unstable` argument must be passed to `rustc` when compiling. This
serves to explicitly opt-in to features which may break semver conventions,
since Cargo [does not yet directly support such opt-ins][unstable features].

You can specify it in your project's `.cargo/config.toml` file:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

<div class="warning">
The <code>[build]</code> section does <strong>not</strong> go in a
<code>Cargo.toml</code> file. Instead it must be placed in the Cargo config
file <code>.cargo/config.toml</code>.
</div>

Alternatively, you can specify it with an environment variable:

```sh
## Many *nix shells:
export RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

```powershell
## Windows PowerShell:
$Env:RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

[unstable features]: https://internals.rust-lang.org/t/feature-request-unstable-opt-in-non-transitive-crate-features/16193#why-not-a-crate-feature-2
[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

# Supported platforms

Tokio currently guarantees support for the following platforms:

 * Linux
 * Windows
 * Android (API level 21)
 * macOS
 * iOS
 * FreeBSD

Tokio will continue to support these platforms in the future. However,
future releases may change requirements such as the minimum required libc
version on Linux, the API level on Android, or the supported FreeBSD
release.

Beyond the above platforms, Tokio is intended to work on all platforms
supported by the mio crate. You can find a longer list [in mio's
documentation][mio-supported]. However, these additional platforms may
become unsupported in the future.

Note that Wine is considered to be a different platform from Windows. See
mio's documentation for more information on Wine support.

[mio-supported]: https://crates.io/crates/mio#platforms

## `WASM` support

Tokio has some limited support for the `WASM` platform. Without the
`tokio_unstable` flag, the following features are supported:

 * `sync`
 * `macros`
 * `io-util`
 * `rt`
 * `time`

Enabling any other feature (including `full`) will cause a compilation
failure.

The `time` module will only work on `WASM` platforms that have support for
timers (e.g. wasm32-wasi). The timing functions will panic if used on a `WASM`
platform that does not support timers.

Note also that if the runtime becomes indefinitely idle, it will panic
immediately instead of blocking forever. On platforms that don't support
time, this means that the runtime can never be idle in any way.

## Unstable `WASM` support

Tokio also has unstable support for some additional `WASM` features. This
requires the use of the `tokio_unstable` flag.

Using this flag enables the use of `tokio::net` on the wasm32-wasi target.
However, not all methods are available on the networking types as `WASI`
currently does not support the creation of new sockets from within `WASM`.
Because of this, sockets must currently be created via the `FromRawFd`
trait.

## Modules

## Module `fs`

**Attributes:**

- `#[<cfg>(feature = "fs")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "fs")))]`
- `#[<cfg>(not(loom))]`

Asynchronous file utilities.

This module contains utility methods for working with the file system
asynchronously. This includes reading/writing to files, and working with
directories.

Be aware that most operating systems do not provide asynchronous file system
APIs. Because of that, Tokio will use ordinary blocking file operations
behind the scenes. This is done using the [`spawn_blocking`] threadpool to
run them in the background.

The `tokio::fs` module should only be used for ordinary files. Trying to use
it with e.g., a named pipe on Linux can result in surprising behavior,
such as hangs during runtime shutdown. For special files, you should use a
dedicated type such as [`tokio::net::unix::pipe`] or [`AsyncFd`] instead.

Currently, Tokio will always use [`spawn_blocking`] on all platforms, but it
may be changed to use asynchronous file system APIs such as io_uring in the
future.

# Usage

The easiest way to use this module is to use the utility functions that
operate on entire files:

 * [`tokio::fs::read`](fn@crate::fs::read)
 * [`tokio::fs::read_to_string`](fn@crate::fs::read_to_string)
 * [`tokio::fs::write`](fn@crate::fs::write)

The two `read` functions reads the entire file and returns its contents.
The `write` function takes the contents of the file and writes those
contents to the file. It overwrites the existing file, if any.

For example, to read the file:

```
# async fn dox() -> std::io::Result<()> {
let contents = tokio::fs::read_to_string("my_file.txt").await?;

println!("File has {} lines.", contents.lines().count());
# Ok(())
# }
```

To overwrite the file:

```
# async fn dox() -> std::io::Result<()> {
let contents = "First line.\nSecond line.\nThird line.\n";

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
# Ok(())
# }
```

## Using `File`

The main type for interacting with files is [`File`]. It can be used to read
from and write to a given file. This is done using the [`AsyncRead`] and
[`AsyncWrite`] traits. This type is generally used when you want to do
something more complex than just reading or writing the entire contents in
one go.

**Note:** It is important to use [`flush`] when writing to a Tokio
[`File`]. This is because calls to `write` will return before the write has
finished, and [`flush`] will wait for the write to finish. (The write will
happen even if you don't flush; it will just happen later.) This is
different from [`std::fs::File`], and is due to the fact that `File` uses
`spawn_blocking` behind the scenes.

For example, to count the number of lines in a file without loading the
entire file into memory:

```no_run
use tokio::fs::File;
use tokio::io::AsyncReadExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::open("my_file.txt").await?;

let mut chunk = vec![0; 4096];
let mut number_of_lines = 0;
loop {
    let len = file.read(&mut chunk).await?;
    if len == 0 {
        // Length of zero means end of file.
        break;
    }
    for &b in &chunk[..len] {
        if b == b'\n' {
            number_of_lines += 1;
        }
    }
}

println!("File has {} lines.", number_of_lines);
# Ok(())
# }
```

For example, to write a file line-by-line:

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::create("my_file.txt").await?;

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Remember to call `flush` after writing!
file.flush().await?;
# Ok(())
# }
```

## Tuning your file IO

Tokio's file uses [`spawn_blocking`] behind the scenes, and this has serious
performance consequences. To get good performance with file IO on Tokio, it
is recommended to batch your operations into as few `spawn_blocking` calls
as possible.

One example of this difference can be seen by comparing the two reading
examples above. The first example uses [`tokio::fs::read`], which reads the
entire file in a single `spawn_blocking` call, and then returns it. The
second example will read the file in chunks using many `spawn_blocking`
calls. This means that the second example will most likely be more expensive
for large files. (Of course, using chunks may be necessary for very large
files that don't fit in memory.)

The following examples will show some strategies for this:

When creating a file, write the data to a `String` or `Vec<u8>` and then
write the entire file in a single `spawn_blocking` call with
`tokio::fs::write`.

```no_run
# async fn dox() -> std::io::Result<()> {
let mut contents = String::new();

contents.push_str("First line.\n");
contents.push_str("Second line.\n");
contents.push_str("Third line.\n");

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
# Ok(())
# }
```

Use [`BufReader`] and [`BufWriter`] to buffer many small reads or writes
into a few large ones. This example will most likely only perform one
`spawn_blocking` call.

```no_run
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

# async fn dox() -> std::io::Result<()> {
let mut file = BufWriter::new(File::create("my_file.txt").await?);

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Due to the BufWriter, the actual write and spawn_blocking
// call happens when you flush.
file.flush().await?;
# Ok(())
# }
```

Manually use [`std::fs`] inside [`spawn_blocking`].

```no_run
use std::fs::File;
use std::io::{self, Write};
use tokio::task::spawn_blocking;

# async fn dox() -> std::io::Result<()> {
spawn_blocking(move || {
    let mut file = File::create("my_file.txt")?;

    file.write_all(b"First line.\n")?;
    file.write_all(b"Second line.\n")?;
    file.write_all(b"Third line.\n")?;

    // Unlike Tokio's file, the std::fs file does
    // not need flush.

    io::Result::Ok(())
}).await.unwrap()?;
# Ok(())
# }
```

It's also good to be aware of [`File::set_max_buf_size`], which controls the
maximum amount of bytes that Tokio's [`File`] will read or write in a single
[`spawn_blocking`] call. The default is two megabytes, but this is subject
to change.

[`spawn_blocking`]: fn@crate::task::spawn_blocking
[`AsyncRead`]: trait@crate::io::AsyncRead
[`AsyncWrite`]: trait@crate::io::AsyncWrite
[`BufReader`]: struct@crate::io::BufReader
[`BufWriter`]: struct@crate::io::BufWriter
[`tokio::net::unix::pipe`]: crate::net::unix::pipe
[`AsyncFd`]: crate::io::unix::AsyncFd
[`flush`]: crate::io::AsyncWriteExt::flush
[`tokio::fs::read`]: fn@crate::fs::read

```rust
pub mod fs { /* ... */ }
```

### Re-exports

#### Re-export `canonicalize`

```rust
pub use self::canonicalize::canonicalize;
```

#### Re-export `create_dir`

```rust
pub use self::create_dir::create_dir;
```

#### Re-export `create_dir_all`

```rust
pub use self::create_dir_all::create_dir_all;
```

#### Re-export `DirBuilder`

```rust
pub use self::dir_builder::DirBuilder;
```

#### Re-export `File`

```rust
pub use self::file::File;
```

#### Re-export `hard_link`

```rust
pub use self::hard_link::hard_link;
```

#### Re-export `metadata`

```rust
pub use self::metadata::metadata;
```

#### Re-export `OpenOptions`

```rust
pub use self::open_options::OpenOptions;
```

#### Re-export `read`

```rust
pub use self::read::read;
```

#### Re-export `read_dir`

```rust
pub use self::read_dir::read_dir;
```

#### Re-export `DirEntry`

```rust
pub use self::read_dir::DirEntry;
```

#### Re-export `ReadDir`

```rust
pub use self::read_dir::ReadDir;
```

#### Re-export `read_link`

```rust
pub use self::read_link::read_link;
```

#### Re-export `read_to_string`

```rust
pub use self::read_to_string::read_to_string;
```

#### Re-export `remove_dir`

```rust
pub use self::remove_dir::remove_dir;
```

#### Re-export `remove_dir_all`

```rust
pub use self::remove_dir_all::remove_dir_all;
```

#### Re-export `remove_file`

```rust
pub use self::remove_file::remove_file;
```

#### Re-export `rename`

```rust
pub use self::rename::rename;
```

#### Re-export `set_permissions`

```rust
pub use self::set_permissions::set_permissions;
```

#### Re-export `symlink_metadata`

```rust
pub use self::symlink_metadata::symlink_metadata;
```

#### Re-export `write`

```rust
pub use self::write::write;
```

#### Re-export `copy`

```rust
pub use self::copy::copy;
```

#### Re-export `try_exists`

```rust
pub use self::try_exists::try_exists;
```

#### Re-export `symlink`

**Attributes:**

- `#[<cfg>(unix)]`
- `#[<cfg_attr>(docsrs, doc(cfg(unix)))]`

```rust
pub use self::symlink::symlink;
```

## Module `io`

**Attributes:**

- `#[<cfg_attr>(not(all(feature = "rt", feature = "net")),
allow(dead_code, unused_imports))]`

Traits, helpers, and type definitions for asynchronous I/O functionality.

This module is the asynchronous version of `std::io`. Primarily, it
defines two traits, [`AsyncRead`] and [`AsyncWrite`], which are asynchronous
versions of the [`Read`] and [`Write`] traits in the standard library.

# `AsyncRead` and `AsyncWrite`

Like the standard library's [`Read`] and [`Write`] traits, [`AsyncRead`] and
[`AsyncWrite`] provide the most general interface for reading and writing
input and output. Unlike the standard library's traits, however, they are
_asynchronous_ &mdash; meaning that reading from or writing to a `tokio::io`
type will _yield_ to the Tokio scheduler when IO is not ready, rather than
blocking. This allows other tasks to run while waiting on IO.

Another difference is that `AsyncRead` and `AsyncWrite` only contain
core methods needed to provide asynchronous reading and writing
functionality. Instead, utility methods are defined in the [`AsyncReadExt`]
and [`AsyncWriteExt`] extension traits. These traits are automatically
implemented for all values that implement `AsyncRead` and `AsyncWrite`
respectively.

End users will rarely interact directly with `AsyncRead` and
`AsyncWrite`. Instead, they will use the async functions defined in the
extension traits. Library authors are expected to implement `AsyncRead`
and `AsyncWrite` in order to provide types that behave like byte streams.

Even with these differences, Tokio's `AsyncRead` and `AsyncWrite` traits
can be used in almost exactly the same manner as the standard library's
`Read` and `Write`. Most types in the standard library that implement `Read`
and `Write` have asynchronous equivalents in `tokio` that implement
`AsyncRead` and `AsyncWrite`, such as [`File`] and [`TcpStream`].

For example, the standard library documentation introduces `Read` by
[demonstrating][std_example] reading some bytes from a [`std::fs::File`]. We
can do the same with [`tokio::fs::File`][`File`]:

```no_run
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

[`File`]: crate::fs::File
[`TcpStream`]: crate::net::TcpStream
[`std::fs::File`]: std::fs::File
[std_example]: std::io#read-and-write

## Buffered Readers and Writers

Byte-based interfaces are unwieldy and can be inefficient, as we'd need to be
making near-constant calls to the operating system. To help with this,
`std::io` comes with [support for _buffered_ readers and writers][stdbuf],
and therefore, `tokio::io` does as well.

Tokio provides an async version of the [`std::io::BufRead`] trait,
[`AsyncBufRead`]; and async [`BufReader`] and [`BufWriter`] structs, which
wrap readers and writers. These wrappers use a buffer, reducing the number
of calls and providing nicer methods for accessing exactly what you want.

For example, [`BufReader`] works with the [`AsyncBufRead`] trait to add
extra methods to any async reader:

```no_run
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::open("foo.txt").await?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer).await?;

    println!("{}", buffer);
    Ok(())
}
```

[`BufWriter`] doesn't add any new ways of writing; it just buffers every call
to [`write`](crate::io::AsyncWriteExt::write). However, you **must** flush
[`BufWriter`] to ensure that any buffered data is written.

```no_run
use tokio::io::{self, BufWriter, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::create("foo.txt").await?;
    {
        let mut writer = BufWriter::new(f);

        // Write a byte to the buffer.
        writer.write(&[42u8]).await?;

        // Flush the buffer before it goes out of scope.
        writer.flush().await?;

    } // Unless flushed or shut down, the contents of the buffer is discarded on drop.

    Ok(())
}
```

[stdbuf]: std::io#bufreader-and-bufwriter
[`std::io::BufRead`]: std::io::BufRead
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`BufReader`]: crate::io::BufReader
[`BufWriter`]: crate::io::BufWriter

## Implementing `AsyncRead` and `AsyncWrite`

Because they are traits, we can implement [`AsyncRead`] and [`AsyncWrite`] for
our own types, as well. Note that these traits must only be implemented for
non-blocking I/O types that integrate with the futures type system. In
other words, these types must never block the thread, and instead the
current task is notified when the I/O resource is ready.

## Conversion to and from Stream/Sink

It is often convenient to encapsulate the reading and writing of bytes in a
[`Stream`] or [`Sink`] of data.

Tokio provides simple wrappers for converting [`AsyncRead`] to [`Stream`]
and vice-versa in the [tokio-util] crate, see [`ReaderStream`] and
[`StreamReader`].

There are also utility traits that abstract the asynchronous buffering
necessary to write your own adaptors for encoding and decoding bytes to/from
your structured data, allowing to transform something that implements
[`AsyncRead`]/[`AsyncWrite`] into a [`Stream`]/[`Sink`], see [`Decoder`] and
[`Encoder`] in the [tokio-util::codec] module.

[tokio-util]: https://docs.rs/tokio-util
[tokio-util::codec]: https://docs.rs/tokio-util/latest/tokio_util/codec/index.html

# Standard input and output

Tokio provides asynchronous APIs to standard [input], [output], and [error].
These APIs are very similar to the ones provided by `std`, but they also
implement [`AsyncRead`] and [`AsyncWrite`].

Note that the standard input / output APIs  **must** be used from the
context of the Tokio runtime, as they require Tokio-specific features to
function. Calling these functions outside of a Tokio runtime will panic.

[input]: fn@stdin
[output]: fn@stdout
[error]: fn@stderr

# `std` re-exports

Additionally, [`Error`], [`ErrorKind`], [`Result`], and [`SeekFrom`] are
re-exported from `std::io` for ease of use.

[`AsyncRead`]: trait@AsyncRead
[`AsyncWrite`]: trait@AsyncWrite
[`AsyncReadExt`]: trait@AsyncReadExt
[`AsyncWriteExt`]: trait@AsyncWriteExt
["codec"]: https://docs.rs/tokio-util/latest/tokio_util/codec/index.html
[`Encoder`]: https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html
[`Decoder`]: https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html
[`ReaderStream`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
[`StreamReader`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html
[`Error`]: struct@Error
[`ErrorKind`]: enum@ErrorKind
[`Result`]: type@Result
[`Read`]: std::io::Read
[`SeekFrom`]: enum@SeekFrom
[`Sink`]: https://docs.rs/futures/0.3/futures/sink/trait.Sink.html
[`Stream`]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html
[`Write`]: std::io::Write

```rust
pub mod io { /* ... */ }
```

### Modules

## Module `unix`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

Asynchronous IO structures specific to Unix-like operating systems.

```rust
pub mod unix { /* ... */ }
```

### Re-exports

#### Re-export `AsyncFd`

```rust
pub use super::async_fd::AsyncFd;
```

#### Re-export `AsyncFdTryNewError`

```rust
pub use super::async_fd::AsyncFdTryNewError;
```

#### Re-export `AsyncFdReadyGuard`

```rust
pub use super::async_fd::AsyncFdReadyGuard;
```

#### Re-export `AsyncFdReadyMutGuard`

```rust
pub use super::async_fd::AsyncFdReadyMutGuard;
```

#### Re-export `TryIoError`

```rust
pub use super::async_fd::TryIoError;
```

### Re-exports

#### Re-export `AsyncBufRead`

```rust
pub use self::async_buf_read::AsyncBufRead;
```

#### Re-export `AsyncRead`

```rust
pub use self::async_read::AsyncRead;
```

#### Re-export `AsyncSeek`

```rust
pub use self::async_seek::AsyncSeek;
```

#### Re-export `AsyncWrite`

```rust
pub use self::async_write::AsyncWrite;
```

#### Re-export `ReadBuf`

```rust
pub use self::read_buf::ReadBuf;
```

#### Re-export `Error`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Error;
```

#### Re-export `ErrorKind`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::ErrorKind;
```

#### Re-export `Result`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Result;
```

#### Re-export `SeekFrom`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::SeekFrom;
```

#### Re-export `Interest`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

```rust
pub use interest::Interest;
```

#### Re-export `Ready`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

```rust
pub use ready::Ready;
```

#### Re-export `stderr`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stderr::stderr;
```

#### Re-export `Stderr`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stderr::Stderr;
```

#### Re-export `stdin`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stdin::stdin;
```

#### Re-export `Stdin`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stdin::Stdin;
```

#### Re-export `stdout`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stdout::stdout;
```

#### Re-export `Stdout`

**Attributes:**

- `#[<cfg>(feature = "io-std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-std")))]`

```rust
pub use stdout::Stdout;
```

#### Re-export `split`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use split::split;
```

#### Re-export `ReadHalf`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use split::ReadHalf;
```

#### Re-export `WriteHalf`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use split::WriteHalf;
```

#### Re-export `join`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use join::join;
```

#### Re-export `Join`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use join::Join;
```

#### Re-export `copy`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::copy;
```

#### Re-export `copy_bidirectional`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::copy_bidirectional;
```

#### Re-export `copy_bidirectional_with_sizes`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::copy_bidirectional_with_sizes;
```

#### Re-export `copy_buf`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::copy_buf;
```

#### Re-export `duplex`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::duplex;
```

#### Re-export `empty`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::empty;
```

#### Re-export `repeat`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::repeat;
```

#### Re-export `sink`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::sink;
```

#### Re-export `simplex`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::simplex;
```

#### Re-export `AsyncBufReadExt`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::AsyncBufReadExt;
```

#### Re-export `AsyncReadExt`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::AsyncReadExt;
```

#### Re-export `AsyncSeekExt`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::AsyncSeekExt;
```

#### Re-export `AsyncWriteExt`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::AsyncWriteExt;
```

#### Re-export `BufReader`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::BufReader;
```

#### Re-export `BufStream`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::BufStream;
```

#### Re-export `BufWriter`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::BufWriter;
```

#### Re-export `DuplexStream`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::DuplexStream;
```

#### Re-export `Empty`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Empty;
```

#### Re-export `Lines`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Lines;
```

#### Re-export `Repeat`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Repeat;
```

#### Re-export `Sink`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Sink;
```

#### Re-export `Split`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Split;
```

#### Re-export `Take`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::Take;
```

#### Re-export `SimplexStream`

**Attributes:**

- `#[<cfg>(feature = "io-util")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "io-util")))]`

```rust
pub use util::SimplexStream;
```

## Module `net`

**Attributes:**

- `#[<cfg>(not(loom))]`

TCP/UDP/Unix bindings for `tokio`.

This module contains the TCP/UDP/Unix networking types, similar to the standard
library, which can be used to implement networking protocols.

# Organization

* [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
* [`UdpSocket`] provides functionality for communication over UDP
* [`UnixListener`] and [`UnixStream`] provide functionality for communication over a
Unix Domain Stream Socket **(available on Unix only)**
* [`UnixDatagram`] provides functionality for communication
over Unix Domain Datagram Socket **(available on Unix only)**
* [`tokio::net::unix::pipe`] for FIFO pipes **(available on Unix only)**
* [`tokio::net::windows::named_pipe`] for Named Pipes **(available on Windows only)**

For IO resources not available in `tokio::net`, you can use [`AsyncFd`].

[`TcpListener`]: TcpListener
[`TcpStream`]: TcpStream
[`UdpSocket`]: UdpSocket
[`UnixListener`]: UnixListener
[`UnixStream`]: UnixStream
[`UnixDatagram`]: UnixDatagram
[`tokio::net::unix::pipe`]: unix::pipe
[`tokio::net::windows::named_pipe`]: windows::named_pipe
[`AsyncFd`]: crate::io::unix::AsyncFd

```rust
pub mod net { /* ... */ }
```

### Modules

## Module `tcp`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

TCP utility types.

```rust
pub mod tcp { /* ... */ }
```

### Re-exports

#### Re-export `ReadHalf`

```rust
pub use split::ReadHalf;
```

#### Re-export `WriteHalf`

```rust
pub use split::WriteHalf;
```

#### Re-export `OwnedReadHalf`

```rust
pub use split_owned::OwnedReadHalf;
```

#### Re-export `OwnedWriteHalf`

```rust
pub use split_owned::OwnedWriteHalf;
```

#### Re-export `ReuniteError`

```rust
pub use split_owned::ReuniteError;
```

## Module `unix`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

Unix specific network types.

```rust
pub mod unix { /* ... */ }
```

### Modules

## Module `pipe`

Unix pipe types.

```rust
pub mod pipe { /* ... */ }
```

### Types

#### Struct `OpenOptions`

Options and flags which can be used to configure how a FIFO file is opened.

This builder allows configuring how to create a pipe end from a FIFO file.
Generally speaking, when using `OpenOptions`, you'll first call [`new`],
then chain calls to methods to set each option, then call either
[`open_receiver`] or [`open_sender`], passing the path of the FIFO file you
are trying to open. This will give you a [`io::Result`] with a pipe end
inside that you can further operate on.

[`new`]: OpenOptions::new
[`open_receiver`]: OpenOptions::open_receiver
[`open_sender`]: OpenOptions::open_sender

# Examples

Opening a pair of pipe ends from a FIFO file:

```no_run
use tokio::net::unix::pipe;
# use std::error::Error;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
let rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
let tx = pipe::OpenOptions::new().open_sender(FIFO_NAME)?;
# Ok(())
# }
```

Opening a [`Sender`] on Linux when you are sure the file is a FIFO:

```ignore
use tokio::net::unix::pipe;
use nix::{unistd::mkfifo, sys::stat::Mode};
# use std::error::Error;

// Our program has exclusive access to this path.
const FIFO_NAME: &str = "path/to/a/new/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
mkfifo(FIFO_NAME, Mode::S_IRWXU)?;
let tx = pipe::OpenOptions::new()
    .read_write(true)
    .unchecked(true)
    .open_sender(FIFO_NAME)?;
# Ok(())
# }
```

```rust
pub struct OpenOptions {
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
  pub fn new() -> OpenOptions { /* ... */ }
  ```
  Creates a blank new set of options ready for configuration.

- ```rust
  pub fn read_write(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets the option for read-write access.

- ```rust
  pub fn unchecked(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```
  Sets the option to skip the check for FIFO file type.

- ```rust
  pub fn open_receiver<P: AsRef<Path>>(self: &Self, path: P) -> io::Result<Receiver> { /* ... */ }
  ```
  Creates a [`Receiver`] from a FIFO file with the options specified by `self`.

- ```rust
  pub fn open_sender<P: AsRef<Path>>(self: &Self, path: P) -> io::Result<Sender> { /* ... */ }
  ```
  Creates a [`Sender`] from a FIFO file with the options specified by `self`.

###### Trait Implementations

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> OpenOptions { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OpenOptions { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Sender`

Writing end of a Unix pipe.

It can be constructed from a FIFO file with [`OpenOptions::open_sender`].

Opening a named pipe for writing involves a few steps.
Call to [`OpenOptions::open_sender`] might fail with an error indicating
different things:

* [`io::ErrorKind::NotFound`] - There is no file at the specified path.
* [`io::ErrorKind::InvalidInput`] - The file exists, but it is not a FIFO.
* [`ENXIO`] - The file is a FIFO, but no process has it open for reading.
  Sleep for a while and try again.
* Other OS errors not specific to opening FIFO files.

Opening a `Sender` from a FIFO file should look like this:

```no_run
use tokio::net::unix::pipe;
use tokio::time::{self, Duration};

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
// Wait for a reader to open the file.
let tx = loop {
    match pipe::OpenOptions::new().open_sender(FIFO_NAME) {
        Ok(tx) => break tx,
        Err(e) if e.raw_os_error() == Some(libc::ENXIO) => {},
        Err(e) => return Err(e.into()),
    }

    time::sleep(Duration::from_millis(50)).await;
};
# Ok(())
# }
```

On Linux, it is possible to create a `Sender` without waiting in a sleeping
loop. This is done by opening a named pipe in read-write access mode with
`OpenOptions::read_write`. This way, a `Sender` can at the same time hold
both a writing end and a reading end, and the latter allows to open a FIFO
without [`ENXIO`] error since the pipe is open for reading as well.

`Sender` cannot be used to read from a pipe, so in practice the read access
is only used when a FIFO is opened. However, using a `Sender` in read-write
mode **may lead to lost data**, because written data will be dropped by the
system as soon as all pipe ends are closed. To avoid lost data you have to
make sure that a reading end has been opened before dropping a `Sender`.

Note that using read-write access mode with FIFO files is not defined by
the POSIX standard and it is only guaranteed to work on Linux.

```ignore
use tokio::io::AsyncWriteExt;
use tokio::net::unix::pipe;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let mut tx = pipe::OpenOptions::new()
    .read_write(true)
    .open_sender(FIFO_NAME)?;

// Asynchronously write to the pipe before a reader.
tx.write_all(b"hello world").await?;
# Ok(())
# }
```

[`ENXIO`]: https://docs.rs/libc/latest/libc/constant.ENXIO.html

```rust
pub struct Sender {
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
  pub fn from_file(file: File) -> io::Result<Sender> { /* ... */ }
  ```
  Creates a new `Sender` from a [`File`].

- ```rust
  pub fn from_owned_fd(owned_fd: OwnedFd) -> io::Result<Sender> { /* ... */ }
  ```
  Creates a new `Sender` from an [`OwnedFd`].

- ```rust
  pub fn from_file_unchecked(file: File) -> io::Result<Sender> { /* ... */ }
  ```
  Creates a new `Sender` from a [`File`] without checking pipe properties.

- ```rust
  pub fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> io::Result<Sender> { /* ... */ }
  ```
  Creates a new `Sender` from an [`OwnedFd`] without checking pipe properties.

- ```rust
  pub async fn ready(self: &Self, interest: Interest) -> io::Result<Ready> { /* ... */ }
  ```
  Waits for any of the requested ready states.

- ```rust
  pub async fn writable(self: &Self) -> io::Result<()> { /* ... */ }
  ```
  Waits for the pipe to become writable.

- ```rust
  pub fn poll_write_ready(self: &Self, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
  ```
  Polls for write readiness.

- ```rust
  pub fn try_write(self: &Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
  ```
  Tries to write a buffer to the pipe, returning how many bytes were

- ```rust
  pub fn try_write_vectored(self: &Self, buf: &[io::IoSlice<''_>]) -> io::Result<usize> { /* ... */ }
  ```
  Tries to write several buffers to the pipe, returning how many bytes

- ```rust
  pub fn into_blocking_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Converts the pipe into an [`OwnedFd`] in blocking mode.

- ```rust
  pub fn into_nonblocking_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Converts the pipe into an [`OwnedFd`] in nonblocking mode.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsFd**
  - ```rust
    fn as_fd(self: &Self) -> BorrowedFd<''_> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[io::IoSlice<''_>]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **AsyncWriteExt**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Sync**
#### Struct `Receiver`

Reading end of a Unix pipe.

It can be constructed from a FIFO file with [`OpenOptions::open_receiver`].

# Examples

Receiving messages from a named pipe in a loop:

```no_run
use tokio::net::unix::pipe;
use tokio::io::{self, AsyncReadExt};

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let mut rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
loop {
    let mut msg = vec![0; 256];
    match rx.read_exact(&mut msg).await {
        Ok(_) => {
            /* handle the message */
        }
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            // Writing end has been closed, we should reopen the pipe.
            rx = pipe::OpenOptions::new().open_receiver(FIFO_NAME)?;
        }
        Err(e) => return Err(e.into()),
    }
}
# }
```

On Linux, you can use a `Receiver` in read-write access mode to implement
resilient reading from a named pipe. Unlike `Receiver` opened in read-only
mode, read from a pipe in read-write mode will not fail with `UnexpectedEof`
when the writing end is closed. This way, a `Receiver` can asynchronously
wait for the next writer to open the pipe.

You should not use functions waiting for EOF such as [`read_to_end`] with
a `Receiver` in read-write access mode, since it **may wait forever**.
`Receiver` in this mode also holds an open writing end, which prevents
receiving EOF.

To set the read-write access mode you can use `OpenOptions::read_write`.
Note that using read-write access mode with FIFO files is not defined by
the POSIX standard and it is only guaranteed to work on Linux.

```ignore
use tokio::net::unix::pipe;
use tokio::io::AsyncReadExt;
# use std::error::Error;

const FIFO_NAME: &str = "path/to/a/fifo";

# async fn dox() -> Result<(), Box<dyn Error>> {
let mut rx = pipe::OpenOptions::new()
    .read_write(true)
    .open_receiver(FIFO_NAME)?;
loop {
    let mut msg = vec![0; 256];
    rx.read_exact(&mut msg).await?;
    /* handle the message */
}
# }
```

[`read_to_end`]: crate::io::AsyncReadExt::read_to_end

```rust
pub struct Receiver {
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
  pub fn from_file(file: File) -> io::Result<Receiver> { /* ... */ }
  ```
  Creates a new `Receiver` from a [`File`].

- ```rust
  pub fn from_owned_fd(owned_fd: OwnedFd) -> io::Result<Receiver> { /* ... */ }
  ```
  Creates a new `Receiver` from an [`OwnedFd`].

- ```rust
  pub fn from_file_unchecked(file: File) -> io::Result<Receiver> { /* ... */ }
  ```
  Creates a new `Receiver` from a [`File`] without checking pipe properties.

- ```rust
  pub fn from_owned_fd_unchecked(owned_fd: OwnedFd) -> io::Result<Receiver> { /* ... */ }
  ```
  Creates a new `Receiver` from an [`OwnedFd`] without checking pipe properties.

- ```rust
  pub async fn ready(self: &Self, interest: Interest) -> io::Result<Ready> { /* ... */ }
  ```
  Waits for any of the requested ready states.

- ```rust
  pub async fn readable(self: &Self) -> io::Result<()> { /* ... */ }
  ```
  Waits for the pipe to become readable.

- ```rust
  pub fn poll_read_ready(self: &Self, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
  ```
  Polls for read readiness.

- ```rust
  pub fn try_read(self: &Self, buf: &mut [u8]) -> io::Result<usize> { /* ... */ }
  ```
  Tries to read data from the pipe into the provided buffer, returning how

- ```rust
  pub fn try_read_vectored(self: &Self, bufs: &mut [io::IoSliceMut<''_>]) -> io::Result<usize> { /* ... */ }
  ```
  Tries to read data from the pipe into the provided buffers, returning

- ```rust
  pub fn try_read_buf<B: BufMut>(self: &Self, buf: &mut B) -> io::Result<usize> { /* ... */ }
  ```
  Tries to read data from the pipe into the provided buffer, advancing the

- ```rust
  pub fn into_blocking_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Converts the pipe into an [`OwnedFd`] in blocking mode.

- ```rust
  pub fn into_nonblocking_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Converts the pipe into an [`OwnedFd`] in nonblocking mode.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **AsFd**
  - ```rust
    fn as_fd(self: &Self) -> BorrowedFd<''_> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **AsyncReadExt**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

### Functions

#### Function `pipe`

Creates a new anonymous Unix pipe.

This function will open a new pipe and associate both pipe ends with the default
event loop.

If you need to create a pipe for communication with a spawned process, you can
use [`Stdio::piped()`] instead.

[`Stdio::piped()`]: std::process::Stdio::piped

# Errors

If creating a pipe fails, this function will return with the related OS error.

# Examples

Create a pipe and pass the writing end to a spawned process.

```no_run
use tokio::net::unix::pipe;
use tokio::process::Command;
# use tokio::io::AsyncReadExt;
# use std::error::Error;

# async fn dox() -> Result<(), Box<dyn Error>> {
let (tx, mut rx) = pipe::pipe()?;
let mut buffer = String::new();

let status = Command::new("echo")
    .arg("Hello, world!")
    .stdout(tx.into_blocking_fd()?)
    .status();
rx.read_to_string(&mut buffer).await?;

assert!(status.await?.success());
assert_eq!(buffer, "Hello, world!\n");
# Ok(())
# }
```

# Panics

This function panics if it is not called from within a runtime with
IO enabled.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

```rust
pub fn pipe() -> io::Result<(Sender, Receiver)> { /* ... */ }
```

### Types

#### Type Alias `uid_t`

**Attributes:**

- `#[allow(non_camel_case_types)]`

A type representing user ID.

```rust
pub type uid_t = u32;
```

#### Type Alias `gid_t`

**Attributes:**

- `#[allow(non_camel_case_types)]`

A type representing group ID.

```rust
pub type gid_t = u32;
```

#### Type Alias `pid_t`

**Attributes:**

- `#[allow(non_camel_case_types)]`

A type representing process and process group IDs.

```rust
pub type pid_t = i32;
```

### Re-exports

#### Re-export `ReadHalf`

```rust
pub use split::ReadHalf;
```

#### Re-export `WriteHalf`

```rust
pub use split::WriteHalf;
```

#### Re-export `OwnedReadHalf`

```rust
pub use split_owned::OwnedReadHalf;
```

#### Re-export `OwnedWriteHalf`

```rust
pub use split_owned::OwnedWriteHalf;
```

#### Re-export `ReuniteError`

```rust
pub use split_owned::ReuniteError;
```

#### Re-export `SocketAddr`

```rust
pub use socketaddr::SocketAddr;
```

#### Re-export `UCred`

```rust
pub use ucred::UCred;
```

### Re-exports

#### Re-export `ToSocketAddrs`

```rust
pub use addr::ToSocketAddrs;
```

#### Re-export `lookup_host`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

```rust
pub use lookup_host::lookup_host;
```

#### Re-export `TcpListener`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

```rust
pub use tcp::listener::TcpListener;
```

#### Re-export `TcpStream`

**Attributes:**

- `#[<cfg>(feature = "net")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "net")))]`

```rust
pub use tcp::stream::TcpStream;
```

#### Re-export `TcpSocket`

**Attributes:**

- `#[<cfg>(not(target_os = "wasi"))]`

```rust
pub use tcp::socket::TcpSocket;
```

#### Re-export `UdpSocket`

**Attributes:**

- `#[<cfg>(not(target_os = "wasi"))]`
- `#[doc(inline)]`

```rust
pub use udp::UdpSocket;
```

#### Re-export `UnixDatagram`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

```rust
pub use unix::datagram::socket::UnixDatagram;
```

#### Re-export `UnixListener`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

```rust
pub use unix::listener::UnixListener;
```

#### Re-export `UnixStream`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

```rust
pub use unix::stream::UnixStream;
```

#### Re-export `UnixSocket`

**Attributes:**

- `#[<cfg>(all(unix, feature = "net"))]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "net"))))]`

```rust
pub use unix::socket::UnixSocket;
```

## Module `process`

**Attributes:**

- `#[<cfg>(feature = "process")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "process")))]`
- `#[<cfg>(not(loom))]`
- `#[<cfg>(not(target_os = "wasi"))]`

An implementation of asynchronous process management for Tokio.

This module provides a [`Command`] struct that imitates the interface of the
[`std::process::Command`] type in the standard library, but provides asynchronous versions of
functions that create processes. These functions (`spawn`, `status`, `output` and their
variants) return "future aware" types that interoperate with Tokio. The asynchronous process
support is provided through signal handling on Unix and system APIs on Windows.

[`std::process::Command`]: std::process::Command

# Examples

Here's an example program which will spawn `echo hello world` and then wait
for it complete.

```no_run
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The usage is similar as with the standard library's `Command` type
    let mut child = Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn()
        .expect("failed to spawn");

    // Await until the command completes
    let status = child.wait().await?;
    println!("the command exited with: {}", status);
    Ok(())
}
```

Next, let's take a look at an example where we not only spawn `echo hello
world` but we also capture its output.

```no_run
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Like above, but use `output` which returns a future instead of
    // immediately returning the `Child`.
    let output = Command::new("echo").arg("hello").arg("world")
                        .output();

    let output = output.await?;

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello world\n");
    Ok(())
}
```

We can also read input line by line.

```no_run
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cat");

    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let stdout = child.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child.wait().await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}
```

Here is another example using `sort` writing into the child process
standard input, capturing the output of the sorted text.

```no_run
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sort");

    // Specifying that we want pipe both the output and the input.
    // Similarly to capturing the output, by configuring the pipe
    // to stdin it can now be used as an asynchronous writer.
    cmd.stdout(Stdio::piped());
    cmd.stdin(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    // These are the animals we want to sort
    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    // Write our animals to the child process
    // Note that the behavior of `sort` is to buffer _all input_ before writing any output.
    // In the general sense, it is recommended to write to the child in a separate task as
    // awaiting its exit (or output) to avoid deadlocks (for example, the child tries to write
    // some output but gets stuck waiting on the parent to read from it, meanwhile the parent
    // is stuck waiting to write its input completely before reading the output).
    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stdin");

    // We drop the handle here which signals EOF to the child process.
    // This tells the child process that it there is no more data on the pipe.
    drop(stdin);

    let op = child.wait_with_output().await?;

    // Results should come back in sorted order
    assert_eq!(op.stdout, "bird\ncat\ndog\nfish\nfrog\n".as_bytes());

    Ok(())
}
```

With some coordination, we can also pipe the output of one command into
another.

```no_run
use tokio::join;
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut echo = Command::new("echo")
        .arg("hello world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn echo");

    let tr_stdin: Stdio = echo
        .stdout
        .take()
        .unwrap()
        .try_into()
        .expect("failed to convert to Stdio");

    let tr = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(tr_stdin)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tr");

    let (echo_result, tr_output) = join!(echo.wait(), tr.wait_with_output());

    assert!(echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    assert!(tr_output.status.success());

    assert_eq!(tr_output.stdout, b"HELLO WORLD!\n");

    Ok(())
}
```

# Caveats

## Dropping/Cancellation

Similar to the behavior to the standard library, and unlike the futures
paradigm of dropping-implies-cancellation, a spawned process will, by
default, continue to execute even after the `Child` handle has been dropped.

The [`Command::kill_on_drop`] method can be used to modify this behavior
and kill the child process if the `Child` wrapper is dropped before it
has exited.

## Unix Processes

On Unix platforms processes must be "reaped" by their parent process after
they have exited in order to release all OS resources. A child process which
has exited, but has not yet been reaped by its parent is considered a "zombie"
process. Such processes continue to count against limits imposed by the system,
and having too many zombie processes present can prevent additional processes
from being spawned.

The tokio runtime will, on a best-effort basis, attempt to reap and clean up
any process which it has spawned. No additional guarantees are made with regard to
how quickly or how often this procedure will take place.

It is recommended to avoid dropping a [`Child`] process handle before it has been
fully `await`ed if stricter cleanup guarantees are required.

[`Command`]: crate::process::Command
[`Command::kill_on_drop`]: crate::process::Command::kill_on_drop
[`Child`]: crate::process::Child

```rust
pub mod process { /* ... */ }
```

### Types

#### Struct `Command`

This structure mimics the API of [`std::process::Command`] found in the standard library, but
replaces functions that create a process with an asynchronous variant. The main provided
asynchronous functions are [spawn](Command::spawn), [status](Command::status), and
[output](Command::output).

`Command` uses asynchronous versions of some `std` types (for example [`Child`]).

[`std::process::Command`]: std::process::Command
[`Child`]: struct@Child

```rust
pub struct Command {
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
  pub fn new<S: AsRef<OsStr>>(program: S) -> Command { /* ... */ }
  ```
  Constructs a new `Command` for launching the program at

- ```rust
  pub fn as_std(self: &Self) -> &StdCommand { /* ... */ }
  ```
  Cheaply convert to a `&std::process::Command` for places where the type from the standard

- ```rust
  pub fn as_std_mut(self: &mut Self) -> &mut StdCommand { /* ... */ }
  ```
  Cheaply convert to a `&mut std::process::Command` for places where the type from the

- ```rust
  pub fn into_std(self: Self) -> StdCommand { /* ... */ }
  ```
  Cheaply convert into a `std::process::Command`.

- ```rust
  pub fn arg<S: AsRef<OsStr>>(self: &mut Self, arg: S) -> &mut Command { /* ... */ }
  ```
  Adds an argument to pass to the program.

- ```rust
  pub fn args<I, S>(self: &mut Self, args: I) -> &mut Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr> { /* ... */ }
  ```
  Adds multiple arguments to pass to the program.

- ```rust
  pub fn env<K, V>(self: &mut Self, key: K, val: V) -> &mut Command
where
    K: AsRef<OsStr>,
    V: AsRef<OsStr> { /* ... */ }
  ```
  Inserts or updates an environment variable mapping.

- ```rust
  pub fn envs<I, K, V>(self: &mut Self, vars: I) -> &mut Command
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr> { /* ... */ }
  ```
  Adds or updates multiple environment variable mappings.

- ```rust
  pub fn env_remove<K: AsRef<OsStr>>(self: &mut Self, key: K) -> &mut Command { /* ... */ }
  ```
  Removes an environment variable mapping.

- ```rust
  pub fn env_clear(self: &mut Self) -> &mut Command { /* ... */ }
  ```
  Clears the entire environment map for the child process.

- ```rust
  pub fn current_dir<P: AsRef<Path>>(self: &mut Self, dir: P) -> &mut Command { /* ... */ }
  ```
  Sets the working directory for the child process.

- ```rust
  pub fn stdin<T: Into<Stdio>>(self: &mut Self, cfg: T) -> &mut Command { /* ... */ }
  ```
  Sets configuration for the child process's standard input (stdin) handle.

- ```rust
  pub fn stdout<T: Into<Stdio>>(self: &mut Self, cfg: T) -> &mut Command { /* ... */ }
  ```
  Sets configuration for the child process's standard output (stdout) handle.

- ```rust
  pub fn stderr<T: Into<Stdio>>(self: &mut Self, cfg: T) -> &mut Command { /* ... */ }
  ```
  Sets configuration for the child process's standard error (stderr) handle.

- ```rust
  pub fn kill_on_drop(self: &mut Self, kill_on_drop: bool) -> &mut Command { /* ... */ }
  ```
  Controls whether a `kill` operation should be invoked on a spawned child

- ```rust
  pub fn uid(self: &mut Self, id: u32) -> &mut Command { /* ... */ }
  ```
  Sets the child process's user ID. This translates to a

- ```rust
  pub fn gid(self: &mut Self, id: u32) -> &mut Command { /* ... */ }
  ```
  Similar to `uid` but sets the group ID of the child process. This has

- ```rust
  pub fn arg0<S>(self: &mut Self, arg: S) -> &mut Command
where
    S: AsRef<OsStr> { /* ... */ }
  ```
  Sets executable argument.

- ```rust
  pub unsafe fn pre_exec<F>(self: &mut Self, f: F) -> &mut Command
where
    F: FnMut() -> io::Result<()> + Send + Sync + ''static { /* ... */ }
  ```
  Schedules a closure to be run just before the `exec` function is

- ```rust
  pub fn process_group(self: &mut Self, pgroup: i32) -> &mut Command { /* ... */ }
  ```
  Sets the process group ID (PGID) of the child process. Equivalent to a

- ```rust
  pub fn spawn(self: &mut Self) -> io::Result<Child> { /* ... */ }
  ```
  Executes the command as a child process, returning a handle to it.

- ```rust
  pub fn status(self: &mut Self) -> impl Future<Output = io::Result<ExitStatus>> { /* ... */ }
  ```
  Executes the command as a child process, waiting for it to finish and

- ```rust
  pub fn output(self: &mut Self) -> impl Future<Output = io::Result<Output>> { /* ... */ }
  ```
  Executes the command as a child process, waiting for it to finish and

- ```rust
  pub fn get_kill_on_drop(self: &Self) -> bool { /* ... */ }
  ```
  Returns the boolean value that was previously set by [`Command::kill_on_drop`].

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(std: StdCommand) -> Command { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Child`

Representation of a child process spawned onto an event loop.

# Caveats
Similar to the behavior to the standard library, and unlike the futures
paradigm of dropping-implies-cancellation, a spawned process will, by
default, continue to execute even after the `Child` handle has been dropped.

The `Command::kill_on_drop` method can be used to modify this behavior
and kill the child process if the `Child` wrapper is dropped before it
has exited.

```rust
pub struct Child {
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `stdin` | `Option<ChildStdin>` | The handle for writing to the child's standard input (stdin), if it has<br>been captured. To avoid partially moving the `child` and thus blocking<br>yourself from calling functions on `child` while using `stdin`, you might<br>find it helpful to do:<br><br>```no_run<br># let mut child = tokio::process::Command::new("echo").spawn().unwrap();<br>let stdin = child.stdin.take().unwrap();<br>``` |
| `stdout` | `Option<ChildStdout>` | The handle for reading from the child's standard output (stdout), if it<br>has been captured. You might find it helpful to do<br><br>```no_run<br># let mut child = tokio::process::Command::new("echo").spawn().unwrap();<br>let stdout = child.stdout.take().unwrap();<br>```<br><br>to avoid partially moving the `child` and thus blocking yourself from calling<br>functions on `child` while using `stdout`. |
| `stderr` | `Option<ChildStderr>` | The handle for reading from the child's standard error (stderr), if it<br>has been captured. You might find it helpful to do<br><br>```no_run<br># let mut child = tokio::process::Command::new("echo").spawn().unwrap();<br>let stderr = child.stderr.take().unwrap();<br>```<br><br>to avoid partially moving the `child` and thus blocking yourself from calling<br>functions on `child` while using `stderr`. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn id(self: &Self) -> Option<u32> { /* ... */ }
  ```
  Returns the OS-assigned process identifier associated with this child

- ```rust
  pub fn start_kill(self: &mut Self) -> io::Result<()> { /* ... */ }
  ```
  Attempts to force the child to exit, but does not wait for the request

- ```rust
  pub async fn kill(self: &mut Self) -> io::Result<()> { /* ... */ }
  ```
  Forces the child to exit.

- ```rust
  pub async fn wait(self: &mut Self) -> io::Result<ExitStatus> { /* ... */ }
  ```
  Waits for the child to exit completely, returning the status that it

- ```rust
  pub fn try_wait(self: &mut Self) -> io::Result<Option<ExitStatus>> { /* ... */ }
  ```
  Attempts to collect the exit status of the child if it has already

- ```rust
  pub async fn wait_with_output(self: Self) -> io::Result<Output> { /* ... */ }
  ```
  Returns a future that will resolve to an `Output`, containing the exit

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ChildStdin`

The standard input stream for spawned children.

This type implements the `AsyncWrite` trait to pass data to the stdin
handle of a child process asynchronously.

```rust
pub struct ChildStdin {
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
  pub fn into_owned_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Convert into [`OwnedFd`].

- ```rust
  pub fn from_std(inner: std::process::ChildStdin) -> io::Result<Self> { /* ... */ }
  ```
  Creates an asynchronous `ChildStdin` from a synchronous one.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsFd**
  - ```rust
    fn as_fd(self: &Self) -> BorrowedFd<''_> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_into(self: Self) -> Result<Stdio, <Self as >::Error> { /* ... */ }
    ```

- **AsyncWriteExt**
- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsyncWrite**
  - ```rust
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &[u8]) -> Poll<io::Result<usize>> { /* ... */ }
    ```

  - ```rust
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

  - ```rust
    fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[io::IoSlice<''_>]) -> Poll<Result<usize, io::Error>> { /* ... */ }
    ```

  - ```rust
    fn is_write_vectored(self: &Self) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
#### Struct `ChildStdout`

The standard output stream for spawned children.

This type implements the `AsyncRead` trait to read data from the stdout
handle of a child process asynchronously.

```rust
pub struct ChildStdout {
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
  pub fn into_owned_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Convert into [`OwnedFd`].

- ```rust
  pub fn from_std(inner: std::process::ChildStdout) -> io::Result<Self> { /* ... */ }
  ```
  Creates an asynchronous `ChildStdout` from a synchronous one.

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_into(self: Self) -> Result<Stdio, <Self as >::Error> { /* ... */ }
    ```

- **Unpin**
- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **AsyncReadExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsFd**
  - ```rust
    fn as_fd(self: &Self) -> BorrowedFd<''_> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ChildStderr`

The standard error stream for spawned children.

This type implements the `AsyncRead` trait to read data from the stderr
handle of a child process asynchronously.

```rust
pub struct ChildStderr {
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
  pub fn into_owned_fd(self: Self) -> io::Result<OwnedFd> { /* ... */ }
  ```
  Convert into [`OwnedFd`].

- ```rust
  pub fn from_std(inner: std::process::ChildStderr) -> io::Result<Self> { /* ... */ }
  ```
  Creates an asynchronous `ChildStderr` from a synchronous one.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **AsyncRead**
  - ```rust
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<''_>, buf: &mut ReadBuf<''_>) -> Poll<io::Result<()>> { /* ... */ }
    ```

- **AsRawFd**
  - ```rust
    fn as_raw_fd(self: &Self) -> RawFd { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **AsyncReadExt**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_into(self: Self) -> Result<Stdio, <Self as >::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsFd**
  - ```rust
    fn as_fd(self: &Self) -> BorrowedFd<''_> { /* ... */ }
    ```

## Module `runtime`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

The Tokio runtime.

Unlike other Rust programs, asynchronous applications require runtime
support. In particular, the following runtime services are necessary:

* An **I/O event loop**, called the driver, which drives I/O resources and
  dispatches I/O events to tasks that depend on them.
* A **scheduler** to execute [tasks] that use these I/O resources.
* A **timer** for scheduling work to run after a set period of time.

Tokio's [`Runtime`] bundles all of these services as a single type, allowing
them to be started, shut down, and configured together. However, often it is
not required to configure a [`Runtime`] manually, and a user may just use the
[`tokio::main`] attribute macro, which creates a [`Runtime`] under the hood.

# Usage

When no fine tuning is required, the [`tokio::main`] attribute macro can be
used.

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

From within the context of the runtime, additional tasks are spawned using
the [`tokio::spawn`] function. Futures spawned using this function will be
executed on the same thread pool used by the [`Runtime`].

A [`Runtime`] instance can also be used directly.

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt  = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
```

## Runtime Configurations

Tokio provides multiple task scheduling strategies, suitable for different
applications. The [runtime builder] or `#[tokio::main]` attribute may be
used to select which scheduler to use.

#### Multi-Thread Scheduler

The multi-thread scheduler executes futures on a _thread pool_, using a
work-stealing strategy. By default, it will start a worker thread for each
CPU core available on the system. This tends to be the ideal configuration
for most applications. The multi-thread scheduler requires the `rt-multi-thread`
feature flag, and is selected by default:
```
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let threaded_rt = runtime::Runtime::new()?;
# Ok(()) }
```

Most applications should use the multi-thread scheduler, except in some
niche use-cases, such as when running only a single thread is required.

#### Current-Thread Scheduler

The current-thread scheduler provides a _single-threaded_ future executor.
All tasks will be created and executed on the current thread. This requires
the `rt` feature flag.
```
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let rt = runtime::Builder::new_current_thread()
    .build()?;
# Ok(()) }
```

#### Resource drivers

When configuring a runtime by hand, no resource drivers are enabled by
default. In this case, attempting to use networking types or time types will
fail. In order to enable these types, the resource drivers must be enabled.
This is done with [`Builder::enable_io`] and [`Builder::enable_time`]. As a
shorthand, [`Builder::enable_all`] enables both resource drivers.

## Lifetime of spawned threads

The runtime may spawn threads depending on its configuration and usage. The
multi-thread scheduler spawns threads to schedule tasks and for `spawn_blocking`
calls.

While the `Runtime` is active, threads may shut down after periods of being
idle. Once `Runtime` is dropped, all runtime threads have usually been
terminated, but in the presence of unstoppable spawned work are not
guaranteed to have been terminated. See the
[struct level documentation](Runtime#shutdown) for more details.

[tasks]: crate::task
[`Runtime`]: Runtime
[`tokio::spawn`]: crate::spawn
[`tokio::main`]: ../attr.main.html
[runtime builder]: crate::runtime::Builder
[`Runtime::new`]: crate::runtime::Runtime::new
[`Builder::threaded_scheduler`]: crate::runtime::Builder::threaded_scheduler
[`Builder::enable_io`]: crate::runtime::Builder::enable_io
[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

# Detailed runtime behavior

This section gives more details into how the Tokio runtime will schedule
tasks for execution.

At its most basic level, a runtime has a collection of tasks that need to be
scheduled. It will repeatedly remove a task from that collection and
schedule it (by calling [`poll`]). When the collection is empty, the thread
will go to sleep until a task is added to the collection.

However, the above is not sufficient to guarantee a well-behaved runtime.
For example, the runtime might have a single task that is always ready to be
scheduled, and schedule that task every time. This is a problem because it
starves other tasks by not scheduling them. To solve this, Tokio provides
the following fairness guarantee:

> If the total number of tasks does not grow without bound, and no task is
> [blocking the thread], then it is guaranteed that tasks are scheduled
> fairly.

Or, more formally:

> Under the following two assumptions:
>
> * There is some number `MAX_TASKS` such that the total number of tasks on
>   the runtime at any specific point in time never exceeds `MAX_TASKS`.
> * There is some number `MAX_SCHEDULE` such that calling [`poll`] on any
>   task spawned on the runtime returns within `MAX_SCHEDULE` time units.
>
> Then, there is some number `MAX_DELAY` such that when a task is woken, it
> will be scheduled by the runtime within `MAX_DELAY` time units.

(Here, `MAX_TASKS` and `MAX_SCHEDULE` can be any number and the user of
the runtime may choose them. The `MAX_DELAY` number is controlled by the
runtime, and depends on the value of `MAX_TASKS` and `MAX_SCHEDULE`.)

Other than the above fairness guarantee, there is no guarantee about the
order in which tasks are scheduled. There is also no guarantee that the
runtime is equally fair to all tasks. For example, if the runtime has two
tasks A and B that are both ready, then the runtime may schedule A five
times before it schedules B. This is the case even if A yields using
[`yield_now`]. All that is guaranteed is that it will schedule B eventually.

Normally, tasks are scheduled only if they have been woken by calling
[`wake`] on their waker. However, this is not guaranteed, and Tokio may
schedule tasks that have not been woken under some circumstances. This is
called a spurious wakeup.

## IO and timers

Beyond just scheduling tasks, the runtime must also manage IO resources and
timers. It does this by periodically checking whether there are any IO
resources or timers that are ready, and waking the relevant task so that
it will be scheduled.

These checks are performed periodically between scheduling tasks. Under the
same assumptions as the previous fairness guarantee, Tokio guarantees that
it will wake tasks with an IO or timer event within some maximum number of
time units.

## Current thread runtime (behavior at the time of writing)

This section describes how the [current thread runtime] behaves today. This
behavior may change in future versions of Tokio.

The current thread runtime maintains two FIFO queues of tasks that are ready
to be scheduled: the global queue and the local queue. The runtime will prefer
to choose the next task to schedule from the local queue, and will only pick a
task from the global queue if the local queue is empty, or if it has picked
a task from the local queue 31 times in a row. The number 31 can be
changed using the [`global_queue_interval`] setting.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row. The
number 61 may be changed using the [`event_interval`] setting.

When a task is woken from within a task running on the runtime, then the
woken task is added directly to the local queue. Otherwise, the task is
added to the global queue. The current thread runtime does not use [the lifo
slot optimization].

## Multi threaded runtime (behavior at the time of writing)

This section describes how the [multi thread runtime] behaves today. This
behavior may change in future versions of Tokio.

A multi thread runtime has a fixed number of worker threads, which are all
created on startup. The multi thread runtime maintains one global queue, and
a local queue for each worker thread. The local queue of a worker thread can
fit at most 256 tasks. If more than 256 tasks are added to the local queue,
then half of them are moved to the global queue to make space.

The runtime will prefer to choose the next task to schedule from the local
queue, and will only pick a task from the global queue if the local queue is
empty, or if it has picked a task from the local queue
[`global_queue_interval`] times in a row. If the value of
[`global_queue_interval`] is not explicitly set using the runtime builder,
then the runtime will dynamically compute it using a heuristic that targets
10ms intervals between each check of the global queue (based on the
[`worker_mean_poll_time`] metric).

If both the local queue and global queue is empty, then the worker thread
will attempt to steal tasks from the local queue of another worker thread.
Stealing is done by moving half of the tasks in one local queue to another
local queue.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row. The
number 61 may be changed using the [`event_interval`] setting.

The multi thread runtime uses [the lifo slot optimization]: Whenever a task
wakes up another task, the other task is added to the worker thread's lifo
slot instead of being added to a queue. If there was already a task in the
lifo slot when this happened, then the lifo slot is replaced, and the task
that used to be in the lifo slot is placed in the thread's local queue.
When the runtime finishes scheduling a task, it will schedule the task in
the lifo slot immediately, if any. When the lifo slot is used, the [coop
budget] is not reset. Furthermore, if a worker thread uses the lifo slot
three times in a row, it is temporarily disabled until the worker thread has
scheduled a task that didn't come from the lifo slot. The lifo slot can be
disabled using the [`disable_lifo_slot`] setting. The lifo slot is separate
from the local queue, so other worker threads cannot steal the task in the
lifo slot.

When a task is woken from a thread that is not a worker thread, then the
task is placed in the global queue.

[`poll`]: std::future::Future::poll
[`wake`]: std::task::Waker::wake
[`yield_now`]: crate::task::yield_now
[blocking the thread]: https://ryhl.io/blog/async-what-is-blocking/
[current thread runtime]: crate::runtime::Builder::new_current_thread
[multi thread runtime]: crate::runtime::Builder::new_multi_thread
[`global_queue_interval`]: crate::runtime::Builder::global_queue_interval
[`event_interval`]: crate::runtime::Builder::event_interval
[`disable_lifo_slot`]: crate::runtime::Builder::disable_lifo_slot
[the lifo slot optimization]: crate::runtime::Builder::disable_lifo_slot
[coop budget]: crate::task::coop#cooperative-scheduling
[`worker_mean_poll_time`]: crate::runtime::RuntimeMetrics::worker_mean_poll_time

```rust
pub mod runtime { /* ... */ }
```

### Re-exports

#### Re-export `Builder`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use self::builder::Builder;
```

#### Re-export `EnterGuard`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use handle::EnterGuard;
```

#### Re-export `Handle`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use handle::Handle;
```

#### Re-export `TryCurrentError`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use handle::TryCurrentError;
```

#### Re-export `Runtime`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use runtime::Runtime;
```

#### Re-export `RuntimeFlavor`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use runtime::RuntimeFlavor;
```

#### Re-export `RuntimeMetrics`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use metrics::RuntimeMetrics;
```

## Module `signal`

**Attributes:**

- `#[<cfg>(feature = "signal")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "signal")))]`
- `#[<cfg>(not(loom))]`
- `#[<cfg>(not(target_os = "wasi"))]`

Asynchronous signal handling for Tokio.

Note that signal handling is in general a very tricky topic and should be
used with great care. This crate attempts to implement 'best practice' for
signal handling, but it should be evaluated for your own applications' needs
to see if it's suitable.

There are some fundamental limitations of this crate documented on the OS
specific structures, as well.

# Examples

Print on "ctrl-c" notification.

```rust,no_run
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}
```

Wait for `SIGHUP` on Unix

```rust,no_run
# #[cfg(unix)] {
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut stream = signal(SignalKind::hangup())?;

    // Print whenever a HUP signal is received
    loop {
        stream.recv().await;
        println!("got signal HUP");
    }
}
# }
```

```rust
pub mod signal { /* ... */ }
```

### Modules

## Module `unix`

**Attributes:**

- `#[<cfg>(unix)]`
- `#[<cfg_attr>(docsrs, doc(cfg(all(unix, feature = "signal"))))]`

Unix-specific types for signal handling.

This module is only defined on Unix platforms and contains the primary
`Signal` type for receiving notifications of signals.

```rust
pub mod unix { /* ... */ }
```

### Types

#### Struct `SignalKind`

Represents the specific kind of signal to listen for.

```rust
pub struct SignalKind(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub const fn from_raw(signum: std::os::raw::c_int) -> Self { /* ... */ }
  ```
  Allows for listening to any valid OS signal.

- ```rust
  pub const fn as_raw_value(self: &Self) -> std::os::raw::c_int { /* ... */ }
  ```
  Get the signal's numeric value.

- ```rust
  pub const fn alarm() -> Self { /* ... */ }
  ```
  Represents the `SIGALRM` signal.

- ```rust
  pub const fn child() -> Self { /* ... */ }
  ```
  Represents the `SIGCHLD` signal.

- ```rust
  pub const fn hangup() -> Self { /* ... */ }
  ```
  Represents the `SIGHUP` signal.

- ```rust
  pub const fn interrupt() -> Self { /* ... */ }
  ```
  Represents the `SIGINT` signal.

- ```rust
  pub const fn io() -> Self { /* ... */ }
  ```
  Represents the `SIGIO` signal.

- ```rust
  pub const fn pipe() -> Self { /* ... */ }
  ```
  Represents the `SIGPIPE` signal.

- ```rust
  pub const fn quit() -> Self { /* ... */ }
  ```
  Represents the `SIGQUIT` signal.

- ```rust
  pub const fn terminate() -> Self { /* ... */ }
  ```
  Represents the `SIGTERM` signal.

- ```rust
  pub const fn user_defined1() -> Self { /* ... */ }
  ```
  Represents the `SIGUSR1` signal.

- ```rust
  pub const fn user_defined2() -> Self { /* ... */ }
  ```
  Represents the `SIGUSR2` signal.

- ```rust
  pub const fn window_change() -> Self { /* ... */ }
  ```
  Represents the `SIGWINCH` signal.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SignalKind) -> bool { /* ... */ }
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

- **RefUnwindSafe**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Send**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SignalKind { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(signum: std::os::raw::c_int) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(kind: SignalKind) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Signal`

**Attributes:**

- `#[must_use = "streams do nothing unless polled"]`

An listener for receiving a particular type of OS signal.

The listener can be turned into a `Stream` using [`SignalStream`].

[`SignalStream`]: https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.SignalStream.html

In general signal handling on Unix is a pretty tricky topic, and this
structure is no exception! There are some important limitations to keep in
mind when using `Signal` streams:

* Signals handling in Unix already necessitates coalescing signals
  together sometimes. This `Signal` stream is also no exception here in
  that it will also coalesce signals. That is, even if the signal handler
  for this process runs multiple times, the `Signal` stream may only return
  one signal notification. Specifically, before `poll` is called, all
  signal notifications are coalesced into one item returned from `poll`.
  Once `poll` has been called, however, a further signal is guaranteed to
  be yielded as an item.

  Put another way, any element pulled off the returned listener corresponds to
  *at least one* signal, but possibly more.

* Signal handling in general is relatively inefficient. Although some
  improvements are possible in this crate, it's recommended to not plan on
  having millions of signal channels open.

If you've got any questions about this feel free to open an issue on the
repo! New approaches to alleviate some of these limitations are always
appreciated!

# Caveats

The first time that a `Signal` instance is registered for a particular
signal kind, an OS signal-handler is installed which replaces the default
platform behavior when that signal is received, **for the duration of the
entire process**.

For example, Unix systems will terminate a process by default when it
receives `SIGINT`. But, when a `Signal` instance is created to listen for
this signal, the next `SIGINT` that arrives will be translated to a stream
event, and the process will continue to execute. **Even if this `Signal`
instance is dropped, subsequent `SIGINT` deliveries will end up captured by
Tokio, and the default platform behavior will NOT be reset**.

Thus, applications should take care to ensure the expected signal behavior
occurs as expected after listening for specific signals.

# Examples

Wait for `SIGHUP`

```rust,no_run
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut sig = signal(SignalKind::hangup())?;

    // Print whenever a HUP signal is received
    loop {
        sig.recv().await;
        println!("got signal HUP");
    }
}
```

```rust
pub struct Signal {
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
  pub async fn recv(self: &mut Self) -> Option<()> { /* ... */ }
  ```
  Receives the next signal notification event.

- ```rust
  pub fn poll_recv(self: &mut Self, cx: &mut Context<''_>) -> Poll<Option<()>> { /* ... */ }
  ```
  Polls to receive the next signal notification event, outside of an

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `signal`

**Attributes:**

- `#[track_caller]`

Creates a new listener which will receive notifications when the current
process receives the specified signal `kind`.

This function will create a new stream which binds to the default reactor.
The `Signal` stream is an infinite stream which will receive
notifications whenever a signal is received. More documentation can be
found on `Signal` itself, but to reiterate:

* Signals may be coalesced beyond what the kernel already does.
* Once a signal handler is registered with the process the underlying
  libc signal handler is never unregistered.

A `Signal` stream can be created for a particular signal number
multiple times. When a signal is received then all the associated
channels will receive the signal notification.

# Errors

* If the lower-level C functions fail for some reason.
* If the previous initialization of this specific signal failed.
* If the signal is one of
  [`signal_hook::FORBIDDEN`](fn@signal_hook_registry::register#panics)

# Panics

This function panics if there is no current reactor set, or if the `rt`
feature flag is not enabled.

```rust
pub fn signal(kind: SignalKind) -> io::Result<Signal> { /* ... */ }
```

### Re-exports

#### Re-export `ctrl_c`

**Attributes:**

- `#[<cfg>(feature = "signal")]`

```rust
pub use ctrl_c::ctrl_c;
```

## Module `sync`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`
- `#[<cfg_attr>(loom, allow(dead_code, unreachable_pub, unused_imports))]`

Synchronization primitives for use in asynchronous contexts.

Tokio programs tend to be organized as a set of [tasks] where each task
operates independently and may be executed on separate physical threads. The
synchronization primitives provided in this module permit these independent
tasks to communicate together.

[tasks]: crate::task

# Message passing

The most common form of synchronization in a Tokio program is message
passing. Two tasks operate independently and send messages to each other to
synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using channels. A channel supports sending a
message from one producer task to one or more consumer tasks. There are a
few flavors of channels provided by Tokio. Each channel flavor supports
different message passing patterns. When a channel supports multiple
producers, many separate tasks may **send** messages. When a channel
supports multiple consumers, many different separate tasks may **receive**
messages.

Tokio provides many different channel flavors as different message passing
patterns are best handled with different implementations.

## `oneshot` channel

The [`oneshot` channel][oneshot] supports sending a **single** value from a
single producer to a single consumer. This channel is usually used to send
the result of a computation to a waiter.

**Example:** using a [`oneshot` channel][oneshot] to receive the result of a
computation.

```
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = rx.await.unwrap();
}
```

Note, if the task produces a computation result as its final
action before terminating, the [`JoinHandle`] can be used to
receive that value instead of allocating resources for the
`oneshot` channel. Awaiting on [`JoinHandle`] returns `Result`. If
the task panics, the `Joinhandle` yields `Err` with the panic
cause.

**Example:**

```
async fn some_computation() -> String {
    "the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let join_handle = tokio::spawn(async move {
        some_computation().await
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = join_handle.await.unwrap();
}
```

[`JoinHandle`]: crate::task::JoinHandle

## `mpsc` channel

The [`mpsc` channel][mpsc] supports sending **many** values from **many**
producers to a single consumer. This channel is often used to send work to a
task or to receive the result of many computations.

This is also the channel you should use if you want to send many messages
from a single producer to a single consumer. There is no dedicated spsc
channel.

**Example:** using an mpsc to incrementally stream the results of a series
of computations.

```
use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!("got = {}", res);
    }
}
```

The argument to `mpsc::channel` is the channel capacity. This is the maximum
number of values that can be stored in the channel pending receipt at any
given time. Properly setting this value is key in implementing robust
programs as the channel capacity plays a critical part in handling back
pressure.

A common concurrency pattern for resource management is to spawn a task
dedicated to managing that resource and using message passing between other
tasks to interact with the resource. The resource may be anything that may
not be concurrently used. Some examples include a socket and program state.
For example, if multiple tasks need to send data over a single socket, spawn
a task to manage the socket and use a channel to synchronize.

**Example:** sending data from many tasks over a single socket using message
passing.

```no_run
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let (tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the
        // original handle.
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.
    drop(tx);

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}
```

The [`mpsc`] and [`oneshot`] channels can be combined to provide a request /
response type synchronization pattern with a shared resource. A task is
spawned to synchronize a resource and waits on commands received on a
[`mpsc`] channel. Each command includes a [`oneshot`] `Sender` on which the
result of the command is sent.

**Example:** use a task to synchronize a `u64` counter. Each task sends an
"fetch and increment" command. The counter value **before** the increment is
sent over the provided `oneshot` channel.

```
use tokio::sync::{oneshot, mpsc};
use Command::Increment;

enum Command {
    Increment,
    // Other commands can be added here
}

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

    // Spawn a task to manage the counter
    tokio::spawn(async move {
        let mut counter: u64 = 0;

        while let Some((cmd, response)) = cmd_rx.recv().await {
            match cmd {
                Increment => {
                    let prev = counter;
                    counter += 1;
                    response.send(prev).unwrap();
                }
            }
        }
    });

    let mut join_handles = vec![];

    // Spawn tasks that will send the increment command.
    for _ in 0..10 {
        let cmd_tx = cmd_tx.clone();

        join_handles.push(tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
            let res = resp_rx.await.unwrap();

            println!("previous value = {}", res);
        }));
    }

    // Wait for all tasks to complete
    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
}
```

## `broadcast` channel

The [`broadcast` channel] supports sending **many** values from
**many** producers to **many** consumers. Each consumer will receive
**each** value. This channel can be used to implement "fan out" style
patterns common with pub / sub or "chat" systems.

This channel tends to be used less often than `oneshot` and `mpsc` but still
has its use cases.

This is also the channel you should use if you want to broadcast values from
a single producer to many consumers. There is no dedicated spmc broadcast
channel.

Basic usage

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

[`broadcast` channel]: crate::sync::broadcast

## `watch` channel

The [`watch` channel] supports sending **many** values from a **many**
producer to **many** consumers. However, only the **most recent** value is
stored in the channel. Consumers are notified when a new value is sent, but
there is no guarantee that consumers will see **all** values.

The [`watch` channel] is similar to a [`broadcast` channel] with capacity 1.

Use cases for the [`watch` channel] include broadcasting configuration
changes or signalling program state changes, such as transitioning to
shutdown.

**Example:** use a [`watch` channel] to notify tasks of configuration
changes. In this example, a configuration file is checked periodically. When
the file changes, the configuration changes are signalled to consumers.

```
use tokio::sync::watch;
use tokio::time::{self, Duration, Instant};

use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
# Ok(Config { timeout: Duration::from_secs(1) })
    }
}

async fn my_async_operation() {
    // Do something here
}

#[tokio::main]
async fn main() {
    // Load initial configuration value
    let mut config = Config::load_from_file().await.unwrap();

    // Create the watch channel, initialized with the loaded configuration
    let (tx, rx) = watch::channel(config.clone());

    // Spawn a task to monitor the file.
    tokio::spawn(async move {
        loop {
            // Wait 10 seconds between checks
            time::sleep(Duration::from_secs(10)).await;

            // Load the configuration file
            let new_config = Config::load_from_file().await.unwrap();

            // If the configuration changed, send the new config value
            // on the watch channel.
            if new_config != config {
                tx.send(new_config.clone()).unwrap();
                config = new_config;
            }
        }
    });

    let mut handles = vec![];

    // Spawn tasks that runs the async operation for at most `timeout`. If
    // the timeout elapses, restart the operation.
    //
    // The task simultaneously watches the `Config` for changes. When the
    // timeout duration changes, the timeout is updated without restarting
    // the in-flight operation.
    for _ in 0..5 {
        // Clone a config watch handle for use in this task
        let mut rx = rx.clone();

        let handle = tokio::spawn(async move {
            // Start the initial operation and pin the future to the stack.
            // Pinning to the stack is required to resume the operation
            // across multiple calls to `select!`
            let op = my_async_operation();
            tokio::pin!(op);

            // Get the initial config value
            let mut conf = rx.borrow().clone();

            let mut op_start = Instant::now();
            let sleep = time::sleep_until(op_start + conf.timeout);
            tokio::pin!(sleep);

            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        // The operation elapsed. Restart it
                        op.set(my_async_operation());

                        // Track the new start time
                        op_start = Instant::now();

                        // Restart the timeout
                        sleep.set(time::sleep_until(op_start + conf.timeout));
                    }
                    _ = rx.changed() => {
                        conf = rx.borrow_and_update().clone();

                        // The configuration has been updated. Update the
                        // `sleep` using the new `timeout` value.
                        sleep.as_mut().reset(op_start + conf.timeout);
                    }
                    _ = &mut op => {
                        // The operation completed!
                        return
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }
}
```

[`watch` channel]: mod@crate::sync::watch
[`broadcast` channel]: mod@crate::sync::broadcast

# State synchronization

The remaining synchronization primitives focus on synchronizing state.
These are asynchronous equivalents to versions provided by `std`. They
operate in a similar way as their `std` counterparts but will wait
asynchronously instead of blocking the thread.

* [`Barrier`] Ensures multiple tasks will wait for each other to reach a
  point in the program, before continuing execution all together.

* [`Mutex`] Mutual Exclusion mechanism, which ensures that at most one
  thread at a time is able to access some data.

* [`Notify`] Basic task notification. `Notify` supports notifying a
  receiving task without sending data. In this case, the task wakes up and
  resumes processing.

* [`RwLock`] Provides a mutual exclusion mechanism which allows multiple
  readers at the same time, while allowing only one writer at a time. In
  some cases, this can be more efficient than a mutex.

* [`Semaphore`] Limits the amount of concurrency. A semaphore holds a
  number of permits, which tasks may request in order to enter a critical
  section. Semaphores are useful for implementing limiting or bounding of
  any kind.

# Runtime compatibility

All synchronization primitives provided in this module are runtime agnostic.
You can freely move them between different instances of the Tokio runtime
or even use them from non-Tokio runtimes.

When used in a Tokio runtime, the synchronization primitives participate in
[cooperative scheduling](crate::task::coop#cooperative-scheduling) to avoid
starvation. This feature does not apply when used from non-Tokio runtimes.

As an exception, methods ending in `_timeout` are not runtime agnostic
because they require access to the Tokio timer. See the documentation of
each `*_timeout` method for more information on its use.

```rust
pub mod sync { /* ... */ }
```

### Modules

## Module `futures`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

Named future types.

```rust
pub mod futures { /* ... */ }
```

### Re-exports

#### Re-export `Notified`

```rust
pub use super::notify::Notified;
```

## Module `broadcast`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
all consumers.

A [`Sender`] is used to broadcast values to **all** connected [`Receiver`]
values. [`Sender`] handles are clone-able, allowing concurrent send and
receive actions. [`Sender`] and [`Receiver`] are both `Send` and `Sync` as
long as `T` is `Send`.

When a value is sent, **all** [`Receiver`] handles are notified and will
receive the value. The value is stored once inside the channel and cloned on
demand for each receiver. Once all receivers have received a clone of the
value, the value is released from the channel.

A channel is created by calling [`channel`], specifying the maximum number
of messages the channel can retain at any given time.

New [`Receiver`] handles are created by calling [`Sender::subscribe`]. The
returned [`Receiver`] will receive values sent **after** the call to
`subscribe`.

This channel is also suitable for the single-producer multi-consumer
use-case, where a single sender broadcasts values to many receivers.

## Lagging

As sent messages must be retained until **all** [`Receiver`] handles receive
a clone, broadcast channels are susceptible to the "slow receiver" problem.
In this case, all but one receiver are able to receive values at the rate
they are sent. Because one receiver is stalled, the channel starts to fill
up.

This broadcast channel implementation handles this case by setting a hard
upper bound on the number of values the channel may retain at any given
time. This upper bound is passed to the [`channel`] function as an argument.

If a value is sent when the channel is at capacity, the oldest value
currently held by the channel is released. This frees up space for the new
value. Any receiver that has not yet seen the released value will return
[`RecvError::Lagged`] the next time [`recv`] is called.

Once [`RecvError::Lagged`] is returned, the lagging receiver's position is
updated to the oldest value contained by the channel. The next call to
[`recv`] will return this value.

This behavior enables a receiver to detect when it has lagged so far behind
that data has been dropped. The caller may decide how to respond to this:
either by aborting its task or by tolerating lost messages and resuming
consumption of the channel.

## Closing

When **all** [`Sender`] handles have been dropped, no new values may be
sent. At this point, the channel is "closed". Once a receiver has received
all values retained by the channel, the next call to [`recv`] will return
with [`RecvError::Closed`].

When a [`Receiver`] handle is dropped, any messages not read by the receiver
will be marked as read. If this receiver was the only one not to have read
that message, the message will be dropped at this point.

[`Sender`]: crate::sync::broadcast::Sender
[`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
[`Receiver`]: crate::sync::broadcast::Receiver
[`channel`]: crate::sync::broadcast::channel
[`RecvError::Lagged`]: crate::sync::broadcast::error::RecvError::Lagged
[`RecvError::Closed`]: crate::sync::broadcast::error::RecvError::Closed
[`recv`]: crate::sync::broadcast::Receiver::recv

# Examples

Basic usage

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

Handling lag

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(2);

    tx.send(10).unwrap();
    tx.send(20).unwrap();
    tx.send(30).unwrap();

    // The receiver lagged behind
    assert!(rx.recv().await.is_err());

    // At this point, we can abort or continue with lost messages

    assert_eq!(20, rx.recv().await.unwrap());
    assert_eq!(30, rx.recv().await.unwrap());
}
```

```rust
pub mod broadcast { /* ... */ }
```

### Modules

## Module `error`

Broadcast error types

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `SendError`

Error returned by the [`send`] function on a [`Sender`].

A **send** operation can only fail if there are no active receivers,
implying that the message could never be received. The error contains the
message being sent as a payload so it can be recovered.

[`send`]: crate::sync::broadcast::Sender::send
[`Sender`]: crate::sync::broadcast::Sender

```rust
pub struct SendError<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **UnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Error**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `RecvError`

An error returned from the [`recv`] function on a [`Receiver`].

[`recv`]: crate::sync::broadcast::Receiver::recv
[`Receiver`]: crate::sync::broadcast::Receiver

```rust
pub enum RecvError {
    Closed,
    Lagged(u64),
}
```

##### Variants

###### `Closed`

There are no more active senders implying no further messages will ever
be sent.

###### `Lagged`

The receiver lagged too far behind. Attempting to receive again will
return the oldest message still retained by the channel.

Includes the number of skipped messages.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RecvError { /* ... */ }
    ```

- **Error**
- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RecvError) -> bool { /* ... */ }
    ```

#### Enum `TryRecvError`

An error returned from the [`try_recv`] function on a [`Receiver`].

[`try_recv`]: crate::sync::broadcast::Receiver::try_recv
[`Receiver`]: crate::sync::broadcast::Receiver

```rust
pub enum TryRecvError {
    Empty,
    Closed,
    Lagged(u64),
}
```

##### Variants

###### `Empty`

The channel is currently empty. There are still active
[`Sender`] handles, so data may yet become available.

[`Sender`]: crate::sync::broadcast::Sender

###### `Closed`

There are no more active senders implying no further messages will ever
be sent.

###### `Lagged`

The receiver lagged too far behind and has been forcibly disconnected.
Attempting to receive again will return the oldest message still
retained by the channel.

Includes the number of skipped messages.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryRecvError { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryRecvError) -> bool { /* ... */ }
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

- **Error**
- **RefUnwindSafe**
- **Unpin**
### Types

#### Struct `Sender`

Sending-half of the [`broadcast`] channel.

May be used from many threads. Messages can be sent with
[`send`][Sender::send].

# Examples

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

[`broadcast`]: crate::sync::broadcast

```rust
pub struct Sender<T> {
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
  pub fn new(capacity: usize) -> Self { /* ... */ }
  ```
  Creates the sending-half of the [`broadcast`] channel.

- ```rust
  pub fn send(self: &Self, value: T) -> Result<usize, SendError<T>> { /* ... */ }
  ```
  Attempts to send a value to all active [`Receiver`] handles, returning

- ```rust
  pub fn subscribe(self: &Self) -> Receiver<T> { /* ... */ }
  ```
  Creates a new [`Receiver`] handle that will receive values sent **after**

- ```rust
  pub fn downgrade(self: &Self) -> WeakSender<T> { /* ... */ }
  ```
  Converts the `Sender` to a [`WeakSender`] that does not count

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of queued values.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if there are no queued values.

- ```rust
  pub fn receiver_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of active receivers.

- ```rust
  pub fn same_channel(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if senders belong to the same channel.

- ```rust
  pub async fn closed(self: &Self) { /* ... */ }
  ```
  A future which completes when the number of [Receiver]s subscribed to this `Sender` reaches

- ```rust
  pub fn strong_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`Sender`] handles.

- ```rust
  pub fn weak_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`WeakSender`] handles.

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sender<T> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `WeakSender`

A sender that does not prevent the channel from being closed.

If all [`Sender`] instances of a channel were dropped and only `WeakSender`
instances remain, the channel is closed.

In order to send messages, the `WeakSender` needs to be upgraded using
[`WeakSender::upgrade`], which returns `Option<Sender>`. It returns `None`
if all `Sender`s have been dropped, and otherwise it returns a `Sender`.

[`Sender`]: Sender
[`WeakSender::upgrade`]: WeakSender::upgrade

# Examples

```
use tokio::sync::broadcast::channel;

#[tokio::main]
async fn main() {
    let (tx, _rx) = channel::<i32>(15);
    let tx_weak = tx.downgrade();

    // Upgrading will succeed because `tx` still exists.
    assert!(tx_weak.upgrade().is_some());

    // If we drop `tx`, then it will fail.
    drop(tx);
    assert!(tx_weak.clone().upgrade().is_none());
}
```

```rust
pub struct WeakSender<T> {
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
  pub fn upgrade(self: &Self) -> Option<Sender<T>> { /* ... */ }
  ```
  Tries to convert a `WeakSender` into a [`Sender`].

- ```rust
  pub fn strong_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`Sender`] handles.

- ```rust
  pub fn weak_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`WeakSender`] handles.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WeakSender<T> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `Receiver`

Receiving-half of the [`broadcast`] channel.

Must not be used concurrently. Messages may be retrieved using
[`recv`][Receiver::recv].

To turn this receiver into a `Stream`, you can use the [`BroadcastStream`]
wrapper.

[`BroadcastStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.BroadcastStream.html

# Examples

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

[`broadcast`]: crate::sync::broadcast

```rust
pub struct Receiver<T> {
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
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of messages that were sent into the channel and that

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if there aren't any messages in the channel that the [`Receiver`]

- ```rust
  pub fn same_channel(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if receivers belong to the same channel.

- ```rust
  pub fn sender_strong_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`Sender`] handles.

- ```rust
  pub fn sender_weak_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of [`WeakSender`] handles.

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Checks if a channel is closed.

- ```rust
  pub fn resubscribe(self: &Self) -> Self { /* ... */ }
  ```
  Re-subscribes to the channel starting from the current tail element.

- ```rust
  pub async fn recv(self: &mut Self) -> Result<T, RecvError> { /* ... */ }
  ```
  Receives the next value for this receiver.

- ```rust
  pub fn try_recv(self: &mut Self) -> Result<T, TryRecvError> { /* ... */ }
  ```
  Attempts to return a pending value on this receiver without awaiting.

- ```rust
  pub fn blocking_recv(self: &mut Self) -> Result<T, RecvError> { /* ... */ }
  ```
  Blocking receive to call outside of asynchronous contexts.

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Functions

#### Function `channel`

**Attributes:**

- `#[track_caller]`

Create a bounded, multi-producer, multi-consumer channel where each sent
value is broadcasted to all active receivers.

**Note:** The actual capacity may be greater than the provided `capacity`.

All data sent on [`Sender`] will become available on every active
[`Receiver`] in the same order as it was sent.

The `Sender` can be cloned to `send` to the same channel from multiple
points in the process or it can be used concurrently from an `Arc`. New
`Receiver` handles are created by calling [`Sender::subscribe`].

If all [`Receiver`] handles are dropped, the `send` method will return a
[`SendError`]. Similarly, if all [`Sender`] handles are dropped, the [`recv`]
method will return a [`RecvError`].

[`Sender`]: crate::sync::broadcast::Sender
[`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
[`Receiver`]: crate::sync::broadcast::Receiver
[`recv`]: crate::sync::broadcast::Receiver::recv
[`SendError`]: crate::sync::broadcast::error::SendError
[`RecvError`]: crate::sync::broadcast::error::RecvError

# Examples

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

# Panics

This will panic if `capacity` is equal to `0` or larger
than `usize::MAX / 2`.

```rust
pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) { /* ... */ }
```

## Module `mpsc`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`
- `#[<cfg_attr>(not(feature = "sync"), allow(dead_code, unreachable_pub))]`

A multi-producer, single-consumer queue for sending values between
asynchronous tasks.

This module provides two variants of the channel: bounded and unbounded. The
bounded variant has a limit on the number of messages that the channel can
store, and if this limit is reached, trying to send another message will
wait until a message is received from the channel. An unbounded channel has
an infinite capacity, so the `send` method will always complete immediately.
This makes the [`UnboundedSender`] usable from both synchronous and
asynchronous code.

Similar to the `mpsc` channels provided by `std`, the channel constructor
functions provide separate send and receive handles, [`Sender`] and
[`Receiver`] for the bounded channel, [`UnboundedSender`] and
[`UnboundedReceiver`] for the unbounded channel. If there is no message to read,
the current task will be notified when a new value is sent. [`Sender`] and
[`UnboundedSender`] allow sending values into the channel. If the bounded
channel is at capacity, the send is rejected and the task will be notified
when additional capacity is available. In other words, the channel provides
backpressure.

This channel is also suitable for the single-producer single-consumer
use-case. (Unless you only need to send one message, in which case you
should use the [oneshot] channel.)

# Disconnection

When all [`Sender`] handles have been dropped, it is no longer
possible to send values into the channel. This is considered the termination
event of the stream. As such, `Receiver::poll` returns `Ok(Ready(None))`.

If the [`Receiver`] handle is dropped, then messages can no longer
be read out of the channel. In this case, all further attempts to send will
result in an error. Additionally, all unread messages will be drained from the
channel and dropped.

# Clean Shutdown

When the [`Receiver`] is dropped, it is possible for unprocessed messages to
remain in the channel. Instead, it is usually desirable to perform a "clean"
shutdown. To do this, the receiver first calls `close`, which will prevent
any further messages to be sent into the channel. Then, the receiver
consumes the channel to completion, at which point the receiver can be
dropped.

# Communicating between sync and async code

When you want to communicate between synchronous and asynchronous code, there
are two situations to consider:

**Bounded channel**: If you need a bounded channel, you should use a bounded
Tokio `mpsc` channel for both directions of communication. Instead of calling
the async [`send`][bounded-send] or [`recv`][bounded-recv] methods, in
synchronous code you will need to use the [`blocking_send`][blocking-send] or
[`blocking_recv`][blocking-recv] methods.

**Unbounded channel**: You should use the kind of channel that matches where
the receiver is. So for sending a message _from async to sync_, you should
use [the standard library unbounded channel][std-unbounded] or
[crossbeam][crossbeam-unbounded].  Similarly, for sending a message _from sync
to async_, you should use an unbounded Tokio `mpsc` channel.

Please be aware that the above remarks were written with the `mpsc` channel
in mind, but they can also be generalized to other kinds of channels. In
general, any channel method that isn't marked async can be called anywhere,
including outside of the runtime. For example, sending a message on a
[oneshot] channel from outside the runtime is perfectly fine.

# Multiple runtimes

The `mpsc` channel is runtime agnostic. You can freely move it between
different instances of the Tokio runtime or even use it from non-Tokio
runtimes.

When used in a Tokio runtime, it participates in
[cooperative scheduling](crate::task::coop#cooperative-scheduling) to avoid
starvation. This feature does not apply when used from non-Tokio runtimes.

As an exception, methods ending in `_timeout` are not runtime agnostic
because they require access to the Tokio timer. See the documentation of
each `*_timeout` method for more information on its use.

# Allocation behavior

<div class="warning">The implementation details described in this section may change in future
Tokio releases.</div>

The mpsc channel stores elements in blocks. Blocks are organized in a linked list. Sending
pushes new elements onto the block at the front of the list, and receiving pops them off the
one at the back. A block can hold 32 messages on a 64-bit target and 16 messages on a 32-bit
target. This number is independent of channel and message size. Each block also stores 4
pointer-sized values for bookkeeping (so on a 64-bit machine, each message has 1 byte of
overhead).

When all values in a block have been received, it becomes empty. It will then be freed, unless
the channel's first block (where newly-sent elements are being stored) has no next block. In
that case, the empty block is reused as the next block.

[`Sender`]: crate::sync::mpsc::Sender
[`Receiver`]: crate::sync::mpsc::Receiver
[bounded-send]: crate::sync::mpsc::Sender::send()
[bounded-recv]: crate::sync::mpsc::Receiver::recv()
[blocking-send]: crate::sync::mpsc::Sender::blocking_send()
[blocking-recv]: crate::sync::mpsc::Receiver::blocking_recv()
[`UnboundedSender`]: crate::sync::mpsc::UnboundedSender
[`UnboundedReceiver`]: crate::sync::mpsc::UnboundedReceiver
[oneshot]: crate::sync::oneshot
[`Handle::block_on`]: crate::runtime::Handle::block_on()
[std-unbounded]: std::sync::mpsc::channel
[crossbeam-unbounded]: https://docs.rs/crossbeam/*/crossbeam/channel/fn.unbounded.html
[`send_timeout`]: crate::sync::mpsc::Sender::send_timeout

```rust
pub mod mpsc { /* ... */ }
```

### Modules

## Module `error`

Channel error types.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `SendError`

Error returned by the `Sender`.

```rust
pub struct SendError<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SendError<T>) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SendError<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: SendError<T>) -> TrySendError<T> { /* ... */ }
    ```

#### Enum `TrySendError`

This enumeration is the list of the possible error outcomes for the
[`try_send`](super::Sender::try_send) method.

```rust
pub enum TrySendError<T> {
    Full(T),
    Closed(T),
}
```

##### Variants

###### `Full`

The data could not be sent on the channel because the channel is
currently full and sending would require blocking.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Closed`

The receive half of the channel was explicitly closed or has been
dropped.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consume the `TrySendError`, returning the unsent value.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TrySendError<T> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Eq**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TrySendError<T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: SendError<T>) -> TrySendError<T> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Enum `TryRecvError`

Error returned by `try_recv`.

```rust
pub enum TryRecvError {
    Empty,
    Disconnected,
}
```

##### Variants

###### `Empty`

This **channel** is currently empty, but the **Sender**(s) have not yet
disconnected, so data may yet become available.

###### `Disconnected`

The **channel**'s sending half has become disconnected, and there will
never be any more data received on it.

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **Copy**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryRecvError) -> bool { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryRecvError { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `SendTimeoutError`

**Attributes:**

- `#[<cfg>(feature = "time")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "time")))]`

Error returned by [`Sender::send_timeout`](super::Sender::send_timeout)].

```rust
pub enum SendTimeoutError<T> {
    Timeout(T),
    Closed(T),
}
```

##### Variants

###### `Timeout`

The data could not be sent on the channel because the channel is
full, and the timeout to send has elapsed.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Closed`

The receive half of the channel was explicitly closed or has been
dropped.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn into_inner(self: Self) -> T { /* ... */ }
  ```
  Consume the `SendTimeoutError`, returning the unsent value.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Error**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SendTimeoutError<T>) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SendTimeoutError<T> { /* ... */ }
    ```

### Re-exports

#### Re-export `channel`

```rust
pub use self::bounded::channel;
```

#### Re-export `OwnedPermit`

```rust
pub use self::bounded::OwnedPermit;
```

#### Re-export `Permit`

```rust
pub use self::bounded::Permit;
```

#### Re-export `PermitIterator`

```rust
pub use self::bounded::PermitIterator;
```

#### Re-export `Receiver`

```rust
pub use self::bounded::Receiver;
```

#### Re-export `Sender`

```rust
pub use self::bounded::Sender;
```

#### Re-export `WeakSender`

```rust
pub use self::bounded::WeakSender;
```

#### Re-export `unbounded_channel`

```rust
pub use self::unbounded::unbounded_channel;
```

#### Re-export `UnboundedReceiver`

```rust
pub use self::unbounded::UnboundedReceiver;
```

#### Re-export `UnboundedSender`

```rust
pub use self::unbounded::UnboundedSender;
```

#### Re-export `WeakUnboundedSender`

```rust
pub use self::unbounded::WeakUnboundedSender;
```

## Module `oneshot`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`
- `#[<cfg_attr>(not(feature = "sync"), allow(dead_code, unreachable_pub))]`

A one-shot channel is used for sending a single message between
asynchronous tasks. The [`channel`] function is used to create a
[`Sender`] and [`Receiver`] handle pair that form the channel.

The `Sender` handle is used by the producer to send the value.
The `Receiver` handle is used by the consumer to receive the value.

Each handle can be used on separate tasks.

Since the `send` method is not async, it can be used anywhere. This includes
sending between two runtimes, and using it from non-async code.

If the [`Receiver`] is closed before receiving a message which has already
been sent, the message will remain in the channel until the receiver is
dropped, at which point the message will be dropped immediately.

# Examples

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        drop(tx);
    });

    match rx.await {
        Ok(_) => panic!("This doesn't happen"),
        Err(_) => println!("the sender dropped"),
    }
}
```

To use a `oneshot` channel in a `tokio::select!` loop, add `&mut` in front of
the channel.

```
use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

#[tokio::main]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    let (send, mut recv) = oneshot::channel();
    let mut interval = interval(Duration::from_millis(100));

    # let handle =
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        send.send("shut down").unwrap();
    });

    loop {
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut recv => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
    # handle.await.unwrap();
}
```

To use a `Sender` from a destructor, put it in an [`Option`] and call
[`Option::take`].

```
use tokio::sync::oneshot;

struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

#[tokio::main]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread")]
async fn main() {
    let (send, recv) = oneshot::channel();

    let send_on_drop = SendOnDrop { sender: Some(send) };
    drop(send_on_drop);

    assert_eq!(recv.await, Ok("I got dropped!"));
}
```

```rust
pub mod oneshot { /* ... */ }
```

### Modules

## Module `error`

`Oneshot` error types.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `RecvError`

Error returned by the `Future` implementation for `Receiver`.

This error is returned by the receiver when the sender is dropped without sending.

```rust
pub struct RecvError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RecvError { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RecvError) -> bool { /* ... */ }
    ```

- **Error**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `TryRecvError`

Error returned by the `try_recv` function on `Receiver`.

```rust
pub enum TryRecvError {
    Empty,
    Closed,
}
```

##### Variants

###### `Empty`

The send half of the channel has not yet sent a value.

###### `Closed`

The send half of the channel was dropped without sending a value.

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryRecvError) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryRecvError { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
- **RefUnwindSafe**
### Types

#### Struct `Sender`

Sends a value to the associated [`Receiver`].

A pair of both a [`Sender`] and a [`Receiver`]  are created by the
[`channel`](fn@channel) function.

# Examples

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        drop(tx);
    });

    match rx.await {
        Ok(_) => panic!("This doesn't happen"),
        Err(_) => println!("the sender dropped"),
    }
}
```

To use a `Sender` from a destructor, put it in an [`Option`] and call
[`Option::take`].

```
use tokio::sync::oneshot;

struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

#[tokio::main]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread")]
async fn main() {
    let (send, recv) = oneshot::channel();

    let send_on_drop = SendOnDrop { sender: Some(send) };
    drop(send_on_drop);

    assert_eq!(recv.await, Ok("I got dropped!"));
}
```

[`Option`]: std::option::Option
[`Option::take`]: std::option::Option::take

```rust
pub struct Sender<T> {
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
  pub fn send(self: Self, t: T) -> Result<(), T> { /* ... */ }
  ```
  Attempts to send a value on this channel, returning it back if it could

- ```rust
  pub async fn closed(self: &mut Self) { /* ... */ }
  ```
  Waits for the associated [`Receiver`] handle to close.

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the associated [`Receiver`] handle has been dropped.

- ```rust
  pub fn poll_closed(self: &mut Self, cx: &mut Context<''_>) -> Poll<()> { /* ... */ }
  ```
  Checks whether the `oneshot` channel has been closed, and if not, schedules the

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Receiver`

Receives a value from the associated [`Sender`].

A pair of both a [`Sender`] and a [`Receiver`]  are created by the
[`channel`](fn@channel) function.

This channel has no `recv` method because the receiver itself implements the
[`Future`] trait. To receive a `Result<T, `[`error::RecvError`]`>`, `.await` the `Receiver` object directly.

The `poll` method on the `Future` trait is allowed to spuriously return
`Poll::Pending` even if the message has been sent. If such a spurious
failure happens, then the caller will be woken when the spurious failure has
been resolved so that the caller can attempt to receive the message again.
Note that receiving such a wakeup does not guarantee that the next call will
succeed — it could fail with another spurious failure. (A spurious failure
does not mean that the message is lost. It is just delayed.)

[`Future`]: trait@std::future::Future

# Examples

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        drop(tx);
    });

    match rx.await {
        Ok(_) => panic!("This doesn't happen"),
        Err(_) => println!("the sender dropped"),
    }
}
```

To use a `Receiver` in a `tokio::select!` loop, add `&mut` in front of the
channel.

```
use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

#[tokio::main]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    let (send, mut recv) = oneshot::channel();
    let mut interval = interval(Duration::from_millis(100));

    # let handle =
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        send.send("shut down").unwrap();
    });

    loop {
        tokio::select! {
            _ = interval.tick() => println!("Another 100ms"),
            msg = &mut recv => {
                println!("Got message: {}", msg.unwrap());
                break;
            }
        }
    }
    # handle.await.unwrap();
}
```

```rust
pub struct Receiver<T> {
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
  pub fn close(self: &mut Self) { /* ... */ }
  ```
  Prevents the associated [`Sender`] handle from sending a value.

- ```rust
  pub fn is_terminated(self: &Self) -> bool { /* ... */ }
  ```
  Checks if this receiver is terminated.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Checks if a channel is empty.

- ```rust
  pub fn try_recv(self: &mut Self) -> Result<T, TryRecvError> { /* ... */ }
  ```
  Attempts to receive a value.

- ```rust
  pub fn blocking_recv(self: Self) -> Result<T, RecvError> { /* ... */ }
  ```
  Blocking receive to call outside of asynchronous contexts.

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

- **Unpin**
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

### Functions

#### Function `channel`

**Attributes:**

- `#[track_caller]`

Creates a new one-shot channel for sending single values across asynchronous
tasks.

The function returns separate "send" and "receive" handles. The `Sender`
handle is used by the producer to send the value. The `Receiver` handle is
used by the consumer to receive the value.

Each handle can be used on separate tasks.

# Examples

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }
}
```

```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>) { /* ... */ }
```

## Module `watch`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`
- `#[<cfg_attr>(not(feature = "sync"), allow(dead_code, unreachable_pub))]`

A multi-producer, multi-consumer channel that only retains the *last* sent
value.

This channel is useful for watching for changes to a value from multiple
points in the code base, for example, changes to configuration values.

# Usage

[`channel`] returns a [`Sender`] / [`Receiver`] pair. These are the producer
and consumer halves of the channel. The channel is created with an initial
value.

Each [`Receiver`] independently tracks the last value *seen* by its caller.

To access the **current** value stored in the channel and mark it as *seen*
by a given [`Receiver`], use [`Receiver::borrow_and_update()`].

To access the current value **without** marking it as *seen*, use
[`Receiver::borrow()`]. (If the value has already been marked *seen*,
[`Receiver::borrow()`] is equivalent to [`Receiver::borrow_and_update()`].)

For more information on when to use these methods, see
[here](#borrow_and_update-versus-borrow).

## Change notifications

The [`Receiver`] half provides an asynchronous [`changed`] method. This
method is ready when a new, *unseen* value is sent via the [`Sender`] half.

* [`Receiver::changed()`] returns `Ok(())` on receiving a new value, or
  `Err(`[`error::RecvError`]`)` if the [`Sender`] has been dropped.
* If the current value is *unseen* when calling [`changed`], then
  [`changed`] will return immediately. If the current value is *seen*, then
  it will sleep until either a new message is sent via the [`Sender`] half,
  or the [`Sender`] is dropped.
* On completion, the [`changed`] method marks the new value as *seen*.
* At creation, the initial value is considered *seen*. In other words,
  [`Receiver::changed()`] will not return until a subsequent value is sent.
* New [`Receiver`] instances can be created with [`Sender::subscribe()`].
  The current value at the time the [`Receiver`] is created is considered
  *seen*.

## `borrow_and_update` versus `borrow`

If the receiver intends to await notifications from [`changed`] in a loop,
[`Receiver::borrow_and_update()`] should be preferred over
[`Receiver::borrow()`].  This avoids a potential race where a new value is
sent between [`changed`] being ready and the value being read. (If
[`Receiver::borrow()`] is used, the loop may run twice with the same value.)

If the receiver is only interested in the current value, and does not intend
to wait for changes, then [`Receiver::borrow()`] can be used. It may be more
convenient to use [`borrow`](Receiver::borrow) since it's an `&self`
method---[`borrow_and_update`](Receiver::borrow_and_update) requires `&mut
self`.

# Examples

The following example prints `hello! world! `.

```
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    // Use the equivalent of a "do-while" loop so the initial value is
    // processed before awaiting the `changed()` future.
    loop {
        println!("{}! ", *rx.borrow_and_update());
        if rx.changed().await.is_err() {
            break;
        }
    }
});

sleep(Duration::from_millis(100)).await;
tx.send("world")?;
# Ok(())
# }
```

# Closing

[`Sender::is_closed`] and [`Sender::closed`] allow the producer to detect
when all [`Receiver`] handles have been dropped. This indicates that there
is no further interest in the values being produced and work can be stopped.

The value in the channel will not be dropped until the sender and all
receivers have been dropped.

# Thread safety

Both [`Sender`] and [`Receiver`] are thread safe. They can be moved to other
threads and can be used in a concurrent environment. Clones of [`Receiver`]
handles may be moved to separate threads and also used concurrently.

[`Sender`]: crate::sync::watch::Sender
[`Receiver`]: crate::sync::watch::Receiver
[`changed`]: crate::sync::watch::Receiver::changed
[`Receiver::changed()`]: crate::sync::watch::Receiver::changed
[`Receiver::borrow()`]: crate::sync::watch::Receiver::borrow
[`Receiver::borrow_and_update()`]:
    crate::sync::watch::Receiver::borrow_and_update
[`channel`]: crate::sync::watch::channel
[`Sender::is_closed`]: crate::sync::watch::Sender::is_closed
[`Sender::closed`]: crate::sync::watch::Sender::closed
[`Sender::subscribe()`]: crate::sync::watch::Sender::subscribe

```rust
pub mod watch { /* ... */ }
```

### Modules

## Module `error`

Watch error types.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `SendError`

Error produced when sending a value fails.

```rust
pub struct SendError<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SendError<T>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **StructuralPartialEq**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SendError<T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
#### Struct `RecvError`

Error produced when receiving a change notification.

```rust
pub struct RecvError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **Error**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RecvError { /* ... */ }
    ```

### Types

#### Struct `Receiver`

Receives values from the associated [`Sender`](struct@Sender).

Instances are created by the [`channel`](fn@channel) function.

To turn this receiver into a `Stream`, you can use the [`WatchStream`]
wrapper.

[`WatchStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.WatchStream.html

```rust
pub struct Receiver<T> {
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
  pub fn borrow(self: &Self) -> Ref<''_, T> { /* ... */ }
  ```
  Returns a reference to the most recently sent value.

- ```rust
  pub fn borrow_and_update(self: &mut Self) -> Ref<''_, T> { /* ... */ }
  ```
  Returns a reference to the most recently sent value and marks that value

- ```rust
  pub fn has_changed(self: &Self) -> Result<bool, error::RecvError> { /* ... */ }
  ```
  Checks if this channel contains a message that this receiver has not yet

- ```rust
  pub fn mark_changed(self: &mut Self) { /* ... */ }
  ```
  Marks the state as changed.

- ```rust
  pub fn mark_unchanged(self: &mut Self) { /* ... */ }
  ```
  Marks the state as unchanged.

- ```rust
  pub async fn changed(self: &mut Self) -> Result<(), error::RecvError> { /* ... */ }
  ```
  Waits for a change notification, then marks the newest value as seen.

- ```rust
  pub async fn wait_for</* synthetic */ impl FnMut(&T) -> bool: FnMut(&T) -> bool>(self: &mut Self, f: impl FnMut(&T) -> bool) -> Result<Ref<''_, T>, error::RecvError> { /* ... */ }
  ```
  Waits for a value that satisfies the provided condition.

- ```rust
  pub fn same_channel(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if receivers belong to the same channel.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Sender`

Sends values to the associated [`Receiver`](struct@Receiver).

Instances are created by the [`channel`](fn@channel) function.

```rust
pub struct Sender<T> {
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
  pub fn new(init: T) -> Self { /* ... */ }
  ```
  Creates the sending-half of the [`watch`] channel.

- ```rust
  pub fn send(self: &Self, value: T) -> Result<(), error::SendError<T>> { /* ... */ }
  ```
  Sends a new value via the channel, notifying all receivers.

- ```rust
  pub fn send_modify<F>(self: &Self, modify: F)
where
    F: FnOnce(&mut T) { /* ... */ }
  ```
  Modifies the watched value **unconditionally** in-place,

- ```rust
  pub fn send_if_modified<F>(self: &Self, modify: F) -> bool
where
    F: FnOnce(&mut T) -> bool { /* ... */ }
  ```
  Modifies the watched value **conditionally** in-place,

- ```rust
  pub fn send_replace(self: &Self, value: T) -> T { /* ... */ }
  ```
  Sends a new value via the channel, notifying all receivers and returning

- ```rust
  pub fn borrow(self: &Self) -> Ref<''_, T> { /* ... */ }
  ```
  Returns a reference to the most recently sent value

- ```rust
  pub fn is_closed(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the channel has been closed. This happens when all receivers

- ```rust
  pub async fn closed(self: &Self) { /* ... */ }
  ```
  Completes when all receivers have dropped.

- ```rust
  pub fn subscribe(self: &Self) -> Receiver<T> { /* ... */ }
  ```
  Creates a new [`Receiver`] connected to this `Sender`.

- ```rust
  pub fn receiver_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of receivers that currently exist.

- ```rust
  pub fn sender_count(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of senders that currently exist.

- ```rust
  pub fn same_channel(self: &Self, other: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if senders belong to the same channel.

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
#### Struct `Ref`

Returns a reference to the inner value.

Outstanding borrows hold a read lock on the inner value. This means that
long-lived borrows could cause the producer half to block. It is recommended
to keep the borrow as short-lived as possible. Additionally, if you are
running in an environment that allows `!Send` futures, you must ensure that
the returned `Ref` type is never held alive across an `.await` point,
otherwise, it can lead to a deadlock.

The priority policy of the lock is dependent on the underlying lock
implementation, and this type does not guarantee that any particular policy
will be used. In particular, a producer which is waiting to acquire the lock
in `send` might or might not block concurrent calls to `borrow`, e.g.:

<details><summary>Potential deadlock example</summary>

```text
// Task 1 (on thread A)    |  // Task 2 (on thread B)
let _ref1 = rx.borrow();   |
                           |  // will block
                           |  let _ = tx.send(());
// may deadlock            |
let _ref2 = rx.borrow();   |
```
</details>

```rust
pub struct Ref<''a, T> {
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
  pub fn has_changed(self: &Self) -> bool { /* ... */ }
  ```
  Indicates if the borrowed value is considered as _changed_ since the last

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `channel`

Creates a new watch channel, returning the "send" and "receive" handles.

All values sent by [`Sender`] will become visible to the [`Receiver`] handles.
Only the last value sent is made available to the [`Receiver`] half. All
intermediate values are dropped.

# Examples

The following example prints `hello! world! `.

```
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    // Use the equivalent of a "do-while" loop so the initial value is
    // processed before awaiting the `changed()` future.
    loop {
        println!("{}! ", *rx.borrow_and_update());
        if rx.changed().await.is_err() {
            break;
        }
    }
});

sleep(Duration::from_millis(100)).await;
tx.send("world")?;
# Ok(())
# }
```

[`Sender`]: struct@Sender
[`Receiver`]: struct@Receiver

```rust
pub fn channel<T>(init: T) -> (Sender<T>, Receiver<T>) { /* ... */ }
```

### Re-exports

#### Re-export `Barrier`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use barrier::Barrier;
```

#### Re-export `BarrierWaitResult`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use barrier::BarrierWaitResult;
```

#### Re-export `Mutex`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::Mutex;
```

#### Re-export `MutexGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::MutexGuard;
```

#### Re-export `TryLockError`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::TryLockError;
```

#### Re-export `OwnedMutexGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::OwnedMutexGuard;
```

#### Re-export `MappedMutexGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::MappedMutexGuard;
```

#### Re-export `OwnedMappedMutexGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use mutex::OwnedMappedMutexGuard;
```

#### Re-export `Notify`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use notify::Notify;
```

#### Re-export `AcquireError`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use batch_semaphore::AcquireError;
```

#### Re-export `TryAcquireError`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use batch_semaphore::TryAcquireError;
```

#### Re-export `Semaphore`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use semaphore::Semaphore;
```

#### Re-export `SemaphorePermit`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use semaphore::SemaphorePermit;
```

#### Re-export `OwnedSemaphorePermit`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use semaphore::OwnedSemaphorePermit;
```

#### Re-export `RwLock`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::RwLock;
```

#### Re-export `OwnedRwLockReadGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::owned_read_guard::OwnedRwLockReadGuard;
```

#### Re-export `OwnedRwLockWriteGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::owned_write_guard::OwnedRwLockWriteGuard;
```

#### Re-export `OwnedRwLockMappedWriteGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::owned_write_guard_mapped::OwnedRwLockMappedWriteGuard;
```

#### Re-export `RwLockReadGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::read_guard::RwLockReadGuard;
```

#### Re-export `RwLockWriteGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::write_guard::RwLockWriteGuard;
```

#### Re-export `RwLockMappedWriteGuard`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use rwlock::write_guard_mapped::RwLockMappedWriteGuard;
```

#### Re-export `OnceCell`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use self::once_cell::OnceCell;
```

#### Re-export `SetError`

**Attributes:**

- `#[<cfg>(feature = "sync")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "sync")))]`

```rust
pub use self::once_cell::SetError;
```

## Module `task`

Asynchronous green-threads.

## What are Tasks?

A _task_ is a light weight, non-blocking unit of execution. A task is similar
to an OS thread, but rather than being managed by the OS scheduler, they are
managed by the [Tokio runtime][rt]. Another name for this general pattern is
[green threads]. If you are familiar with [Go's goroutines], [Kotlin's
coroutines], or [Erlang's processes], you can think of Tokio's tasks as
something similar.

Key points about tasks include:

* Tasks are **light weight**. Because tasks are scheduled by the Tokio
  runtime rather than the operating system, creating new tasks or switching
  between tasks does not require a context switch and has fairly low
  overhead. Creating, running, and destroying large numbers of tasks is
  quite cheap, especially compared to OS threads.

* Tasks are scheduled **cooperatively**. Most operating systems implement
  _preemptive multitasking_. This is a scheduling technique where the
  operating system allows each thread to run for a period of time, and then
  _preempts_ it, temporarily pausing that thread and switching to another.
  Tasks, on the other hand, implement _cooperative multitasking_. In
  cooperative multitasking, a task is allowed to run until it _yields_,
  indicating to the Tokio runtime's scheduler that it cannot currently
  continue executing. When a task yields, the Tokio runtime switches to
  executing the next task.

* Tasks are **non-blocking**. Typically, when an OS thread performs I/O or
  must synchronize with another thread, it _blocks_, allowing the OS to
  schedule another thread. When a task cannot continue executing, it must
  yield instead, allowing the Tokio runtime to schedule another task. Tasks
  should generally not perform system calls or other operations that could
  block a thread, as this would prevent other tasks running on the same
  thread from executing as well. Instead, this module provides APIs for
  running blocking operations in an asynchronous context.

[rt]: crate::runtime
[green threads]: https://en.wikipedia.org/wiki/Green_threads
[Go's goroutines]: https://tour.golang.org/concurrency/1
[Kotlin's coroutines]: https://kotlinlang.org/docs/reference/coroutines-overview.html
[Erlang's processes]: http://erlang.org/doc/getting_started/conc_prog.html#processes

## Working with Tasks

This module provides the following APIs for working with tasks:

### Spawning

Perhaps the most important function in this module is [`task::spawn`]. This
function can be thought of as an async equivalent to the standard library's
[`thread::spawn`][`std::thread::spawn`]. It takes an `async` block or other
[future], and creates a new task to run that work concurrently:

```
use tokio::task;

# async fn doc() {
task::spawn(async {
    // perform some work here...
});
# }
```

Like [`std::thread::spawn`], `task::spawn` returns a [`JoinHandle`] struct.
A `JoinHandle` is itself a future which may be used to await the output of
the spawned task. For example:

```
use tokio::task;

# #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
let join = task::spawn(async {
    // ...
    "hello world!"
});

// ...

// Await the result of the spawned task.
let result = join.await?;
assert_eq!(result, "hello world!");
# Ok(())
# }
```

Again, like `std::thread`'s [`JoinHandle` type][thread_join], if the spawned
task panics, awaiting its `JoinHandle` will return a [`JoinError`]. For
example:

```
use tokio::task;

# #[tokio::main] async fn main() {
let join = task::spawn(async {
    panic!("something bad happened!")
});

// The returned result indicates that the task failed.
assert!(join.await.is_err());
# }
```

`spawn`, `JoinHandle`, and `JoinError` are present when the "rt"
feature flag is enabled.

[`task::spawn`]: crate::task::spawn()
[future]: std::future::Future
[`std::thread::spawn`]: std::thread::spawn
[`JoinHandle`]: crate::task::JoinHandle
[thread_join]: std::thread::JoinHandle
[`JoinError`]: crate::task::JoinError

#### Cancellation

Spawned tasks may be cancelled using the [`JoinHandle::abort`] or
[`AbortHandle::abort`] methods. When one of these methods are called, the
task is signalled to shut down next time it yields at an `.await` point. If
the task is already idle, then it will be shut down as soon as possible
without running again before being shut down. Additionally, shutting down a
Tokio runtime (e.g. by returning from `#[tokio::main]`) immediately cancels
all tasks on it.

When tasks are shut down, it will stop running at whichever `.await` it has
yielded at. All local variables are destroyed by running their destructor.
Once shutdown has completed, awaiting the [`JoinHandle`] will fail with a
[cancelled error](crate::task::JoinError::is_cancelled).

Note that aborting a task does not guarantee that it fails with a cancelled
error, since it may complete normally first. For example, if the task does
not yield to the runtime at any point between the call to `abort` and the
end of the task, then the [`JoinHandle`] will instead report that the task
exited normally.

Be aware that tasks spawned using [`spawn_blocking`] cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking`
task, then this *will not have any effect*, and the task will continue
running normally. The exception is if the task has not started running
yet; in that case, calling `abort` may prevent the task from starting.

Be aware that calls to [`JoinHandle::abort`] just schedule the task for
cancellation, and will return before the cancellation has completed. To wait
for cancellation to complete, wait for the task to finish by awaiting the
[`JoinHandle`]. Similarly, the [`JoinHandle::is_finished`] method does not
return `true` until the cancellation has finished.

Calling [`JoinHandle::abort`] multiple times has the same effect as calling
it once.

Tokio also provides an [`AbortHandle`], which is like the [`JoinHandle`],
except that it does not provide a mechanism to wait for the task to finish.
Each task can only have one [`JoinHandle`], but it can have more than one
[`AbortHandle`].

[`JoinHandle::abort`]: crate::task::JoinHandle::abort
[`AbortHandle::abort`]: crate::task::AbortHandle::abort
[`AbortHandle`]: crate::task::AbortHandle
[`JoinHandle::is_finished`]: crate::task::JoinHandle::is_finished

### Blocking and Yielding

As we discussed above, code running in asynchronous tasks should not perform
operations that can block. A blocking operation performed in a task running
on a thread that is also running other tasks would block the entire thread,
preventing other tasks from running.

Instead, Tokio provides two APIs for running blocking operations in an
asynchronous context: [`task::spawn_blocking`] and [`task::block_in_place`].

Be aware that if you call a non-async method from async code, that non-async
method is still inside the asynchronous context, so you should also avoid
blocking operations there. This includes destructors of objects destroyed in
async code.

#### `spawn_blocking`

The `task::spawn_blocking` function is similar to the `task::spawn` function
discussed in the previous section, but rather than spawning an
_non-blocking_ future on the Tokio runtime, it instead spawns a
_blocking_ function on a dedicated thread pool for blocking tasks. For
example:

```
use tokio::task;

# async fn docs() {
task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
});
# }
```

Just like `task::spawn`, `task::spawn_blocking` returns a `JoinHandle`
which we can use to await the result of the blocking operation:

```rust
# use tokio::task;
# async fn docs() -> Result<(), Box<dyn std::error::Error>>{
let join = task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

let result = join.await?;
assert_eq!(result, "blocking completed");
# Ok(())
# }
```

#### `block_in_place`

When using the [multi-threaded runtime][rt-multi-thread], the [`task::block_in_place`]
function is also available. Like `task::spawn_blocking`, this function
allows running a blocking operation from an asynchronous context. Unlike
`spawn_blocking`, however, `block_in_place` works by transitioning the
_current_ worker thread to a blocking thread, moving other tasks running on
that thread to another worker thread. This can improve performance by avoiding
context switches.

For example:

```
use tokio::task;

# async fn docs() {
let result = task::block_in_place(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

assert_eq!(result, "blocking completed");
# }
```

#### `yield_now`

In addition, this module provides a [`task::yield_now`] async function
that is analogous to the standard library's [`thread::yield_now`]. Calling
and `await`ing this function will cause the current task to yield to the
Tokio runtime's scheduler, allowing other tasks to be
scheduled. Eventually, the yielding task will be polled again, allowing it
to execute. For example:

```rust
use tokio::task;

# #[tokio::main] async fn main() {
async {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");
}
# .await;
# }
```

[`task::spawn_blocking`]: crate::task::spawn_blocking
[`task::block_in_place`]: crate::task::block_in_place
[rt-multi-thread]: ../runtime/index.html#threaded-scheduler
[`task::yield_now`]: crate::task::yield_now()
[`thread::yield_now`]: std::thread::yield_now

```rust
pub mod task { /* ... */ }
```

### Modules

## Module `coop`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`
- `#[<cfg_attr>(not(feature = "full"), allow(dead_code))]`
- `#[<cfg_attr>(not(feature = "rt"), allow(unreachable_pub))]`

Utilities for improved cooperative scheduling.

### Cooperative scheduling

A single call to [`poll`] on a top-level task may potentially do a lot of
work before it returns `Poll::Pending`. If a task runs for a long period of
time without yielding back to the executor, it can starve other tasks
waiting on that executor to execute them, or drive underlying resources.
Since Rust does not have a runtime, it is difficult to forcibly preempt a
long-running task. Instead, this module provides an opt-in mechanism for
futures to collaborate with the executor to avoid starvation.

Consider a future like this one:

```
# use tokio_stream::{Stream, StreamExt};
async fn drop_all<I: Stream + Unpin>(mut input: I) {
    while let Some(_) = input.next().await {}
}
```

It may look harmless, but consider what happens under heavy load if the
input stream is _always_ ready. If we spawn `drop_all`, the task will never
yield, and will starve other tasks and resources on the same executor.

To account for this, Tokio has explicit yield points in a number of library
functions, which force tasks to return to the executor periodically.


#### unconstrained

If necessary, [`task::unconstrained`] lets you opt a future out of Tokio's cooperative
scheduling. When a future is wrapped with `unconstrained`, it will never be forced to yield to
Tokio. For example:

```
# #[tokio::main]
# async fn main() {
use tokio::{task, sync::mpsc};

let fut = async {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for i in 0..1000 {
        let _ = tx.send(());
        // This will always be ready. If coop was in effect, this code would be forced to yield
        // periodically. However, if left unconstrained, then this code will never yield.
        rx.recv().await;
    }
};

task::coop::unconstrained(fut).await;
# }
```
[`poll`]: method@std::future::Future::poll
[`task::unconstrained`]: crate::task::unconstrained()

```rust
pub mod coop { /* ... */ }
```

### Functions

#### Function `has_budget_remaining`

**Attributes:**

- `#[inline(always)]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

 Returns `true` if there is still budget left on the task.

 # Examples

 This example defines a `Timeout` future that requires a given `future` to complete before the
 specified duration elapses. If it does, its result is returned; otherwise, an error is returned
 and the future is canceled.

 Note that the future could exhaust the budget before we evaluate the timeout. Using `has_budget_remaining`,
 we can detect this scenario and ensure the timeout is always checked.

 ```
 # use std::future::Future;
 # use std::pin::{pin, Pin};
 # use std::task::{ready, Context, Poll};
 # use tokio::task::coop;
 # use tokio::time::Sleep;
 pub struct Timeout<T> {
     future: T,
     delay: Pin<Box<Sleep>>,
 }

 impl<T> Future for Timeout<T>
 where
     T: Future + Unpin,
 {
     type Output = Result<T::Output, ()>;

     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
         let this = Pin::into_inner(self);
         let future = Pin::new(&mut this.future);
         let delay = Pin::new(&mut this.delay);

         // check if the future is ready
         let had_budget_before = coop::has_budget_remaining();
         if let Poll::Ready(v) = future.poll(cx) {
             return Poll::Ready(Ok(v));
         }
         let has_budget_now = coop::has_budget_remaining();

         // evaluate the timeout
         if let (true, false) = (had_budget_before, has_budget_now) {
             // it is the underlying future that exhausted the budget
             ready!(pin!(coop::unconstrained(delay)).poll(cx));
         } else {
             ready!(delay.poll(cx));
         }
         return Poll::Ready(Err(()));
     }
 }
```

```rust
pub fn has_budget_remaining() -> bool { /* ... */ }
```

### Re-exports

#### Re-export `consume_budget`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use consume_budget::consume_budget;
```

#### Re-export `unconstrained`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use unconstrained::unconstrained;
```

#### Re-export `Unconstrained`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use unconstrained::Unconstrained;
```

## Module `futures`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

Task-related futures.

```rust
pub mod futures { /* ... */ }
```

### Re-exports

#### Re-export `TaskLocalFuture`

```rust
pub use super::task_local::TaskLocalFuture;
```

### Re-exports

#### Re-export `JoinError`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::JoinError;
```

#### Re-export `JoinHandle`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::JoinHandle;
```

#### Re-export `spawn_blocking`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use blocking::spawn_blocking;
```

#### Re-export `spawn`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use spawn::spawn;
```

#### Re-export `block_in_place`

**Attributes:**

- `#[<cfg>(feature = "rt-multi-thread")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt-multi-thread")))]`

```rust
pub use blocking::block_in_place;
```

#### Re-export `yield_now`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use yield_now::yield_now;
```

#### Re-export `spawn_local`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use local::spawn_local;
```

#### Re-export `LocalSet`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use local::LocalSet;
```

#### Re-export `LocalEnterGuard`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use local::LocalEnterGuard;
```

#### Re-export `LocalKey`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use task_local::LocalKey;
```

#### Re-export `JoinSet`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`
- `#[doc(inline)]`

```rust
pub use join_set::JoinSet;
```

#### Re-export `AbortHandle`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::AbortHandle;
```

#### Re-export `Id`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::Id;
```

#### Re-export `id`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::id;
```

#### Re-export `try_id`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use crate::runtime::task::try_id;
```

## Module `time`

**Attributes:**

- `#[<cfg>(feature = "time")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "time")))]`

Utilities for tracking time.

This module provides a number of types for executing code after a set period
of time.

* [`Sleep`] is a future that does no work and completes at a specific [`Instant`]
  in time.

* [`Interval`] is a stream yielding a value at a fixed period. It is
  initialized with a [`Duration`] and repeatedly yields each time the duration
  elapses.

* [`Timeout`]: Wraps a future or stream, setting an upper bound to the amount
  of time it is allowed to execute. If the future or stream does not
  complete in time, then it is canceled and an error is returned.

These types are sufficient for handling a large number of scenarios
involving time.

These types must be used from within the context of the [`Runtime`](crate::runtime::Runtime).

# Examples

Wait 100ms and print "100 ms have elapsed"

```
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    sleep(Duration::from_millis(100)).await;
    println!("100 ms have elapsed");
}
```

Require that an operation takes no more than 1s.

```
use tokio::time::{timeout, Duration};

async fn long_future() {
    // do work here
}

# async fn dox() {
let res = timeout(Duration::from_secs(1), long_future()).await;

if res.is_err() {
    println!("operation timed out");
}
# }
```

A simple example using [`interval`] to execute a task every two seconds.

The difference between [`interval`] and [`sleep`] is that an [`interval`]
measures the time since the last tick, which means that `.tick().await` may
wait for a shorter time than the duration specified for the interval
if some time has passed between calls to `.tick().await`.

If the tick in the example below was replaced with [`sleep`], the task
would only be executed once every three seconds, and not every two
seconds.

```
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(time::Duration::from_secs(2));
    for _i in 0..5 {
        interval.tick().await;
        task_that_takes_a_second().await;
    }
}
```

[`interval`]: crate::time::interval()
[`sleep`]: sleep()

```rust
pub mod time { /* ... */ }
```

### Modules

## Module `error`

Time error types.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `Error`

Errors encountered by the timer implementation.

Currently, there are two different errors that can occur:

* `shutdown` occurs when a timer operation is attempted, but the timer
  instance has been dropped. In this case, the operation will never be able
  to complete and the `shutdown` error is returned. This is a permanent
  error, i.e., once this error is observed, timer operations will never
  succeed in the future.

* `at_capacity` occurs when a timer operation is attempted, but the timer
  instance is currently handling its maximum number of outstanding sleep instances.
  In this case, the operation is not able to be performed at the current
  moment, and `at_capacity` is returned. This is a transient error, i.e., at
  some point in the future, if the operation is attempted again, it might
  succeed. Callers that observe this error should attempt to [shed load]. One
  way to do this would be dropping the future that issued the timer operation.

[shed load]: https://en.wikipedia.org/wiki/Load_Shedding

```rust
pub struct Error(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn shutdown() -> Error { /* ... */ }
  ```
  Creates an error representing a shutdown timer.

- ```rust
  pub fn is_shutdown(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the error was caused by the timer being shutdown.

- ```rust
  pub fn at_capacity() -> Error { /* ... */ }
  ```
  Creates an error representing a timer at capacity.

- ```rust
  pub fn is_at_capacity(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the error was caused by the timer being at capacity.

- ```rust
  pub fn invalid() -> Error { /* ... */ }
  ```
  Creates an error representing a misconfigured timer.

- ```rust
  pub fn is_invalid(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the error was caused by the timer being misconfigured.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Error**
- **RefUnwindSafe**
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

- **Send**
- **UnwindSafe**
- **Copy**
- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Elapsed`

Errors returned by `Timeout`.

This error is returned when a timeout expires before the function was able
to finish.

```rust
pub struct Elapsed(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **UnwindSafe**
- **Send**
- **Freeze**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Error**
- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Elapsed) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_err: Elapsed) -> std::io::Error { /* ... */ }
    ```

- **RefUnwindSafe**
### Re-exports

#### Re-export `Instant`

```rust
pub use self::instant::Instant;
```

#### Re-export `interval`

```rust
pub use interval::interval;
```

#### Re-export `interval_at`

```rust
pub use interval::interval_at;
```

#### Re-export `Interval`

```rust
pub use interval::Interval;
```

#### Re-export `MissedTickBehavior`

```rust
pub use interval::MissedTickBehavior;
```

#### Re-export `sleep`

```rust
pub use sleep::sleep;
```

#### Re-export `sleep_until`

```rust
pub use sleep::sleep_until;
```

#### Re-export `Sleep`

```rust
pub use sleep::Sleep;
```

#### Re-export `timeout`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use timeout::timeout;
```

#### Re-export `timeout_at`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use timeout::timeout_at;
```

#### Re-export `Timeout`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use timeout::Timeout;
```

#### Re-export `Duration`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::time::Duration;
```

## Module `stream`

Due to the `Stream` trait's inclusion in `std` landing later than Tokio's 1.0
release, most of the Tokio stream utilities have been moved into the [`tokio-stream`]
crate.

# Why was `Stream` not included in Tokio 1.0?

Originally, we had planned to ship Tokio 1.0 with a stable `Stream` type
but unfortunately the [RFC] had not been merged in time for `Stream` to
reach `std` on a stable compiler in time for the 1.0 release of Tokio. For
this reason, the team has decided to move all `Stream` based utilities to
the [`tokio-stream`] crate. While this is not ideal, once `Stream` has made
it into the standard library and the `MSRV` period has passed, we will implement
stream for our different types.

While this may seem unfortunate, not all is lost as you can get much of the
`Stream` support with `async/await` and `while let` loops. It is also possible
to create a `impl Stream` from `async fn` using the [`async-stream`] crate.

[`tokio-stream`]: https://docs.rs/tokio-stream
[`async-stream`]: https://docs.rs/async-stream
[RFC]: https://github.com/rust-lang/rfcs/pull/2996

# Example

Convert a [`sync::mpsc::Receiver`] to an `impl Stream`.

```rust,no_run
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<usize>(16);

let stream = async_stream::stream! {
    while let Some(item) = rx.recv().await {
        yield item;
    }
};
```

```rust
pub mod stream { /* ... */ }
```

## Macros

### Macro `pin`

**Attributes:**

- `#[macro_export]`

Pins a value on the stack.

Calls to `async fn` return anonymous [`Future`] values that are `!Unpin`.
These values must be pinned before they can be polled. Calling `.await` will
handle this, but consumes the future. If it is required to call `.await` on
a `&mut _` reference, the caller is responsible for pinning the future.

Pinning may be done by allocating with [`Box::pin`] or by using the stack
with the `pin!` macro.

The following will **fail to compile**:

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = my_async_fn();
    (&mut future).await;
}
```

To make this work requires pinning:

```
use tokio::pin;

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let future = my_async_fn();
    pin!(future);

    (&mut future).await;
}
```

Pinning is useful when using `select!` and stream operators that require `T:
Stream + Unpin`.

[`Future`]: trait@std::future::Future
[`Box::pin`]: std::boxed::Box::pin

# Usage

The `pin!` macro takes **identifiers** as arguments. It does **not** work
with expressions.

The following does not compile as an expression is passed to `pin!`.

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = pin!(my_async_fn());
    (&mut future).await;
}
```

# Examples

Using with select:

```
use tokio::{pin, select};
use tokio_stream::{self as stream, StreamExt};

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3, 4]);

    let future = my_async_fn();
    pin!(future);

    loop {
        select! {
            _ = &mut future => {
                // Stop looping `future` will be polled after completion
                break;
            }
            Some(val) = stream.next() => {
                println!("got value = {}", val);
            }
        }
    }
}
```

Because assigning to a variable followed by pinning is common, there is also
a variant of the macro that supports doing both in one go.

```
use tokio::{pin, select};

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    pin! {
        let future1 = my_async_fn();
        let future2 = my_async_fn();
    }

    select! {
        _ = &mut future1 => {}
        _ = &mut future2 => {}
    }
}
```

```rust
pub macro_rules! pin {
    /* macro_rules! pin {
    ($($x:ident),*) => { ... };
    ($(
            let $x:ident = $init:expr;
    )*) => { ... };
} */
}
```

### Macro `select`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

Waits on multiple concurrent branches, returning when the **first** branch
completes, cancelling the remaining branches.

The `select!` macro must be used inside of async functions, closures, and
blocks.

The `select!` macro accepts one or more branches with the following pattern:

```text
<pattern> = <async expression> (, if <precondition>)? => <handler>,
```

Additionally, the `select!` macro may include a single, optional `else`
branch, which evaluates if none of the other branches match their patterns:

```text
else => <expression>
```

The macro aggregates all `<async expression>` expressions and runs them
concurrently on the **current** task. Once the **first** expression
completes with a value that matches its `<pattern>`, the `select!` macro
returns the result of evaluating the completed branch's `<handler>`
expression.

Additionally, each branch may include an optional `if` precondition. If the
precondition returns `false`, then the branch is disabled. The provided
`<async expression>` is still evaluated but the resulting future is never
polled. This capability is useful when using `select!` within a loop.

The complete lifecycle of a `select!` expression is as follows:

1. Evaluate all provided `<precondition>` expressions. If the precondition
   returns `false`, disable the branch for the remainder of the current call
   to `select!`. Re-entering `select!` due to a loop clears the "disabled"
   state.
2. Aggregate the `<async expression>`s from each branch, including the
   disabled ones. If the branch is disabled, `<async expression>` is still
   evaluated, but the resulting future is not polled.
3. If **all** branches are disabled: go to step 6.
4. Concurrently await on the results for all remaining `<async expression>`s.
5. Once an `<async expression>` returns a value, attempt to apply the value to the
   provided `<pattern>`. If the pattern matches, evaluate the `<handler>` and return.
   If the pattern **does not** match, disable the current branch for the remainder of
   the current call to `select!`. Continue from step 3.
6. Evaluate the `else` expression. If no else expression is provided, panic.

# Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `select!`.

[`tokio::spawn`]: crate::spawn

# Fairness

By default, `select!` randomly picks a branch to check first. This provides
some level of fairness when calling `select!` in a loop with branches that
are always ready.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `select` to poll
the futures in the order they appear from top to bottom. There are a few
reasons you may want this:

- The random number generation of `tokio::select!` has a non-zero CPU cost
- Your futures may interact in a way where known polling order is significant

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the polling order of your futures is fair. If for example you
are selecting between a stream and a shutdown future, and the stream has a
huge volume of messages and zero or nearly zero time between them, you should
place the shutdown future earlier in the `select!` list to ensure that it is
always polled, and will not be ignored due to the stream being constantly
ready.

# Panics

The `select!` macro panics if all branches are disabled **and** there is no
provided `else` branch. A branch is disabled when the provided `if`
precondition returns `false` **or** when the pattern does not match the
result of `<async expression>`.

# Cancellation safety

When using `select!` in a loop to receive messages from multiple sources,
you should make sure that the receive call is cancellation safe to avoid
losing messages. This section goes through various common methods and
describes whether they are cancel safe.  The lists in this section are not
exhaustive.

The following methods are cancellation safe:

 * [`tokio::sync::mpsc::Receiver::recv`](crate::sync::mpsc::Receiver::recv)
 * [`tokio::sync::mpsc::UnboundedReceiver::recv`](crate::sync::mpsc::UnboundedReceiver::recv)
 * [`tokio::sync::broadcast::Receiver::recv`](crate::sync::broadcast::Receiver::recv)
 * [`tokio::sync::watch::Receiver::changed`](crate::sync::watch::Receiver::changed)
 * [`tokio::net::TcpListener::accept`](crate::net::TcpListener::accept)
 * [`tokio::net::UnixListener::accept`](crate::net::UnixListener::accept)
 * [`tokio::signal::unix::Signal::recv`](crate::signal::unix::Signal::recv)
 * [`tokio::io::AsyncReadExt::read`](crate::io::AsyncReadExt::read) on any `AsyncRead`
 * [`tokio::io::AsyncReadExt::read_buf`](crate::io::AsyncReadExt::read_buf) on any `AsyncRead`
 * [`tokio::io::AsyncWriteExt::write`](crate::io::AsyncWriteExt::write) on any `AsyncWrite`
 * [`tokio::io::AsyncWriteExt::write_buf`](crate::io::AsyncWriteExt::write_buf) on any `AsyncWrite`
 * [`tokio_stream::StreamExt::next`](https://docs.rs/tokio-stream/0.1/tokio_stream/trait.StreamExt.html#method.next) on any `Stream`
 * [`futures::stream::StreamExt::next`](https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html#method.next) on any `Stream`

The following methods are not cancellation safe and can lead to loss of data:

 * [`tokio::io::AsyncReadExt::read_exact`](crate::io::AsyncReadExt::read_exact)
 * [`tokio::io::AsyncReadExt::read_to_end`](crate::io::AsyncReadExt::read_to_end)
 * [`tokio::io::AsyncReadExt::read_to_string`](crate::io::AsyncReadExt::read_to_string)
 * [`tokio::io::AsyncWriteExt::write_all`](crate::io::AsyncWriteExt::write_all)

The following methods are not cancellation safe because they use a queue for
fairness and cancellation makes you lose your place in the queue:

 * [`tokio::sync::Mutex::lock`](crate::sync::Mutex::lock)
 * [`tokio::sync::RwLock::read`](crate::sync::RwLock::read)
 * [`tokio::sync::RwLock::write`](crate::sync::RwLock::write)
 * [`tokio::sync::Semaphore::acquire`](crate::sync::Semaphore::acquire)
 * [`tokio::sync::Notify::notified`](crate::sync::Notify::notified)

To determine whether your own methods are cancellation safe, look for the
location of uses of `.await`. This is because when an asynchronous method is
cancelled, that always happens at an `.await`. If your function behaves
correctly even if it is restarted while waiting at an `.await`, then it is
cancellation safe.

Cancellation safety can be defined in the following way: If you have a
future that has not yet completed, then it must be a no-op to drop that
future and recreate it. This definition is motivated by the situation where
a `select!` is used in a loop. Without this guarantee, you would lose your
progress when another branch completes and you restart the `select!` by
going around the loop.

Be aware that cancelling something that is not cancellation safe is not
necessarily wrong. For example, if you are cancelling a task because the
application is shutting down, then you probably don't care that partially
read data is lost.

# Examples

Basic select with two branches.

```
async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_stuff_async() => {
            println!("do_stuff_async() completed first")
        }
        _ = more_async_work() => {
            println!("more_async_work() completed first")
        }
    };
}
```

Basic stream selecting.

```
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let next = tokio::select! {
        v = stream1.next() => v.unwrap(),
        v = stream2.next() => v.unwrap(),
    };

    assert!(next == 1 || next == 4);
}
```

Collect the contents of two streams. In this example, we rely on pattern
matching and the fact that `stream::iter` is "fused", i.e. once the stream
is complete, all calls to `next()` return `None`.

```
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let mut values = vec![];

    loop {
        tokio::select! {
            Some(v) = stream1.next() => values.push(v),
            Some(v) = stream2.next() => values.push(v),
            else => break,
        }
    }

    values.sort();
    assert_eq!(&[1, 2, 3, 4, 5, 6], &values[..]);
}
```

Using the same future in multiple `select!` expressions can be done by passing
a reference to the future. Doing so requires the future to be [`Unpin`]. A
future can be made [`Unpin`] by either using [`Box::pin`] or stack pinning.

[`Unpin`]: std::marker::Unpin
[`Box::pin`]: std::boxed::Box::pin

Here, a stream is consumed for at most 1 second.

```
use tokio_stream::{self as stream, StreamExt};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3]);
    let sleep = time::sleep(Duration::from_secs(1));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            maybe_v = stream.next() => {
                if let Some(v) = maybe_v {
                    println!("got = {}", v);
                } else {
                    break;
                }
            }
            _ = &mut sleep => {
                println!("timeout");
                break;
            }
        }
    }
}
```

Joining two values using `select!`.

```
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = oneshot::channel();
    let (tx2, mut rx2) = oneshot::channel();

    tokio::spawn(async move {
        tx1.send("first").unwrap();
    });

    tokio::spawn(async move {
        tx2.send("second").unwrap();
    });

    let mut a = None;
    let mut b = None;

    while a.is_none() || b.is_none() {
        tokio::select! {
            v1 = (&mut rx1), if a.is_none() => a = Some(v1.unwrap()),
            v2 = (&mut rx2), if b.is_none() => b = Some(v2.unwrap()),
        }
    }

    let res = (a.unwrap(), b.unwrap());

    assert_eq!(res.0, "first");
    assert_eq!(res.1, "second");
}
```

Using the `biased;` mode to control polling order.

```
#[tokio::main]
async fn main() {
    let mut count = 0u8;

    loop {
        tokio::select! {
            // If you run this example without `biased;`, the polling order is
            // pseudo-random, and the assertions on the value of count will
            // (probably) fail.
            biased;

            _ = async {}, if count < 1 => {
                count += 1;
                assert_eq!(count, 1);
            }
            _ = async {}, if count < 2 => {
                count += 1;
                assert_eq!(count, 2);
            }
            _ = async {}, if count < 3 => {
                count += 1;
                assert_eq!(count, 3);
            }
            _ = async {}, if count < 4 => {
                count += 1;
                assert_eq!(count, 4);
            }

            else => {
                break;
            }
        };
    }
}
```

## Avoid racy `if` preconditions

Given that `if` preconditions are used to disable `select!` branches, some
caution must be used to avoid missing values.

For example, here is **incorrect** usage of `sleep` with `if`. The objective
is to repeatedly run an asynchronous task for up to 50 milliseconds.
However, there is a potential for the `sleep` completion to be missed.

```no_run,should_panic
use tokio::time::{self, Duration};

async fn some_async_work() {
    // do work
}

#[tokio::main]
async fn main() {
    let sleep = time::sleep(Duration::from_millis(50));
    tokio::pin!(sleep);

    while !sleep.is_elapsed() {
        tokio::select! {
            _ = &mut sleep, if !sleep.is_elapsed() => {
                println!("operation timed out");
            }
            _ = some_async_work() => {
                println!("operation completed");
            }
        }
    }

    panic!("This example shows how not to do it!");
}
```

In the above example, `sleep.is_elapsed()` may return `true` even if
`sleep.poll()` never returned `Ready`. This opens up a potential race
condition where `sleep` expires between the `while !sleep.is_elapsed()`
check and the call to `select!` resulting in the `some_async_work()` call to
run uninterrupted despite the sleep having elapsed.

One way to write the above example without the race would be:

```
use tokio::time::{self, Duration};

async fn some_async_work() {
# time::sleep(Duration::from_millis(10)).await;
    // do work
}

#[tokio::main]
async fn main() {
    let sleep = time::sleep(Duration::from_millis(50));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            _ = &mut sleep => {
                println!("operation timed out");
                break;
            }
            _ = some_async_work() => {
                println!("operation completed");
            }
        }
    }
}
```
# Alternatives from the Ecosystem

The `select!` macro is a powerful tool for managing multiple asynchronous
branches, enabling tasks to run concurrently within the same thread. However,
its use can introduce challenges, particularly around cancellation safety, which
can lead to subtle and hard-to-debug errors. For many use cases, ecosystem
alternatives may be preferable as they mitigate these concerns by offering
clearer syntax, more predictable control flow, and reducing the need to manually
handle issues like fuse semantics or cancellation safety.

## Merging Streams

For cases where `loop { select! { ... } }` is used to poll multiple tasks,
stream merging offers a concise alternative, inherently handle cancellation-safe
processing, removing the risk of data loss. Libraries such as [`tokio_stream`],
[`futures::stream`] and [`futures_concurrency`] provide tools for merging
streams and handling their outputs sequentially.

[`tokio_stream`]: https://docs.rs/tokio-stream/latest/tokio_stream/
[`futures::stream`]: https://docs.rs/futures/latest/futures/stream/
[`futures_concurrency`]: https://docs.rs/futures-concurrency/latest/futures_concurrency/

### Example with `select!`

```
struct File;
struct Channel;
struct Socket;

impl Socket {
    async fn read_packet(&mut self) -> Vec<u8> {
        vec![]
    }
}

async fn read_send(_file: &mut File, _channel: &mut Channel) {
    // do work that is not cancel safe
}

#[tokio::main]
async fn main() {
    // open our IO types
    let mut file = File;
    let mut channel = Channel;
    let mut socket = Socket;

    loop {
        tokio::select! {
            _ = read_send(&mut file, &mut channel) => { /* ... */ },
            _data = socket.read_packet() => { /* ... */ }
            _ = futures::future::ready(()) => break
        }
    }
}

```

### Moving to `merge`

By using merge, you can unify multiple asynchronous tasks into a single stream,
eliminating the need to manage tasks manually and reducing the risk of
unintended behavior like data loss.

```
use std::pin::pin;

use futures::stream::unfold;
use tokio_stream::StreamExt;

struct File;
struct Channel;
struct Socket;

impl Socket {
    async fn read_packet(&mut self) -> Vec<u8> {
        vec![]
    }
}

async fn read_send(_file: &mut File, _channel: &mut Channel) {
    // do work that is not cancel safe
}

enum Message {
    Stop,
    Sent,
    Data(Vec<u8>),
}

#[tokio::main]
async fn main() {
    // open our IO types
    let file = File;
    let channel = Channel;
    let socket = Socket;

    let a = unfold((file, channel), |(mut file, mut channel)| async {
        read_send(&mut file, &mut channel).await;
        Some((Message::Sent, (file, channel)))
    });
    let b = unfold(socket, |mut socket| async {
        let data = socket.read_packet().await;
        Some((Message::Data(data), socket))
    });
    let c = tokio_stream::iter([Message::Stop]);

    let mut s = pin!(a.merge(b).merge(c));
    while let Some(msg) = s.next().await {
        match msg {
            Message::Data(_data) => { /* ... */ }
            Message::Sent => continue,
            Message::Stop => break,
        }
    }
}
```

## Racing Futures

If you need to wait for the first completion among several asynchronous tasks,
ecosystem utilities such as
[`futures`](https://docs.rs/futures/latest/futures/),
[`futures-lite`](https://docs.rs/futures-lite/latest/futures_lite/) or
[`futures-concurrency`](https://docs.rs/futures-concurrency/latest/futures_concurrency/)
provide streamlined syntax for racing futures:

- [`futures_concurrency::future::Race`](https://docs.rs/futures-concurrency/latest/futures_concurrency/future/trait.Race.html)
- [`futures::select`](https://docs.rs/futures/latest/futures/macro.select.html)
- [`futures::stream::select_all`](https://docs.rs/futures/latest/futures/stream/select_all/index.html) (for streams)
- [`futures_lite::future::or`](https://docs.rs/futures-lite/latest/futures_lite/future/fn.or.html)
- [`futures_lite::future::race`](https://docs.rs/futures-lite/latest/futures_lite/future/fn.race.html)

```
use futures_concurrency::future::Race;

#[tokio::main]
async fn main() {
    let task_a = async { Ok("ok") };
    let task_b = async { Err("error") };
    let result = (task_a, task_b).race().await;

    match result {
        Ok(output) => println!("First task completed with: {output}"),
        Err(err) => eprintln!("Error occurred: {err}"),
    }
}
```

```rust
pub macro_rules! select {
    /* macro_rules! select {
    {
        $(
            biased;
        )?
        $(
            $bind:pat = $fut:expr $(, if $cond:expr)? => $handler:expr,
        )*
        $(
            else => $els:expr $(,)?
        )?
    } => { ... };
} */
}
```

### Macro `join`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

Waits on multiple concurrent branches, returning when **all** branches
complete.

The `join!` macro must be used inside of async functions, closures, and
blocks.

The `join!` macro takes a list of async expressions and evaluates them
concurrently on the same task. Each async expression evaluates to a future
and the futures from each expression are multiplexed on the current task.

When working with async expressions returning `Result`, `join!` will wait
for **all** branches complete regardless if any complete with `Err`. Use
[`try_join!`] to return early when `Err` is encountered.

[`try_join!`]: crate::try_join

# Notes

The supplied futures are stored inline and do not require allocating a
`Vec`.

### Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `join!`.

[`tokio::spawn`]: crate::spawn

# Examples

Basic join with two branches

```
async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

#[tokio::main]
async fn main() {
    let (first, second) = tokio::join!(
        do_stuff_async(),
        more_async_work());

    // do something with the values
}
```

```rust
pub macro_rules! join {
    /* macro_rules! join {
    ($($future:expr),*) => { ... };
} */
}
```

### Macro `try_join`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`

Waits on multiple concurrent branches, returning when **all** branches
complete with `Ok(_)` or on the first `Err(_)`.

The `try_join!` macro must be used inside of async functions, closures, and
blocks.

Similar to [`join!`], the `try_join!` macro takes a list of async
expressions and evaluates them concurrently on the same task. Each async
expression evaluates to a future and the futures from each expression are
multiplexed on the current task. The `try_join!` macro returns when **all**
branches return with `Ok` or when the **first** branch returns with `Err`.

[`join!`]: macro@join

# Notes

The supplied futures are stored inline and do not require allocating a
`Vec`.

### Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `try_join!`.

[`tokio::spawn`]: crate::spawn

# Examples

Basic `try_join` with two branches.

```
async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
# Ok(())
}

async fn more_async_work() -> Result<(), &'static str> {
    // more here
# Ok(())
}

#[tokio::main]
async fn main() {
    let res = tokio::try_join!(
        do_stuff_async(),
        more_async_work());

    match res {
         Ok((first, second)) => {
             // do something with the values
         }
         Err(err) => {
            println!("processing failed; error = {}", err);
         }
    }
}
```

Using `try_join!` with spawned tasks.

```
use tokio::task::JoinHandle;

async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
# Err("failed")
}

async fn more_async_work() -> Result<(), &'static str> {
    // more here
# Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(err) => Err("handling failed"),
    }
}

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(do_stuff_async());
    let handle2 = tokio::spawn(more_async_work());
    match tokio::try_join!(flatten(handle1), flatten(handle2)) {
        Ok(val) => {
            // do something with the values
        }
        Err(err) => {
            println!("Failed with {}.", err);
            # assert_eq!(err, "failed");
        }
    }
}
```

```rust
pub macro_rules! try_join {
    /* macro_rules! try_join {
    ($($future:expr),*) => { ... };
} */
}
```

### Macro `task_local`

**Attributes:**

- `#[macro_export]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

Declares a new task-local key of type [`tokio::task::LocalKey`].

# Syntax

The macro wraps any number of static declarations and makes them local to the current task.
Publicity and attributes for each static is preserved. For example:

# Examples

```
# use tokio::task_local;
task_local! {
    pub static ONE: u32;

    #[allow(unused)]
    static TWO: f32;
}
# fn main() {}
```

See [`LocalKey` documentation][`tokio::task::LocalKey`] for more
information.

[`tokio::task::LocalKey`]: struct@crate::task::LocalKey

```rust
pub macro_rules! task_local {
    /* macro_rules! task_local {
    () => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty) => { ... };
} */
}
```

## Re-exports

### Re-export `spawn`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`

```rust
pub use task::spawn;
```

### Re-export `main`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`
- `#[<cfg>(feature = "rt-multi-thread")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`
- `#[doc(inline)]`

```rust
pub use tokio_macros::main;
```

### Re-export `test`

**Attributes:**

- `#[<cfg>(feature = "rt")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "rt")))]`
- `#[<cfg>(feature = "rt-multi-thread")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "macros")))]`
- `#[doc(inline)]`

```rust
pub use tokio_macros::test;
```

