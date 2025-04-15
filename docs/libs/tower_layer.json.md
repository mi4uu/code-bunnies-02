# Crate Documentation

**Version:** 0.3.3

**Format Version:** 43

# Module `tower_layer`

Layer traits and extensions.

A layer decorates an service and provides additional functionality. It
allows other services to be composed with the service that implements layer.

A middleware implements the [`Layer`] and [`Service`] trait.

[`Service`]: https://docs.rs/tower/*/tower/trait.Service.html

## Traits

### Trait `Layer`

Decorates a [`Service`], transforming either the request or the response.

Often, many of the pieces needed for writing network applications can be
reused across multiple services. The `Layer` trait can be used to write
reusable components that can be applied to very different kinds of services;
for example, it can be applied to services operating on different protocols,
and to both the client and server side of a network transaction.

# Log

Take request logging as an example:

```rust
# use tower_service::Service;
# use std::task::{Poll, Context};
# use tower_layer::Layer;
# use std::fmt;

pub struct LogLayer {
    target: &'static str,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

// This service implements the Log behavior
pub struct LogService<S> {
    target: &'static str,
    service: S,
}

impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request>,
    Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}
```

The above log implementation is decoupled from the underlying protocol and
is also decoupled from client or server concerns. In other words, the same
log middleware could be used in either a client or a server.

[`Service`]: https://docs.rs/tower/*/tower/trait.Service.html

```rust
pub trait Layer<S> {
    /* Associated items */
}
```

#### Required Items

##### Associated Types

- `Service`: The wrapped service

##### Required Methods

- `layer`: Wrap the given service with the middleware, returning a new service

#### Implementations

This trait is implemented for the following types:

- `Identity` with <S>
- `LayerFn<F>` with <F, S, Out>
- `Stack<Inner, Outer>` with <S, Inner, Outer>
- `()` with <S>
- `(L1)` with <S, L1>
- `(L1, L2)` with <S, L1, L2>
- `(L1, L2, L3)` with <S, L1, L2, L3>
- `(L1, L2, L3, L4)` with <S, L1, L2, L3, L4>
- `(L1, L2, L3, L4, L5)` with <S, L1, L2, L3, L4, L5>
- `(L1, L2, L3, L4, L5, L6)` with <S, L1, L2, L3, L4, L5, L6>
- `(L1, L2, L3, L4, L5, L6, L7)` with <S, L1, L2, L3, L4, L5, L6, L7>
- `(L1, L2, L3, L4, L5, L6, L7, L8)` with <S, L1, L2, L3, L4, L5, L6, L7, L8>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15>
- `(L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16)` with <S, L1, L2, L3, L4, L5, L6, L7, L8, L9, L10, L11, L12, L13, L14, L15, L16>
- `&''a T` with <''a, T, S>

## Re-exports

### Re-export `Identity`

```rust
pub use self::identity::Identity;
```

### Re-export `layer_fn`

```rust
pub use self::layer_fn::layer_fn;
```

### Re-export `LayerFn`

```rust
pub use self::layer_fn::LayerFn;
```

### Re-export `Stack`

```rust
pub use self::stack::Stack;
```

