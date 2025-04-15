# Crate Documentation

**Version:** 1.0.140

**Format Version:** 43

# Module `serde_json`

# Serde JSON

JSON is a ubiquitous open-standard format that uses human-readable text to
transmit data objects consisting of key-value pairs.

```json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

There are three common ways that you might find yourself needing to work
with JSON data in Rust.

 - **As text data.** An unprocessed string of JSON data that you receive on
   an HTTP endpoint, read from a file, or prepare to send to a remote
   server.
 - **As an untyped or loosely typed representation.** Maybe you want to
   check that some JSON data is valid before passing it on, but without
   knowing the structure of what it contains. Or you want to do very basic
   manipulations like insert a key in a particular spot.
 - **As a strongly typed Rust data structure.** When you expect all or most
   of your data to conform to a particular structure and want to get real
   work done without JSON's loosey-goosey nature tripping you up.

Serde JSON provides efficient, flexible, safe ways of converting data
between each of these representations.

# Operating on untyped JSON values

Any valid JSON data can be manipulated in the following recursive enum
representation. This data structure is [`serde_json::Value`][value].

```
# use serde_json::{Number, Map};
#
# #[allow(dead_code)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][from_str] function. There is also [`from_slice`]
for parsing from a byte slice `&[u8]` and [`from_reader`] for parsing from
any `io::Read` like a File or a TCP stream.

```
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
#
# fn main() {
#     untyped_example().unwrap();
# }
```

The result of square bracket indexing like `v["name"]` is a borrow of the
data at that index, so the type is `&Value`. A JSON map can be indexed with
string keys, while a JSON array can be indexed with integer keys. If the
type of the data is not right for the type with which it is being indexed,
or if a map does not contain the key being indexed, or if the index into a
vector is out of bounds, the returned element is `Value::Null`.

When a `Value` is printed, it is printed as a JSON string. So in the code
above, the output looks like `Please call "John Doe" at the number "+44
1234567"`. The quotation marks appear because `v["name"]` is a `&Value`
containing a JSON string and its JSON representation is `"John Doe"`.
Printing as a plain string without quotation marks involves converting from
a JSON string to a Rust string with [`as_str()`] or avoiding the use of
`Value` as described in the following section.

[`as_str()`]: crate::Value::as_str

The `Value` representation is sufficient for very basic tasks but can be
tedious to work with for anything more significant. Error handling is
verbose to implement correctly, for example imagine trying to detect the
presence of unrecognized fields in the input data. The compiler is powerless
to help you when you make a mistake, for example imagine typoing `v["name"]`
as `v["nmae"]` in one of the dozens of places it is used in your code.

# Parsing JSON as strongly typed data structures

Serde provides a powerful way of mapping JSON data into Rust data structures
largely automatically.

```
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
#
# fn main() {
#     typed_example().unwrap();
# }
```

This is the same `serde_json::from_str` function as before, but this time we
assign the return value to a variable of type `Person` so Serde will
automatically interpret the input data as a `Person` and produce informative
error messages if the layout does not conform to what a `Person` is expected
to look like.

Any type that implements Serde's `Deserialize` trait can be deserialized
this way. This includes built-in Rust standard library types like `Vec<T>`
and `HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Deserialize)]`.

Once we have `p` of type `Person`, our IDE and the Rust compiler can help us
use it correctly like they do for any other Rust code. The IDE can
autocomplete field names to prevent typos, which was impossible in the
`serde_json::Value` representation. And the Rust compiler can check that
when we write `p.phones[0]`, then `p.phones` is guaranteed to be a
`Vec<String>` so indexing into it makes sense and produces a `String`.

# Constructing JSON values

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

This is amazingly convenient, but we have the problem we had before with
`Value`: the IDE and Rust compiler cannot help us if we get it wrong. Serde
JSON provides a better way of serializing strongly-typed data structures
into JSON text.

# Creating JSON by serializing data structures

A data structure can be converted to a JSON string by
[`serde_json::to_string`][to_string]. There is also
[`serde_json::to_vec`][to_vec] which serializes to a `Vec<u8>` and
[`serde_json::to_writer`][to_writer] which serializes to any `io::Write`
such as a File or a TCP stream.

```
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}
#
# fn main() {
#     print_an_address().unwrap();
# }
```

Any type that implements Serde's `Serialize` trait can be serialized this
way. This includes built-in Rust standard library types like `Vec<T>` and
`HashMap<K, V>`, as well as any structs or enums annotated with
`#[derive(Serialize)]`.

# No-std support

As long as there is a memory allocator, it is possible to use serde_json
without the rest of the Rust standard library. Disable the default "std"
feature and enable the "alloc" feature:

