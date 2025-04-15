# Crate Documentation

**Version:** 2.11.0

**Format Version:** 43

# Module `ipnet`

Types for IPv4 and IPv6 network addresses.

This module provides types and useful methods for working with IPv4
and IPv6 network addresses, commonly called IP prefixes. The new
[`IpNet`], [`Ipv4Net`], and [`Ipv6Net`] types build on the existing
[`IpAddr`], [`Ipv4Addr`], and [`Ipv6Addr`] types already provided in
Rust's standard library and align to their design to stay
consistent.
 
The module also provides the [`IpSubnets`], [`Ipv4Subnets`], and
[`Ipv6Subnets`] types for iterating over the subnets contained in
an IP address range. The [`IpAddrRange`], [`Ipv4AddrRange`], and
[`Ipv6AddrRange`] types for iterating over IP addresses in a range.
And traits that extend `Ipv4Addr` and `Ipv6Addr` with methods for
addition, subtraction, bitwise-and, and bitwise-or operations that
are missing in Rust's standard library.

The module only uses stable features so it is guaranteed to compile
using the stable toolchain.

# Organization

* [`IpNet`] represents an IP network address, either IPv4 or IPv6.
* [`Ipv4Net`] and [`Ipv6Net`] are respectively IPv4 and IPv6 network
  addresses.
* [`IpSubnets`], [`Ipv4Subnets`], and [`Ipv6Subnets`] are iterators
  that generate the smallest set of IP network addresses bound by an
  IP address range and minimum prefix length. These can be created
  using their constructors. They are also returned by the
  [`subnets()`] methods and used within the [`aggregate()`] methods.
* [`IpAddrRange`], [`Ipv4AddrRange`], and [`Ipv6AddrRange`] are
  iterators that generate IP addresses. These can be created using
  their constructors. They are also returned by the [`hosts()`]
  methods.
* The [`IpAdd`], [`IpSub`], [`IpBitAnd`], [`IpBitOr`] traits extend
  the [`Ipv4Addr`] and [`Ipv6Addr`] types with methods to perform
  these operations.

[`IpNet`]: enum.IpNet.html
[`Ipv4Net`]: struct.Ipv4Net.html
[`Ipv6Net`]: struct.Ipv6Net.html
[`IpAddr`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[`Ipv4Addr`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html
[`Ipv6Addr`]: https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html
[`IpSubnets`]: enum.IpSubnets.html
[`Ipv4Subnets`]: struct.Ipv4Subnets.html
[`Ipv6Subnets`]: struct.Ipv6Subnets.html
[`subnets()`]: enum.IpNet.html#method.subnets
[`aggregate()`]: enum.IpNet.html#method.aggregate
[`IpAddrRange`]: enum.IpAddrRange.html
[`Ipv4AddrRange`]: struct.Ipv4AddrRange.html
[`Ipv6AddrRange`]: struct.Ipv6AddrRange.html
[`hosts()`]: enum.IpNet.html#method.hosts
[`IpAdd`]: trait.IpAdd.html
[`IpSub`]: trait.IpSub.html
[`IpBitAnd`]: trait.IpBitAnd.html
[`IpBitOr`]: trait.IpBitOr.html

# Serde support

This library comes with support for [serde](https://serde.rs) but
it's not enabled by default. Use the `serde` [feature] to enable.
 
```toml
[dependencies]
ipnet = { version = "2", features = ["serde"] }
```

For human readable formats (e.g. JSON) the `IpNet`, `Ipv4Net`, and
`Ipv6Net` types will serialize to their `Display` strings.
 
For compact binary formats (e.g. Bincode) the `Ipv4Net` and
`Ipv6Net` types will serialize to a string of 5 and 17 bytes that
consist of the network address octects followed by the prefix
length. The `IpNet` type will serialize to an Enum with the V4 or V6
variant index prepending the above string of 5 or 17 bytes.

[feature]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

## Re-exports

### Re-export `IpAdd`

```rust
pub use self::ipext::IpAdd;
```

### Re-export `IpSub`

```rust
pub use self::ipext::IpSub;
```

### Re-export `IpBitAnd`

```rust
pub use self::ipext::IpBitAnd;
```

### Re-export `IpBitOr`

```rust
pub use self::ipext::IpBitOr;
```

### Re-export `IpAddrRange`

```rust
pub use self::ipext::IpAddrRange;
```

### Re-export `Ipv4AddrRange`

```rust
pub use self::ipext::Ipv4AddrRange;
```

### Re-export `Ipv6AddrRange`

```rust
pub use self::ipext::Ipv6AddrRange;
```

### Re-export `IpNet`

```rust
pub use self::ipnet::IpNet;
```

### Re-export `Ipv4Net`

```rust
pub use self::ipnet::Ipv4Net;
```

### Re-export `Ipv6Net`

```rust
pub use self::ipnet::Ipv6Net;
```

### Re-export `PrefixLenError`

```rust
pub use self::ipnet::PrefixLenError;
```

### Re-export `IpSubnets`

```rust
pub use self::ipnet::IpSubnets;
```

### Re-export `Ipv4Subnets`

```rust
pub use self::ipnet::Ipv4Subnets;
```

### Re-export `Ipv6Subnets`

```rust
pub use self::ipnet::Ipv6Subnets;
```

### Re-export `AddrParseError`

```rust
pub use self::parser::AddrParseError;
```

### Re-export `ip_mask_to_prefix`

```rust
pub use self::mask::ip_mask_to_prefix;
```

### Re-export `ipv4_mask_to_prefix`

```rust
pub use self::mask::ipv4_mask_to_prefix;
```

### Re-export `ipv6_mask_to_prefix`

```rust
pub use self::mask::ipv6_mask_to_prefix;
```

