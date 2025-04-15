# Crate Documentation

**Version:** 0.5.0

**Format Version:** 43

# Module `pretty_env_logger`

A logger configured via an environment variable which writes to standard
error with nice colored output for log levels.

## Example

```
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
```

Run the program with the environment variable `RUST_LOG=trace`.

## Defaults

The defaults can be setup by calling `init()` or `try_init()` at the start
of the program.

## Enable logging

This crate uses [env_logger][] internally, so the same ways of enabling
logs through an environment variable are supported.

[env_logger]: https://docs.rs/env_logger

## Functions

### Function `init`

Initializes the global logger with a pretty env logger.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Panics

This function fails to set the global logger if one has already been set.

```rust
pub fn init() { /* ... */ }
```

### Function `init_timed`

Initializes the global logger with a timed pretty env logger.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Panics

This function fails to set the global logger if one has already been set.

```rust
pub fn init_timed() { /* ... */ }
```

### Function `try_init`

Initializes the global logger with a pretty env logger.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Errors

This function fails to set the global logger if one has already been set.

```rust
pub fn try_init() -> Result<(), log::SetLoggerError> { /* ... */ }
```

### Function `try_init_timed`

Initializes the global logger with a timed pretty env logger.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Errors

This function fails to set the global logger if one has already been set.

```rust
pub fn try_init_timed() -> Result<(), log::SetLoggerError> { /* ... */ }
```

### Function `init_custom_env`

Initialized the global logger with a pretty env logger, with a custom variable name.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Panics

This function fails to set the global logger if one has already been set.

```rust
pub fn init_custom_env(environment_variable_name: &str) { /* ... */ }
```

### Function `try_init_custom_env`

Initialized the global logger with a pretty env logger, with a custom variable name.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Errors

This function fails to set the global logger if one has already been set.

```rust
pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> { /* ... */ }
```

### Function `try_init_timed_custom_env`

Initialized the global logger with a timed pretty env logger, with a custom variable name.

This should be called early in the execution of a Rust program, and the
global logger may only be initialized once. Future initialization attempts
will return an error.

# Errors

This function fails to set the global logger if one has already been set.

```rust
pub fn try_init_timed_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> { /* ... */ }
```

### Function `formatted_builder`

Returns a `env_logger::Builder` for further customization.

This method will return a colored and formatted `env_logger::Builder`
for further customization. Refer to env_logger::Build crate documentation
for further details and usage.

```rust
pub fn formatted_builder() -> env_logger::Builder { /* ... */ }
```

### Function `formatted_timed_builder`

Returns a `env_logger::Builder` for further customization.

This method will return a colored and time formatted `env_logger::Builder`
for further customization. Refer to env_logger::Build crate documentation
for further details and usage.

```rust
pub fn formatted_timed_builder() -> env_logger::Builder { /* ... */ }
```

