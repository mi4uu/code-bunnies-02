# Crate Documentation

**Version:** 1.4.2

**Format Version:** 43

# Module `signal_hook_registry`

Backend of the [signal-hook] crate.

The [signal-hook] crate tries to provide an API to the unix signals, which are a global
resource. Therefore, it is desirable an application contains just one version of the crate
which manages this global resource. But that makes it impossible to make breaking changes in
the API.

Therefore, this crate provides very minimal and low level API to the signals that is unlikely
to have to change, while there may be multiple versions of the [signal-hook] that all use this
low-level API to provide different versions of the high level APIs.

It is also possible some other crates might want to build a completely different API. This
split allows these crates to still reuse the same low-level routines in this crate instead of
going to the (much more dangerous) unix calls.

# What this crate provides

The only thing this crate does is multiplexing the signals. An application or library can add
or remove callbacks and have multiple callbacks for the same signal.

It handles dispatching the callbacks and managing them in a way that uses only the
[async-signal-safe] functions inside the signal handler. Note that the callbacks are still run
inside the signal handler, so it is up to the caller to ensure they are also
[async-signal-safe].

# What this is for

This is a building block for other libraries creating reasonable abstractions on top of
signals. The [signal-hook] is the generally preferred way if you need to handle signals in your
application and provides several safe patterns of doing so.

# Rust version compatibility

Currently builds on 1.26.0 an newer and this is very unlikely to change. However, tests
require dependencies that don't build there, so tests need newer Rust version (they are run on
stable).

# Portability

This crate includes a limited support for Windows, based on `signal`/`raise` in the CRT.
There are differences in both API and behavior:

- Due to lack of `siginfo_t`, we don't provide `register_sigaction` or `register_unchecked`.
- Due to lack of signal blocking, there's a race condition.
  After the call to `signal`, there's a moment where we miss a signal.
  That means when you register a handler, there may be a signal which invokes
  neither the default handler or the handler you register.
- Handlers registered by `signal` in Windows are cleared on first signal.
  To match behavior in other platforms, we re-register the handler each time the handler is
  called, but there's a moment where we miss a handler.
  That means when you receive two signals in a row, there may be a signal which invokes
  the default handler, nevertheless you certainly have registered the handler.

[signal-hook]: https://docs.rs/signal-hook
[async-signal-safe]: http://www.man7.org/linux/man-pages/man7/signal-safety.7.html

## Types

### Struct `SigId`

An ID of registered action.

This is returned by all the registration routines and can be used to remove the action later on
with a call to [`unregister`].

```rust
pub struct SigId {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SigId) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SigId { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SigId) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Unpin**
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

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SigId) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
## Functions

### Function `register`

Registers an arbitrary action for the given signal.

This makes sure there's a signal handler for the given signal. It then adds the action to the
ones called each time the signal is delivered. If multiple actions are set for the same signal,
all are called, in the order of registration.

If there was a previous signal handler for the given signal, it is chained ‒ it will be called
as part of this library's signal handler, before any actions set through this function.

On success, the function returns an ID that can be used to remove the action again with
[`unregister`].

# Panics

If the signal is one of (see [`FORBIDDEN`]):

* `SIGKILL`
* `SIGSTOP`
* `SIGILL`
* `SIGFPE`
* `SIGSEGV`

The first two are not possible to override (and the underlying C functions simply ignore all
requests to do so, which smells of possible bugs, or return errors). The rest can be set, but
generally needs very special handling to do so correctly (direct manipulation of the
application's address space, `longjmp` and similar). Unless you know very well what you're
doing, you'll shoot yourself into the foot and this library won't help you with that.

# Errors

Since the library manipulates signals using the low-level C functions, all these can return
errors. Generally, the errors mean something like the specified signal does not exist on the
given platform ‒ after a program is debugged and tested on a given OS, it should never return
an error.

However, if an error *is* returned, there are no guarantees if the given action was registered
or not.

# Safety

This function is unsafe, because the `action` is run inside a signal handler. The set of
functions allowed to be called from within is very limited (they are called async-signal-safe
functions by POSIX). These specifically do *not* contain mutexes and memory
allocation/deallocation. They *do* contain routines to terminate the program, to further
manipulate signals (by the low-level functions, not by this library) and to read and write file
descriptors. Calling program's own functions consisting only of these is OK, as is manipulating
program's variables ‒ however, as the action can be called on any thread that does not have the
given signal masked (by default no signal is masked on any thread), and mutexes are a no-go,
this is harder than it looks like at first.

As panicking from within a signal handler would be a panic across FFI boundary (which is
undefined behavior), the passed handler must not panic.

If you find these limitations hard to satisfy, choose from the helper functions in the
[signal-hook](https://docs.rs/signal-hook) crate ‒ these provide safe interface to use some
common signal handling patters.

# Race condition

Upon registering the first hook for a given signal into this library, there's a short race
condition under the following circumstances:

* The program already has a signal handler installed for this particular signal (through some
  other library, possibly).
* Concurrently, some other thread installs a different signal handler while it is being
  installed by this library.
* At the same time, the signal is delivered.

Under such conditions signal-hook might wrongly "chain" to the older signal handler for a short
while (until the registration is fully complete).

Note that the exact conditions of the race condition might change in future versions of the
library. The recommended way to avoid it is to register signals before starting any additional
threads, or at least not to register signals concurrently.

Alternatively, make sure all signals are handled through this library.

# Performance

Even when it is possible to repeatedly install and remove actions during the lifetime of a
program, the installation and removal is considered a slow operation and should not be done
very often. Also, there's limited (though huge) amount of distinct IDs (they are `u128`).

# Examples

```rust
extern crate signal_hook_registry;