```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

For JSON support in Serde without a memory allocator, please see the
[`serde-json-core`] crate.

[value]: crate::value::Value
[from_str]: crate::de::from_str
[from_slice]: crate::de::from_slice
[from_reader]: crate::de::from_reader
[to_string]: crate::ser::to_string
[to_vec]: crate::ser::to_vec
[to_writer]: crate::ser::to_writer
[macro]: crate::json
[`serde-json-core`]: https://github.com/rust-embedded-community/serde-json-core

## Modules

## Module `de`

Deserialize JSON data to a Rust data structure.

```rust
pub mod de { /* ... */ }
```

### Types

#### Struct `Deserializer`

A structure that deserializes JSON into Rust values.

```rust
pub struct Deserializer<R> {
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
  pub fn new(read: R) -> Self { /* ... */ }
  ```
  Create a JSON deserializer from one of the possible serde_json input

- ```rust
  pub fn from_reader(reader: R) -> Self { /* ... */ }
  ```
  Creates a JSON deserializer from an `io::Read`.

- ```rust
  pub fn from_slice(bytes: &''a [u8]) -> Self { /* ... */ }
  ```
  Creates a JSON deserializer from a `&[u8]`.

- ```rust
  pub fn from_str(s: &''a str) -> Self { /* ... */ }
  ```
  Creates a JSON deserializer from a `&str`.

- ```rust
  pub fn end(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  The `Deserializer::end` method should be called after a value has been fully deserialized.

- ```rust
  pub fn into_iter<T>(self: Self) -> StreamDeserializer<''de, R, T>
where
    T: de::Deserialize<''de> { /* ... */ }
  ```
  Turn a JSON deserializer into an iterator over values of type T.

- ```rust
  pub fn disable_recursion_limit(self: &mut Self) { /* ... */ }
  ```
  Parse arbitrarily deep JSON structures without any consideration for

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```
    Parses a JSON string as bytes. Note that this function does not check

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```
    Parses a `null` as a None, and any other values as a `Some(...)`.

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &str, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```
    Parses a newtype struct as the underlying value.

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, _name: &''static str, _len: usize, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, _name: &''static str, _fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, _name: &str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```
    Parses an enum as an object like `{"$KEY":$VALUE}`, where $VALUE is either a straight

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value>
where
    V: de::Visitor<''de> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `StreamDeserializer`

Iterator that deserializes a stream into multiple JSON values.

A stream deserializer can be created from any JSON deserializer using the
`Deserializer::into_iter` method.

The data can consist of any JSON value. Values need to be a self-delineating value e.g.
arrays, objects, or strings, or be followed by whitespace or a self-delineating value.

```
use serde_json::{Deserializer, Value};

fn main() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";

    let stream = Deserializer::from_str(data).into_iter::<Value>();

    for value in stream {
        println!("{}", value.unwrap());
    }
}
```

```rust
pub struct StreamDeserializer<''de, R, T> {
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
  pub fn new(read: R) -> Self { /* ... */ }
  ```
  Create a JSON stream deserializer from one of the possible serde_json

- ```rust
  pub fn byte_offset(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of bytes so far deserialized into a successful `T`.

###### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<T>> { /* ... */ }
    ```

- **FusedIterator**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
### Functions

#### Function `from_reader`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Deserialize an instance of type `T` from an I/O stream of JSON.

The content of the I/O stream is deserialized directly from the stream
without being buffered in memory by serde_json.

When reading from a source against which short reads are not efficient, such
as a [`File`], you will want to apply your own buffering because serde_json
will not buffer the input. See [`std::io::BufReader`].

It is expected that the input stream ends after the deserialized object.
If the stream does not end, such as in the case of a persistent socket connection,
this function will not return. It is possible instead to deserialize from a prefix of an input
stream without looking for EOF by managing your own [`Deserializer`].

Note that counter to intuition, this function is usually slower than
reading a file completely into memory and then applying [`from_str`]
or [`from_slice`] on it. See [issue #160].

[`File`]: std::fs::File
[issue #160]: https://github.com/serde-rs/json/issues/160

# Example

Reading the contents of a file.

```
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
```

Reading from a persistent socket connection.

```
use serde::Deserialize;

use std::error::Error;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_stream(stream: &mut BufReader<TcpStream>) -> Result<User, Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_reader(stream);
    let u = User::deserialize(&mut de)?;

    Ok(u)
}

fn main() {
# }
# fn fake_main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for tcp_stream in listener.incoming() {
        let mut buffered = BufReader::new(tcp_stream.unwrap());
        println!("{:#?}", read_user_from_stream(&mut buffered));
    }
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
pub fn from_reader<R, T>(rdr: R) -> crate::error::Result<T>
where
    R: crate::io::Read,
    T: de::DeserializeOwned { /* ... */ }
```

#### Function `from_slice`

Deserialize an instance of type `T` from bytes of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&[u8]`
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_slice(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
pub fn from_slice<''a, T>(v: &''a [u8]) -> crate::error::Result<T>
where
    T: de::Deserialize<''a> { /* ... */ }
```

#### Function `from_str`

Deserialize an instance of type `T` from a string of JSON text.

# Example

```
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `&str`
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";

    let u: User = serde_json::from_str(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the input does not match the
structure expected by `T`, for example if `T` is a struct type but the input
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
pub fn from_str<''a, T>(s: &''a str) -> crate::error::Result<T>
where
    T: de::Deserialize<''a> { /* ... */ }
```

### Re-exports

#### Re-export `Read`

```rust
pub use crate::read::Read;
```

#### Re-export `SliceRead`

```rust
pub use crate::read::SliceRead;
```

#### Re-export `StrRead`

```rust
pub use crate::read::StrRead;
```

#### Re-export `IoRead`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

```rust
pub use crate::read::IoRead;
```

## Module `error`

When serializing or deserializing JSON goes wrong.

```rust
pub mod error { /* ... */ }
```

### Types

#### Struct `Error`

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

```rust
pub struct Error {
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
  pub fn line(self: &Self) -> usize { /* ... */ }
  ```
  One-based line number at which the error was detected.

- ```rust
  pub fn column(self: &Self) -> usize { /* ... */ }
  ```
  One-based column number at which the error was detected.

- ```rust
  pub fn classify(self: &Self) -> Category { /* ... */ }
  ```
  Categorizes the cause of this error.

- ```rust
  pub fn is_io(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this error was caused by a failure to read or write

- ```rust
  pub fn is_syntax(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this error was caused by input that was not

- ```rust
  pub fn is_data(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this error was caused by input data that was

- ```rust
  pub fn is_eof(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this error was caused by prematurely reaching the end of

- ```rust
  pub fn io_error_kind(self: &Self) -> Option<ErrorKind> { /* ... */ }
  ```
  The kind reported by the underlying standard library I/O error, if this

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(j: Error) -> Self { /* ... */ }
    ```
    Convert a `serde_json::Error` into an `io::Error`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

  - ```rust
    fn custom<T: Display>(msg: T) -> Error { /* ... */ }
    ```

  - ```rust
    fn invalid_type(unexp: de::Unexpected<''_>, exp: &dyn de::Expected) -> Self { /* ... */ }
    ```

  - ```rust
    fn invalid_value(unexp: de::Unexpected<''_>, exp: &dyn de::Expected) -> Self { /* ... */ }
    ```

  - ```rust
    fn custom<T: Display>(msg: T) -> Error { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
#### Type Alias `Result`

Alias for a `Result` with the error type `serde_json::Error`.

```rust
pub type Result<T> = result::Result<T, Error>;
```

#### Enum `Category`

Categorizes the cause of a `serde_json::Error`.

```rust
pub enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}
```

##### Variants

###### `Io`

The error was caused by a failure to read or write bytes on an I/O
stream.

###### `Syntax`

The error was caused by input that was not syntactically valid JSON.

###### `Data`

The error was caused by input data that was semantically incorrect.

For example, JSON containing a number is semantically incorrect when the
type being deserialized into holds a String.

###### `Eof`

The error was caused by prematurely reaching the end of the input data.

Callers that process streaming input may be interested in retrying the
deserialization once more data is available.

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Category { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **StructuralPartialEq**
- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Category) -> bool { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

## Module `map`

A map of String to serde_json::Value.

By default the map is backed by a [`BTreeMap`]. Enable the `preserve_order`
feature of serde_json to use [`IndexMap`] instead.

[`BTreeMap`]: std::collections::BTreeMap
[`IndexMap`]: indexmap::IndexMap

```rust
pub mod map { /* ... */ }
```

### Types

#### Struct `Map`

Represents a JSON key/value type.

```rust
pub struct Map<K, V> {
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
  Makes a new empty Map.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Makes a new empty Map with the given initial capacity.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the map, removing all values.

- ```rust
  pub fn get<Q>(self: &Self, key: &Q) -> Option<&Value>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Returns a reference to the value corresponding to the key.

- ```rust
  pub fn contains_key<Q>(self: &Self, key: &Q) -> bool
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Returns true if the map contains a value for the specified key.

- ```rust
  pub fn get_mut<Q>(self: &mut Self, key: &Q) -> Option<&mut Value>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the key.

- ```rust
  pub fn get_key_value<Q>(self: &Self, key: &Q) -> Option<(&String, &Value)>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Returns the key-value pair matching the given key.

- ```rust
  pub fn insert(self: &mut Self, k: String, v: Value) -> Option<Value> { /* ... */ }
  ```
  Inserts a key-value pair into the map.

- ```rust
  pub fn remove<Q>(self: &mut Self, key: &Q) -> Option<Value>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Removes a key from the map, returning the value at the key if the key

- ```rust
  pub fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(String, Value)>
where
    String: Borrow<Q>,
    Q: ?Sized + Ord + Eq + Hash { /* ... */ }
  ```
  Removes a key from the map, returning the stored key and value if the

- ```rust
  pub fn append(self: &mut Self, other: &mut Self) { /* ... */ }
  ```
  Moves all elements from other into self, leaving other empty.

- ```rust
  pub fn entry<S>(self: &mut Self, key: S) -> Entry<''_>
where
    S: Into<String> { /* ... */ }
  ```
  Gets the given key's corresponding entry in the map for in-place

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the map contains no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_> { /* ... */ }
  ```
  Gets an iterator over the entries of the map.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_> { /* ... */ }
  ```
  Gets a mutable iterator over the entries of the map.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_> { /* ... */ }
  ```
  Gets an iterator over the keys of the map.

- ```rust
  pub fn values(self: &Self) -> Values<''_> { /* ... */ }
  ```
  Gets an iterator over the values of the map.

- ```rust
  pub fn values_mut(self: &mut Self) -> ValuesMut<''_> { /* ... */ }
  ```
  Gets an iterator over mutable values of the map.

- ```rust
  pub fn into_values(self: Self) -> IntoValues { /* ... */ }
  ```
  Gets an iterator over the values of the map.

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&String, &mut Value) -> bool { /* ... */ }
  ```
  Retains only the elements specified by the predicate.

- ```rust
  pub fn sort_keys(self: &mut Self) { /* ... */ }
  ```
  Sorts this map's entries in-place using `str`'s usual ordering.

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
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

  - ```rust
    fn from(f: Map<String, Value>) -> Self { /* ... */ }
    ```
    Convert map (with string keys) to `Value::Object`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

  - ```rust
    fn clone_from(self: &mut Self, source: &Self) { /* ... */ }
    ```

- **DeserializeOwned**
- **Extend**
  - ```rust
    fn extend<T>(self: &mut Self, iter: T)
where
    T: IntoIterator<Item = (String, Value)> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: serde::ser::Serializer { /* ... */ }
    ```

- **Send**
- **FromIterator**
  - ```rust
    fn from_iter<T>(iter: T) -> Self
where
    T: IntoIterator<Item = (String, Value)> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, _name: &''static str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, _name: &''static str, _variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u128<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, name: &''static str, len: usize, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, name: &''static str, fields: &''static [&''static str], visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> $crate::__private::Result<<V as >::Value, <Self as $crate::de::Deserializer<''de>>::Error>
where
    V: $crate::de::Visitor<''de> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: &Q) -> &Value { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>
where
    D: de::Deserializer<''de> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: &Q) -> &mut Value { /* ... */ }
    ```

#### Enum `Entry`

A view into a single entry in a map, which may either be vacant or occupied.
This enum is constructed from the [`entry`] method on [`Map`].

[`entry`]: Map::entry

```rust
pub enum Entry<''a> {
    Vacant(VacantEntry<''a>),
    Occupied(OccupiedEntry<''a>),
}
```

##### Variants

###### `Vacant`

A vacant Entry.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `VacantEntry<''a>` |  |

###### `Occupied`

An occupied Entry.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OccupiedEntry<''a>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Returns a reference to this entry's key.

- ```rust
  pub fn or_insert(self: Self, default: Value) -> &''a mut Value { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the default if empty, and

- ```rust
  pub fn or_insert_with<F>(self: Self, default: F) -> &''a mut Value
where
    F: FnOnce() -> Value { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the result of the default

- ```rust
  pub fn and_modify<F>(self: Self, f: F) -> Self
where
    F: FnOnce(&mut Value) { /* ... */ }
  ```
  Provides in-place mutable access to an occupied entry before any

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `VacantEntry`

A vacant Entry. It is part of the [`Entry`] enum.

```rust
pub struct VacantEntry<''a> {
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
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Gets a reference to the key that would be used when inserting a value

- ```rust
  pub fn insert(self: Self, value: Value) -> &''a mut Value { /* ... */ }
  ```
  Sets the value of the entry with the VacantEntry's key, and returns a

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `OccupiedEntry`

An occupied Entry. It is part of the [`Entry`] enum.

```rust
pub struct OccupiedEntry<''a> {
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
  pub fn key(self: &Self) -> &String { /* ... */ }
  ```
  Gets a reference to the key in the entry.

- ```rust
  pub fn get(self: &Self) -> &Value { /* ... */ }
  ```
  Gets a reference to the value in the entry.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut Value { /* ... */ }
  ```
  Gets a mutable reference to the value in the entry.

- ```rust
  pub fn into_mut(self: Self) -> &''a mut Value { /* ... */ }
  ```
  Converts the entry into a mutable reference to its value.

- ```rust
  pub fn insert(self: &mut Self, value: Value) -> Value { /* ... */ }
  ```
  Sets the value of the entry with the `OccupiedEntry`'s key, and returns

- ```rust
  pub fn remove(self: Self) -> Value { /* ... */ }
  ```
  Takes the value of the entry out of the map, and returns it.

- ```rust
  pub fn remove_entry(self: Self) -> (String, Value) { /* ... */ }
  ```
  Removes the entry from the map, returning the stored key and value.

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
#### Struct `Iter`

An iterator over a serde_json::Map's entries.

```rust
pub struct Iter<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **FusedIterator**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `IterMut`

A mutable iterator over a serde_json::Map's entries.

```rust
pub struct IterMut<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FusedIterator**
- **Send**
#### Struct `IntoIter`

An owning iterator over a serde_json::Map's entries.

```rust
pub struct IntoIter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **Freeze**
- **RefUnwindSafe**
- **FusedIterator**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Keys`

An iterator over a serde_json::Map's keys.

```rust
pub struct Keys<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **FusedIterator**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Values`

An iterator over a serde_json::Map's values.

```rust
pub struct Values<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FusedIterator**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `ValuesMut`

A mutable iterator over a serde_json::Map's values.

```rust
pub struct ValuesMut<''a> {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **FusedIterator**
- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Freeze**
#### Struct `IntoValues`

An owning iterator over a serde_json::Map's values.

```rust
pub struct IntoValues {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **FusedIterator**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ExactSizeIterator**
  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

## Module `ser`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Serialize a Rust data structure into JSON data.

```rust
pub mod ser { /* ... */ }
```

### Types

#### Struct `Serializer`

**Attributes:**

- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

A structure for serializing Rust values into JSON.

```rust
pub struct Serializer<W, F = CompactFormatter> {
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
  pub fn new(writer: W) -> Self { /* ... */ }
  ```
  Creates a new JSON serializer.

- ```rust
  pub fn pretty(writer: W) -> Self { /* ... */ }
  ```
  Creates a new JSON pretty print serializer.

- ```rust
  pub fn with_formatter(writer: W, formatter: F) -> Self { /* ... */ }
  ```
  Creates a new JSON visitor whose output will be written to the writer

- ```rust
  pub fn into_inner(self: Self) -> W { /* ... */ }
  ```
  Unwrap the `Writer` from the `Serializer`.

###### Trait Implementations

- **Serializer**
  - ```rust
    fn serialize_bool(self: Self, value: bool) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i8(self: Self, value: i8) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i16(self: Self, value: i16) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i32(self: Self, value: i32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i64(self: Self, value: i64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_i128(self: Self, value: i128) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u8(self: Self, value: u8) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u16(self: Self, value: u16) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u32(self: Self, value: u32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u64(self: Self, value: u64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_u128(self: Self, value: u128) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_f32(self: Self, value: f32) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_f64(self: Self, value: f64) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_char(self: Self, value: char) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_str(self: Self, value: &str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_bytes(self: Self, value: &[u8]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit(self: Self) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_struct(self: Self, _name: &''static str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_unit_variant(self: Self, _name: &''static str, _variant_index: u32, variant: &''static str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_newtype_struct<T>(self: Self, _name: &''static str, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```
    Serialize newtypes without an object wrapper.

  - ```rust
    fn serialize_newtype_variant<T>(self: Self, _name: &''static str, _variant_index: u32, variant: &''static str, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```

  - ```rust
    fn serialize_none(self: Self) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn serialize_some<T>(self: Self, value: &T) -> Result<()>
where
    T: ?Sized + Serialize { /* ... */ }
    ```

  - ```rust
    fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_struct(self: Self, _name: &''static str, len: usize) -> Result<<Self as >::SerializeTupleStruct> { /* ... */ }
    ```

  - ```rust
    fn serialize_tuple_variant(self: Self, _name: &''static str, _variant_index: u32, variant: &''static str, len: usize) -> Result<<Self as >::SerializeTupleVariant> { /* ... */ }
    ```

  - ```rust
    fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct(self: Self, name: &''static str, len: usize) -> Result<<Self as >::SerializeStruct> { /* ... */ }
    ```

  - ```rust
    fn serialize_struct_variant(self: Self, _name: &''static str, _variant_index: u32, variant: &''static str, len: usize) -> Result<<Self as >::SerializeStructVariant> { /* ... */ }
    ```

  - ```rust
    fn collect_str<T>(self: Self, value: &T) -> Result<()>
where
    T: ?Sized + Display { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `CharEscape`

Represents a character escape code in a type-safe manner.

```rust
pub enum CharEscape {
    Quote,
    ReverseSolidus,
    Solidus,
    Backspace,
    FormFeed,
    LineFeed,
    CarriageReturn,
    Tab,
    AsciiControl(u8),
}
```

##### Variants

###### `Quote`

An escaped quote `"`

###### `ReverseSolidus`

An escaped reverse solidus `\`

###### `Solidus`

An escaped solidus `/`

###### `Backspace`

An escaped backspace character (usually escaped as `\b`)

###### `FormFeed`

An escaped form feed character (usually escaped as `\f`)

###### `LineFeed`

An escaped line feed character (usually escaped as `\n`)

###### `CarriageReturn`

An escaped carriage return character (usually escaped as `\r`)

###### `Tab`

An escaped tab character (usually escaped as `\t`)

###### `AsciiControl`

An escaped ASCII plane control character (usually escaped as
`\u00XX` where `XX` are two hex characters)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `CompactFormatter`

This structure compacts a JSON value with no extra whitespace.

```rust
pub struct CompactFormatter;
```

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Formatter**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompactFormatter { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `PrettyFormatter`

This structure pretty prints a JSON value to make it human readable.

```rust
pub struct PrettyFormatter<''a> {
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
  Construct a pretty printer formatter that defaults to using two spaces for indentation.

- ```rust
  pub fn with_indent(indent: &''a [u8]) -> Self { /* ... */ }
  ```
  Construct a pretty printer formatter that uses the `indent` string for indentation.

###### Trait Implementations

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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PrettyFormatter<''a> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Unpin**
- **Formatter**
  - ```rust
    fn begin_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn end_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn begin_array_value<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn end_array_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn begin_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn end_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn begin_object_key<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn begin_object_value<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

  - ```rust
    fn end_object_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Traits

#### Trait `Formatter`

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

```rust
pub trait Formatter {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn write_null<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a `null` value to the specified writer.

- ```rust
  fn write_bool<W>(self: &mut Self, writer: &mut W, value: bool) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a `true` or `false` value to the specified writer.

- ```rust
  fn write_i8<W>(self: &mut Self, writer: &mut W, value: i8) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `-123` to the specified writer.

- ```rust
  fn write_i16<W>(self: &mut Self, writer: &mut W, value: i16) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `-123` to the specified writer.

- ```rust
  fn write_i32<W>(self: &mut Self, writer: &mut W, value: i32) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `-123` to the specified writer.

- ```rust
  fn write_i64<W>(self: &mut Self, writer: &mut W, value: i64) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `-123` to the specified writer.

- ```rust
  fn write_i128<W>(self: &mut Self, writer: &mut W, value: i128) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `-123` to the specified writer.

- ```rust
  fn write_u8<W>(self: &mut Self, writer: &mut W, value: u8) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `123` to the specified writer.

- ```rust
  fn write_u16<W>(self: &mut Self, writer: &mut W, value: u16) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `123` to the specified writer.

- ```rust
  fn write_u32<W>(self: &mut Self, writer: &mut W, value: u32) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `123` to the specified writer.

- ```rust
  fn write_u64<W>(self: &mut Self, writer: &mut W, value: u64) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `123` to the specified writer.

- ```rust
  fn write_u128<W>(self: &mut Self, writer: &mut W, value: u128) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes an integer value like `123` to the specified writer.

- ```rust
  fn write_f32<W>(self: &mut Self, writer: &mut W, value: f32) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a floating point value like `-31.26e+12` to the specified writer.

- ```rust
  fn write_f64<W>(self: &mut Self, writer: &mut W, value: f64) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a floating point value like `-31.26e+12` to the specified writer.

- ```rust
  fn write_number_str<W>(self: &mut Self, writer: &mut W, value: &str) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a number that has already been rendered to a string.

- ```rust
  fn begin_string<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before each series of `write_string_fragment` and

- ```rust
  fn end_string<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after each series of `write_string_fragment` and

- ```rust
  fn write_string_fragment<W>(self: &mut Self, writer: &mut W, fragment: &str) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a string fragment that doesn't need any escaping to the

- ```rust
  fn write_char_escape<W>(self: &mut Self, writer: &mut W, char_escape: CharEscape) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a character escape code to the specified writer.

- ```rust
  fn write_byte_array<W>(self: &mut Self, writer: &mut W, value: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes the representation of a byte array. Formatters can choose whether

- ```rust
  fn begin_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before every array.  Writes a `[` to the specified

- ```rust
  fn end_array<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after every array.  Writes a `]` to the specified

- ```rust
  fn begin_array_value<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before every array value.  Writes a `,` if needed to

- ```rust
  fn end_array_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after every array value.

- ```rust
  fn begin_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before every object.  Writes a `{` to the specified

- ```rust
  fn end_object<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after every object.  Writes a `}` to the specified

- ```rust
  fn begin_object_key<W>(self: &mut Self, writer: &mut W, first: bool) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before every object key.

- ```rust
  fn end_object_key<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after every object key.  A `:` should be written to the

- ```rust
  fn begin_object_value<W>(self: &mut Self, writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called before every object value.  A `:` should be written to

- ```rust
  fn end_object_value<W>(self: &mut Self, _writer: &mut W) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Called after every object value.

- ```rust
  fn write_raw_fragment<W>(self: &mut Self, writer: &mut W, fragment: &str) -> io::Result<()>
where
    W: ?Sized + io::Write { /* ... */ }
  ```
  Writes a raw JSON fragment that doesn't need any escaping to the

##### Implementations

This trait is implemented for the following types:

- `CompactFormatter`
- `PrettyFormatter<''a>` with <''a>

### Functions

#### Function `to_writer`

**Attributes:**

- `#[inline]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Serialize the given data structure as JSON into the I/O stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_writer_pretty`

**Attributes:**

- `#[inline]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> crate::error::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_vec`

**Attributes:**

- `#[inline]`

Serialize the given data structure as a JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_vec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_vec_pretty`

**Attributes:**

- `#[inline]`

Serialize the given data structure as a pretty-printed JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_vec_pretty<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
where
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_string`

**Attributes:**

- `#[inline]`

Serialize the given data structure as a String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_string<T>(value: &T) -> crate::error::Result<alloc::string::String>
where
    T: ?Sized + Serialize { /* ... */ }
```

#### Function `to_string_pretty`

**Attributes:**

- `#[inline]`

Serialize the given data structure as a pretty-printed String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
pub fn to_string_pretty<T>(value: &T) -> crate::error::Result<alloc::string::String>
where
    T: ?Sized + Serialize { /* ... */ }
```

## Module `value`

The Value enum, a loosely typed way of representing any valid JSON value.

# Constructing JSON

Serde JSON provides a [`json!` macro][macro] to build `serde_json::Value`
objects with very natural JSON syntax.

```
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions can
be interpolated directly into the JSON value as you are building it. Serde
will check at compile time that the value you are interpolating is able to
be represented as JSON.

```
# use serde_json::json;
#
# fn random_phone() -> u16 { 0 }
#
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`][from_str] function. There is also
[`from_slice`][from_slice] for parsing from a byte slice `&[u8]` and
[`from_reader`][from_reader] for parsing from any `io::Read` like a File or
a TCP stream.

```
use serde_json::{json, Value, Error};

fn untyped_example() -> Result<(), Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
#
# untyped_example().unwrap();
```

[macro]: crate::json
[from_str]: crate::de::from_str
[from_slice]: crate::de::from_slice
[from_reader]: crate::de::from_reader

```rust
pub mod value { /* ... */ }
```

### Types

#### Enum `Value`

Represents any valid JSON value.

See the [`serde_json::value` module documentation](self) for usage examples.

```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(alloc::string::String),
    Array(alloc::vec::Vec<Value>),
    Object(Map<alloc::string::String, Value>),
}
```

##### Variants

###### `Null`

Represents a JSON null value.

```
# use serde_json::json;
#
let v = json!(null);
```

###### `Bool`

Represents a JSON boolean.

```
# use serde_json::json;
#
let v = json!(true);
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `bool` |  |

###### `Number`

Represents a JSON number, whether integer or floating point.

```
# use serde_json::json;
#
let v = json!(12.5);
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Number` |  |

###### `String`

Represents a JSON string.

```
# use serde_json::json;
#
let v = json!("a string");
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::string::String` |  |

###### `Array`

Represents a JSON array.

```
# use serde_json::json;
#
let v = json!(["an", "array"]);
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `alloc::vec::Vec<Value>` |  |

###### `Object`

Represents a JSON object.

By default the map is backed by a BTreeMap. Enable the `preserve_order`
feature of serde_json to use IndexMap instead, which preserves
entries in the order they are inserted into the map. In particular, this
allows JSON data to be deserialized into a Value and serialized to a
string while retaining the order of map keys in the input.

```
# use serde_json::json;
#
let v = json!({ "an": "object" });
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Map<alloc::string::String, Value>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get<I: Index>(self: &Self, index: I) -> Option<&Value> { /* ... */ }
  ```
  Index into a JSON array or map. A string index can be used to access a

- ```rust
  pub fn get_mut<I: Index>(self: &mut Self, index: I) -> Option<&mut Value> { /* ... */ }
  ```
  Mutably index into a JSON array or map. A string index can be used to

- ```rust
  pub fn is_object(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is an Object. Returns false otherwise.

- ```rust
  pub fn as_object(self: &Self) -> Option<&Map<String, Value>> { /* ... */ }
  ```
  If the `Value` is an Object, returns the associated Map. Returns None

- ```rust
  pub fn as_object_mut(self: &mut Self) -> Option<&mut Map<String, Value>> { /* ... */ }
  ```
  If the `Value` is an Object, returns the associated mutable Map.

- ```rust
  pub fn is_array(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is an Array. Returns false otherwise.

- ```rust
  pub fn as_array(self: &Self) -> Option<&Vec<Value>> { /* ... */ }
  ```
  If the `Value` is an Array, returns the associated vector. Returns None

- ```rust
  pub fn as_array_mut(self: &mut Self) -> Option<&mut Vec<Value>> { /* ... */ }
  ```
  If the `Value` is an Array, returns the associated mutable vector.

- ```rust
  pub fn is_string(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is a String. Returns false otherwise.

- ```rust
  pub fn as_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  If the `Value` is a String, returns the associated str. Returns None

- ```rust
  pub fn is_number(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is a Number. Returns false otherwise.

- ```rust
  pub fn as_number(self: &Self) -> Option<&Number> { /* ... */ }
  ```
  If the `Value` is a Number, returns the associated [`Number`]. Returns

- ```rust
  pub fn is_i64(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is an integer between `i64::MIN` and

- ```rust
  pub fn is_u64(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is an integer between zero and `u64::MAX`.

- ```rust
  pub fn is_f64(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is a number that can be represented by f64.

- ```rust
  pub fn as_i64(self: &Self) -> Option<i64> { /* ... */ }
  ```
  If the `Value` is an integer, represent it as i64 if possible. Returns

- ```rust
  pub fn as_u64(self: &Self) -> Option<u64> { /* ... */ }
  ```
  If the `Value` is an integer, represent it as u64 if possible. Returns

- ```rust
  pub fn as_f64(self: &Self) -> Option<f64> { /* ... */ }
  ```
  If the `Value` is a number, represent it as f64 if possible. Returns

- ```rust
  pub fn is_boolean(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is a Boolean. Returns false otherwise.

- ```rust
  pub fn as_bool(self: &Self) -> Option<bool> { /* ... */ }
  ```
  If the `Value` is a Boolean, returns the associated bool. Returns None

- ```rust
  pub fn is_null(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `Value` is a Null. Returns false otherwise.

- ```rust
  pub fn as_null(self: &Self) -> Option<()> { /* ... */ }
  ```
  If the `Value` is a Null, returns (). Returns None otherwise.

- ```rust
  pub fn pointer(self: &Self, pointer: &str) -> Option<&Value> { /* ... */ }
  ```
  Looks up a value by a JSON Pointer.

- ```rust
  pub fn pointer_mut(self: &mut Self, pointer: &str) -> Option<&mut Value> { /* ... */ }
  ```
  Looks up a value by a JSON Pointer and returns a mutable reference to

- ```rust
  pub fn take(self: &mut Self) -> Value { /* ... */ }
  ```
  Takes the value out of the `Value`, leaving a `Null` in its place.

- ```rust
  pub fn sort_all_objects(self: &mut Self) { /* ... */ }
  ```
  Reorders the entries of all `Value::Object` nested within this JSON

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Value { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Value, Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &i64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &isize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &isize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &isize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u8) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &usize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &usize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &usize) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f32) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &f64) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &bool) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &bool) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &bool) -> bool { /* ... */ }
    ```

- **Deserializer**
  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, _name: &''static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, _name: &''static str, _fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_enum<V>(self: Self, name: &''static str, variants: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_newtype_struct<V>(self: Self, name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_unit_struct<V>(self: Self, _name: &''static str, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple<V>(self: Self, _len: usize, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_tuple_struct<V>(self: Self, _name: &''static str, _len: usize, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_struct<V>(self: Self, _name: &''static str, _fields: &''static [&''static str], visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

  - ```rust
    fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, Error>
where
    V: Visitor<''de> { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: I) -> &mut Value { /* ... */ }
    ```
    Write into a `serde_json::Value` using the syntax `value[0] = ...` or

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> result::Result<<S as >::Ok, <S as >::Error>
where
    S: ::serde::Serializer { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(n: i8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: isize) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: u8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(n: usize) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(f: f32) -> Self { /* ... */ }
    ```
    Convert 32-bit floating point number to `Value::Number`, or

  - ```rust
    fn from(f: f64) -> Self { /* ... */ }
    ```
    Convert 64-bit floating point number to `Value::Number`, or

  - ```rust
    fn from(f: bool) -> Self { /* ... */ }
    ```
    Convert boolean to `Value::Bool`.

  - ```rust
    fn from(f: String) -> Self { /* ... */ }
    ```
    Convert `String` to `Value::String`.

  - ```rust
    fn from(f: &str) -> Self { /* ... */ }
    ```
    Convert string slice to `Value::String`.

  - ```rust
    fn from(f: Cow<''a, str>) -> Self { /* ... */ }
    ```
    Convert copy-on-write string to `Value::String`.

  - ```rust
    fn from(f: Number) -> Self { /* ... */ }
    ```
    Convert `Number` to `Value::Number`.

  - ```rust
    fn from(f: Map<String, Value>) -> Self { /* ... */ }
    ```
    Convert map (with string keys) to `Value::Object`.

  - ```rust
    fn from(f: Vec<T>) -> Self { /* ... */ }
    ```
    Convert a `Vec` to `Value::Array`.

  - ```rust
    fn from(array: [T; N]) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(f: &[T]) -> Self { /* ... */ }
    ```
    Convert a slice to `Value::Array`.

  - ```rust
    fn from((): ()) -> Self { /* ... */ }
    ```
    Convert `()` to `Value::Null`.

  - ```rust
    fn from(opt: Option<T>) -> Self { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```
    Display a JSON value as a string.

- **Unpin**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { /* ... */ }
    ```
    Create a `Value::Array` by collecting an iterator of array elements.

  - ```rust
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self { /* ... */ }
    ```
    Create a `Value::Object` by collecting an iterator of key-value pairs.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: I) -> &Value { /* ... */ }
    ```
    Index into a `serde_json::Value` using the syntax `value[0]` or

- **IntoDeserializer**
  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

  - ```rust
    fn into_deserializer(self: Self) -> <Self as >::Deserializer { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **DeserializeOwned**
- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<Value, <D as >::Error>
where
    D: serde::Deserializer<''de> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Functions

#### Function `to_value`

Convert a `T` into `serde_json::Value` which is an enum that can represent
any valid JSON data.

# Example

```
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
struct User {
    fingerprint: String,
    location: String,
}

fn compare_json_values() -> Result<(), Box<dyn Error>> {
    let u = User {
        fingerprint: "0xF9BA143B95FF6D82".to_owned(),
        location: "Menlo Park, CA".to_owned(),
    };

    // The type of `expected` is `serde_json::Value`
    let expected = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA",
    });

    let v = serde_json::to_value(u).unwrap();
    assert_eq!(v, expected);

    Ok(())
}
#
# compare_json_values().unwrap();
```

# Errors

This conversion can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```
use std::collections::BTreeMap;

fn main() {
    // The keys in this map are vectors, not strings.
    let mut map = BTreeMap::new();
    map.insert(vec![32, 64], "x86");

    println!("{}", serde_json::to_value(map).unwrap_err());
}
```

```rust
pub fn to_value<T>(value: T) -> Result<Value, crate::error::Error>
where
    T: Serialize { /* ... */ }
```

#### Function `from_value`

Interpret a `serde_json::Value` as an instance of type `T`.

# Example

```
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn main() {
    // The type of `j` is `serde_json::Value`
    let j = json!({
        "fingerprint": "0xF9BA143B95FF6D82",
        "location": "Menlo Park, CA"
    });

    let u: User = serde_json::from_value(j).unwrap();
    println!("{:#?}", u);
}
```

# Errors

This conversion can fail if the structure of the Value does not match the
structure expected by `T`, for example if `T` is a struct type but the Value
contains something other than a JSON map. It can also fail if the structure
is correct but `T`'s implementation of `Deserialize` decides that something
is wrong with the data, for example required struct fields are missing from
the JSON map or some number is too big to fit in the expected primitive
type.

```rust
pub fn from_value<T>(value: Value) -> Result<T, crate::error::Error>
where
    T: DeserializeOwned { /* ... */ }
```

### Re-exports

#### Re-export `Index`

```rust
pub use self::index::Index;
```

#### Re-export `Serializer`

```rust
pub use self::ser::Serializer;
```

#### Re-export `Map`

```rust
pub use crate::map::Map;
```

#### Re-export `Number`

```rust
pub use crate::number::Number;
```

#### Re-export `to_raw_value`

**Attributes:**

- `#[<cfg>(feature = "raw_value")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "raw_value")))]`

```rust
pub use crate::raw::to_raw_value;
```

#### Re-export `RawValue`

**Attributes:**

- `#[<cfg>(feature = "raw_value")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "raw_value")))]`

```rust
pub use crate::raw::RawValue;
```

## Macros

### Macro `json`

**Attributes:**

- `#[macro_export]`

Construct a `serde_json::Value` from a JSON literal.

```
# use serde_json::json;
#
let value = json!({
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "serde",
            "json"
        ],
        "homepage": null
    }
});
```

Variables or expressions can be interpolated into the JSON literal. Any type
interpolated into an array element or object value must implement Serde's
`Serialize` trait, while any type interpolated into a object key must
implement `Into<String>`. If the `Serialize` implementation of the
interpolated type decides to fail, or if the interpolated type contains a
map with non-string keys, the `json!` macro will panic.

```
# use serde_json::json;
#
let code = 200;
let features = vec!["serde", "json"];

let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }
});
```

Trailing commas are allowed inside both arrays and objects.

```
# use serde_json::json;
#
let value = json!([
    "notice",
    "the",
    "trailing",
    "comma -->",
]);
```

```rust
pub macro_rules! json {
    /* macro_rules! json {
    ($($json:tt)+) => { ... };
} */
}
```

## Re-exports

### Re-export `from_reader`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[doc(inline)]`

```rust
pub use crate::de::from_reader;
```

### Re-export `from_slice`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::from_slice;
```

### Re-export `from_str`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::from_str;
```

### Re-export `Deserializer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::Deserializer;
```

### Re-export `StreamDeserializer`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::de::StreamDeserializer;
```

### Re-export `Error`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::error::Error;
```

### Re-export `Result`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::error::Result;
```

### Re-export `to_string`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::to_string;
```

### Re-export `to_string_pretty`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::to_string_pretty;
```

### Re-export `to_vec`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::to_vec;
```

### Re-export `to_vec_pretty`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::ser::to_vec_pretty;
```

### Re-export `to_writer`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[doc(inline)]`

```rust
pub use crate::ser::to_writer;
```

### Re-export `to_writer_pretty`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[doc(inline)]`

```rust
pub use crate::ser::to_writer_pretty;
```

### Re-export `Serializer`

**Attributes:**

- `#[<cfg>(feature = "std")]`
- `#[<cfg_attr>(docsrs, doc(cfg(feature = "std")))]`
- `#[doc(inline)]`

```rust
pub use crate::ser::Serializer;
```

### Re-export `from_value`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::from_value;
```

### Re-export `to_value`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::to_value;
```

### Re-export `Map`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::Map;
```

### Re-export `Number`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::Number;
```

### Re-export `Value`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use crate::value::Value;
```

