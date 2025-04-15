# Crate Documentation

**Version:** 1.12.1

**Format Version:** 43

# Module `rayon_core`

Rayon-core houses the core stable APIs of Rayon.

These APIs have been mirrored in the Rayon crate and it is recommended to use these from there.

[`join`] is used to take two closures and potentially run them in parallel.
  - It will run in parallel if task B gets stolen before task A can finish.
  - It will run sequentially if task A finishes before task B is stolen and can continue on task B.

[`scope`] creates a scope in which you can run any number of parallel tasks.
These tasks can spawn nested tasks and scopes, but given the nature of work stealing, the order of execution can not be guaranteed.
The scope will exist until all tasks spawned within the scope have been completed.

[`spawn`] add a task into the 'static' or 'global' scope, or a local scope created by the [`scope()`] function.

[`ThreadPool`] can be used to create your own thread pools (using [`ThreadPoolBuilder`]) or to customize the global one.
Tasks spawned within the pool (using [`install()`], [`join()`], etc.) will be added to a deque,
where it becomes available for work stealing from other threads in the local threadpool.

[`join`]: fn.join.html
[`scope`]: fn.scope.html
[`scope()`]: fn.scope.html
[`spawn`]: fn.spawn.html
[`ThreadPool`]: struct.threadpool.html
[`install()`]: struct.ThreadPool.html#method.install
[`spawn()`]: struct.ThreadPool.html#method.spawn
[`join()`]: struct.ThreadPool.html#method.join
[`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html

# Global fallback when threading is unsupported

Rayon uses `std` APIs for threading, but some targets have incomplete implementations that
always return `Unsupported` errors. The WebAssembly `wasm32-unknown-unknown` and `wasm32-wasi`
targets are notable examples of this. Rather than panicking on the unsupported error when
creating the implicit global threadpool, Rayon configures a fallback mode instead.

This fallback mode mostly functions as if it were using a single-threaded "pool", like setting
`RAYON_NUM_THREADS=1`. For example, `join` will execute its two closures sequentially, since
there is no other thread to share the work. However, since the pool is not running independent
of the main thread, non-blocking calls like `spawn` may not execute at all, unless a lower-
priority call like `broadcast` gives them an opening. The fallback mode does not try to emulate
anything like thread preemption or `async` task switching, but `yield_now` or `yield_local`
can also volunteer execution time.

Explicit `ThreadPoolBuilder` methods always report their error without any fallback.

# Restricting multiple versions

In order to ensure proper coordination between threadpools, and especially
to make sure there's only one global threadpool, `rayon-core` is actively
restricted from building multiple versions of itself into a single target.
You may see a build error like this in violation:

```text
error: native library `rayon-core` is being linked to by more
than one package, and can only be linked to by one package
```

While we strive to keep `rayon-core` semver-compatible, it's still
possible to arrive at this situation if different crates have overly
restrictive tilde or inequality requirements for `rayon-core`.  The
conflicting requirements will need to be resolved before the build will
succeed.

## Types

### Struct `ThreadPoolBuildError`

Error when initializing a thread pool.

```rust
pub struct ThreadPoolBuildError {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Trait Implementations

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn source(self: &Self) -> Option<&dyn Error + ''static> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `ThreadPoolBuilder`

Used to create a new [`ThreadPool`] or to configure the global rayon thread pool.
## Creating a ThreadPool
The following creates a thread pool with 22 threads.

```rust
# use rayon_core as rayon;
let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
```

To instead configure the global thread pool, use [`build_global()`]:

```rust
# use rayon_core as rayon;
rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
```

[`ThreadPool`]: struct.ThreadPool.html
[`build_global()`]: struct.ThreadPoolBuilder.html#method.build_global

```rust
pub struct ThreadPoolBuilder<S = self::registry::DefaultSpawn> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates and returns a valid rayon thread pool builder, but does not initialize it.

- ```rust
  pub fn build(self: Self) -> Result<ThreadPool, ThreadPoolBuildError> { /* ... */ }
  ```
  Creates a new `ThreadPool` initialized using this configuration.

- ```rust
  pub fn build_global(self: Self) -> Result<(), ThreadPoolBuildError> { /* ... */ }
  ```
  Initializes the global thread pool. This initialization is

- ```rust
  pub fn build_scoped<W, F, R>(self: Self, wrapper: W, with_pool: F) -> Result<R, ThreadPoolBuildError>
where
    W: Fn(ThreadBuilder) + Sync,
    F: FnOnce(&ThreadPool) -> R { /* ... */ }
  ```
  Creates a scoped `ThreadPool` initialized using this configuration.

- ```rust
  pub fn spawn_handler<F>(self: Self, spawn: F) -> ThreadPoolBuilder<CustomSpawn<F>>
where
    F: FnMut(ThreadBuilder) -> io::Result<()> { /* ... */ }
  ```
  Sets a custom function for spawning threads.

- ```rust
  pub fn thread_name<F>(self: Self, closure: F) -> Self
where
    F: FnMut(usize) -> String + ''static { /* ... */ }
  ```
  Sets a closure which takes a thread index and returns

- ```rust
  pub fn num_threads(self: Self, num_threads: usize) -> Self { /* ... */ }
  ```
  Sets the number of threads to be used in the rayon threadpool.

- ```rust
  pub fn use_current_thread(self: Self) -> Self { /* ... */ }
  ```
  Use the current thread as one of the threads in the pool.

- ```rust
  pub fn panic_handler<H>(self: Self, panic_handler: H) -> Self
where
    H: Fn(Box<dyn Any + Send>) + Send + Sync + ''static { /* ... */ }
  ```
  Normally, whenever Rayon catches a panic, it tries to

- ```rust
  pub fn stack_size(self: Self, stack_size: usize) -> Self { /* ... */ }
  ```
  Sets the stack size of the worker threads

- ```rust
  pub fn breadth_first(self: Self) -> Self { /* ... */ }
  ```
  **(DEPRECATED)** Suggest to worker threads that they execute

- ```rust
  pub fn start_handler<H>(self: Self, start_handler: H) -> Self
where
    H: Fn(usize) + Send + Sync + ''static { /* ... */ }
  ```
  Sets a callback to be invoked on thread start.

- ```rust
  pub fn exit_handler<H>(self: Self, exit_handler: H) -> Self
where
    H: Fn(usize) + Send + Sync + ''static { /* ... */ }
  ```
  Sets a callback to be invoked on thread exit.

##### Trait Implementations

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

- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
### Struct `Configuration`

**⚠️ Deprecated**: Use `ThreadPoolBuilder`

Contains the rayon thread pool configuration. Use [`ThreadPoolBuilder`] instead.

[`ThreadPoolBuilder`]: struct.ThreadPoolBuilder.html

```rust
pub struct Configuration {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Configuration { /* ... */ }
  ```
  Creates and return a valid rayon thread pool configuration, but does not initialize it.

- ```rust
  pub fn build(self: Self) -> Result<ThreadPool, Box<dyn Error + ''static>> { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::build`.

- ```rust
  pub fn thread_name<F>(self: Self, closure: F) -> Self
where
    F: FnMut(usize) -> String + ''static { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::thread_name`.

- ```rust
  pub fn num_threads(self: Self, num_threads: usize) -> Configuration { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::num_threads`.

- ```rust
  pub fn panic_handler<H>(self: Self, panic_handler: H) -> Configuration
where
    H: Fn(Box<dyn Any + Send>) + Send + Sync + ''static { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::panic_handler`.

- ```rust
  pub fn stack_size(self: Self, stack_size: usize) -> Self { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::stack_size`.

- ```rust
  pub fn breadth_first(self: Self) -> Self { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::breadth_first`.

- ```rust
  pub fn start_handler<H>(self: Self, start_handler: H) -> Configuration
where
    H: Fn(usize) + Send + Sync + ''static { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::start_handler`.

- ```rust
  pub fn exit_handler<H>(self: Self, exit_handler: H) -> Configuration
where
    H: Fn(usize) + Send + Sync + ''static { /* ... */ }
  ```
  Deprecated in favor of `ThreadPoolBuilder::exit_handler`.

##### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> Configuration { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `FnContext`

Provides the calling context to a closure called by `join_context`.

```rust
pub struct FnContext {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn migrated(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the closure was called from a different thread

##### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
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

- **Pointable**
  - ```rust
    unsafe fn init(init: <T as Pointable>::Init) -> usize { /* ... */ }
    ```

  - ```rust
    unsafe fn deref<''a>(ptr: usize) -> &''a T { /* ... */ }
    ```

  - ```rust
    unsafe fn deref_mut<''a>(ptr: usize) -> &''a mut T { /* ... */ }
    ```

  - ```rust
    unsafe fn drop(ptr: usize) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
## Functions

### Function `max_num_threads`

Returns the maximum number of threads that Rayon supports in a single thread-pool.

If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting
the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum.

The value may vary between different targets, and is subject to change in new Rayon versions.

```rust
pub fn max_num_threads() -> usize { /* ... */ }
```

### Function `current_num_threads`

Returns the number of threads in the current registry. If this
code is executing within a Rayon thread-pool, then this will be
the number of threads for the thread-pool of the current
thread. Otherwise, it will be the number of threads for the global
thread-pool.

This can be useful when trying to judge how many times to split
parallel work (the parallel iterator traits use this value
internally for this purpose).

# Future compatibility note

Note that unless this thread-pool was created with a
builder that specifies the number of threads, then this
number may vary over time in future versions (see [the
`num_threads()` method for details][snt]).

[snt]: struct.ThreadPoolBuilder.html#method.num_threads

```rust
pub fn current_num_threads() -> usize { /* ... */ }
```

### Function `initialize`

**Attributes:**

- `#[allow(deprecated)]`

**⚠️ Deprecated**: use `ThreadPoolBuilder::build_global`

Deprecated in favor of `ThreadPoolBuilder::build_global`.

```rust
pub fn initialize(config: Configuration) -> Result<(), Box<dyn Error>> { /* ... */ }
```

## Re-exports

### Re-export `broadcast`

```rust
pub use self::broadcast::broadcast;
```

### Re-export `spawn_broadcast`

```rust
pub use self::broadcast::spawn_broadcast;
```

### Re-export `BroadcastContext`

```rust
pub use self::broadcast::BroadcastContext;
```

### Re-export `join`

```rust
pub use self::join::join;
```

### Re-export `join_context`

```rust
pub use self::join::join_context;
```

### Re-export `ThreadBuilder`

```rust
pub use self::registry::ThreadBuilder;
```

### Re-export `in_place_scope`

```rust
pub use self::scope::in_place_scope;
```

### Re-export `scope`

```rust
pub use self::scope::scope;
```

### Re-export `Scope`

```rust
pub use self::scope::Scope;
```

### Re-export `in_place_scope_fifo`

```rust
pub use self::scope::in_place_scope_fifo;
```

### Re-export `scope_fifo`

```rust
pub use self::scope::scope_fifo;
```

### Re-export `ScopeFifo`

```rust
pub use self::scope::ScopeFifo;
```

### Re-export `spawn`

```rust
pub use self::spawn::spawn;
```

### Re-export `spawn_fifo`

```rust
pub use self::spawn::spawn_fifo;
```

### Re-export `current_thread_has_pending_tasks`

```rust
pub use self::thread_pool::current_thread_has_pending_tasks;
```

### Re-export `current_thread_index`

```rust
pub use self::thread_pool::current_thread_index;
```

### Re-export `ThreadPool`

```rust
pub use self::thread_pool::ThreadPool;
```

### Re-export `yield_local`

```rust
pub use self::thread_pool::yield_local;
```

### Re-export `yield_now`

```rust
pub use self::thread_pool::yield_now;
```

### Re-export `Yield`

```rust
pub use self::thread_pool::Yield;
```

