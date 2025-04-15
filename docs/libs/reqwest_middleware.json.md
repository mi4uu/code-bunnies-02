# Crate Documentation

**Version:** 0.4.2

**Format Version:** 43

# Module `reqwest_middleware`

This crate provides [`ClientWithMiddleware`], a wrapper around [`reqwest::Client`] with the
ability to attach middleware which runs on every request.

You'll want to instantiate [`ClientWithMiddleware`] using [`ClientBuilder`], then you can
attach your middleware using [`with`], finalize it with [`build`] and from then on sending
requests is the same as with reqwest:

```
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result};
use http::Extensions;

struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        println!("Request started {:?}", req);
        let res = next.run(req, extensions).await;
        println!("Result: {:?}", res);
        res
    }
}

async fn run() {
    let reqwest_client = Client::builder().build().unwrap();
    let client = ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build();
    let resp = client.get("https://truelayer.com").send().await.unwrap();
    println!("TrueLayer page HTML: {}", resp.text().await.unwrap());
}
```

[`build`]: ClientBuilder::build
[`ClientBuilder`]: ClientBuilder
[`ClientWithMiddleware`]: ClientWithMiddleware
[`with`]: ClientBuilder::with

## Re-exports

### Re-export `ClientBuilder`

```rust
pub use client::ClientBuilder;
```

### Re-export `ClientWithMiddleware`

```rust
pub use client::ClientWithMiddleware;
```

### Re-export `RequestBuilder`

```rust
pub use client::RequestBuilder;
```

### Re-export `Error`

```rust
pub use error::Error;
```

### Re-export `Result`

```rust
pub use error::Result;
```

### Re-export `Middleware`

```rust
pub use middleware::Middleware;
```

### Re-export `Next`

```rust
pub use middleware::Next;
```

### Re-export `Extension`

```rust
pub use req_init::Extension;
```

### Re-export `RequestInitialiser`

```rust
pub use req_init::RequestInitialiser;
```

### Re-export `reqwest`

```rust
pub use reqwest;
```