use std::io::Error;
use std::process;

fn main() -> Result<(), Error> {
    let signal = unsafe {
        signal_hook_registry::register(signal_hook::consts::SIGTERM, || process::abort())
    }?;
    // Stuff here...
    signal_hook_registry::unregister(signal); // Not really necessary.
    Ok(())
}
```

```rust
pub unsafe fn register<F>(signal: libc::c_int, action: F) -> Result<SigId, std::io::Error>
where
    F: Fn() + Sync + Send + ''static { /* ... */ }
```

### Function `register_sigaction`

**Attributes:**

- `#[<cfg>(not(windows))]`

Register a signal action.

This acts in the same way as [`register`], including the drawbacks, panics and performance
characteristics. The only difference is the provided action accepts a [`siginfo_t`] argument,
providing information about the received signal.

# Safety

See the details of [`register`].

```rust
pub unsafe fn register_sigaction<F>(signal: libc::c_int, action: F) -> Result<SigId, std::io::Error>
where
    F: Fn(&libc::siginfo_t) + Sync + Send + ''static { /* ... */ }
```

### Function `register_signal_unchecked`

Register a signal action without checking for forbidden signals.

This acts in the same way as [`register_unchecked`], including the drawbacks, panics and
performance characteristics. The only difference is the provided action doesn't accept a
[`siginfo_t`] argument.

# Safety

See the details of [`register`].

```rust
pub unsafe fn register_signal_unchecked<F>(signal: libc::c_int, action: F) -> Result<SigId, std::io::Error>
where
    F: Fn() + Sync + Send + ''static { /* ... */ }
```

### Function `register_unchecked`

**Attributes:**

- `#[<cfg>(not(windows))]`

Register a signal action without checking for forbidden signals.

This acts the same way as [`register_sigaction`], but without checking for the [`FORBIDDEN`]
signals. All the signals passed are registered and it is up to the caller to make some sense of
them.

Note that you really need to know what you're doing if you change eg. the `SIGSEGV` signal
handler. Generally, you don't want to do that. But unlike the other functions here, this
function still allows you to do it.

# Safety

See the details of [`register`].

```rust
pub unsafe fn register_unchecked<F>(signal: libc::c_int, action: F) -> Result<SigId, std::io::Error>
where
    F: Fn(&libc::siginfo_t) + Sync + Send + ''static { /* ... */ }
```

### Function `unregister`

Removes a previously installed action.

This function does nothing if the action was already removed. It returns true if it was removed
and false if the action wasn't found.

It can unregister all the actions installed by [`register`] as well as the ones from downstream
crates (like [`signal-hook`](https://docs.rs/signal-hook)).

# Warning

This does *not* currently return the default/previous signal handler if the last action for a
signal was just unregistered. That means that if you replaced for example `SIGTERM` and then
removed the action, the program will effectively ignore `SIGTERM` signals from now on, not
terminate on them as is the default action. This is OK if you remove it as part of a shutdown,
but it is not recommended to remove termination actions during the normal runtime of
application (unless the desired effect is to create something that can be terminated only by
SIGKILL).

```rust
pub fn unregister(id: SigId) -> bool { /* ... */ }
```

## Constants and Statics

### Constant `FORBIDDEN`

List of forbidden signals.

Some signals are impossible to replace according to POSIX and some are so special that this
library refuses to handle them (eg. SIGSEGV). The routines panic in case registering one of
these signals is attempted.

See [`register`].

```rust
pub const FORBIDDEN: &[libc::c_int] = FORBIDDEN_IMPL;
```

