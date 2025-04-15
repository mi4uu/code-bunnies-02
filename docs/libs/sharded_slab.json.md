# Crate Documentation

**Version:** 0.1.7

**Format Version:** 43

# Module `sharded_slab`

 A lock-free concurrent slab.

 Slabs provide pre-allocated storage for many instances of a single data
 type. When a large number of values of a single type are required,
 this can be more efficient than allocating each item individually. Since the
 allocated items are the same size, memory fragmentation is reduced, and
 creating and removing new items can be very cheap.

 This crate implements a lock-free concurrent slab, indexed by `usize`s.

 ## Usage

 First, add this to your `Cargo.toml`:

 ```toml
 sharded-slab = "0.1.1"
 ```

 This crate provides two  types, [`Slab`] and [`Pool`], which provide
 slightly different APIs for using a sharded slab.

 [`Slab`] implements a slab for _storing_ small types, sharing them between
 threads, and accessing them by index. New entries are allocated by
 [inserting] data, moving it in by value. Similarly, entries may be
 deallocated by [taking] from the slab, moving the value out. This API is
 similar to a `Vec<Option<T>>`, but allowing lock-free concurrent insertion
 and removal.

 In contrast, the [`Pool`] type provides an [object pool] style API for
 _reusing storage_. Rather than constructing values and moving them into the
 pool, as with [`Slab`], [allocating an entry][create] from the pool takes a
 closure that's provided with a mutable reference to initialize the entry in
 place. When entries are deallocated, they are [cleared] in place. Types
 which own a heap allocation can be cleared by dropping any _data_ they
 store, but retaining any previously-allocated capacity. This means that a
 [`Pool`] may be used to reuse a set of existing heap allocations, reducing
 allocator load.

 [inserting]: Slab::insert
 [taking]: Slab::take
 [create]: Pool::create
 [cleared]: Clear
 [object pool]: https://en.wikipedia.org/wiki/Object_pool_pattern

 # Examples

 Inserting an item into the slab, returning an index:
 ```rust
 # use sharded_slab::Slab;
 let slab = Slab::new();

 let key = slab.insert("hello world").unwrap();
 assert_eq!(slab.get(key).unwrap(), "hello world");
 ```

 To share a slab across threads, it may be wrapped in an `Arc`:
 ```rust
 # use sharded_slab::Slab;
 use std::sync::Arc;
 let slab = Arc::new(Slab::new());

 let slab2 = slab.clone();
 let thread2 = std::thread::spawn(move || {
     let key = slab2.insert("hello from thread two").unwrap();
     assert_eq!(slab2.get(key).unwrap(), "hello from thread two");
     key
 });

 let key1 = slab.insert("hello from thread one").unwrap();
 assert_eq!(slab.get(key1).unwrap(), "hello from thread one");

 // Wait for thread 2 to complete.
 let key2 = thread2.join().unwrap();

 // The item inserted by thread 2 remains in the slab.
 assert_eq!(slab.get(key2).unwrap(), "hello from thread two");
```

 If items in the slab must be mutated, a `Mutex` or `RwLock` may be used for
 each item, providing granular locking of items rather than of the slab:

 ```rust
 # use sharded_slab::Slab;
 use std::sync::{Arc, Mutex};
 let slab = Arc::new(Slab::new());

 let key = slab.insert(Mutex::new(String::from("hello world"))).unwrap();

 let slab2 = slab.clone();
 let thread2 = std::thread::spawn(move || {
     let hello = slab2.get(key).expect("item missing");
     let mut hello = hello.lock().expect("mutex poisoned");
     *hello = String::from("hello everyone!");
 });

 thread2.join().unwrap();

 let hello = slab.get(key).expect("item missing");
 let mut hello = hello.lock().expect("mutex poisoned");
 assert_eq!(hello.as_str(), "hello everyone!");
 ```

 # Configuration

 For performance reasons, several values used by the slab are calculated as
 constants. In order to allow users to tune the slab's parameters, we provide
 a [`Config`] trait which defines these parameters as associated `consts`.
 The `Slab` type is generic over a `C: Config` parameter.

 [`Config`]: trait.Config.html

 # Comparison with Similar Crates

 - [`slab`]: Carl Lerche's `slab` crate provides a slab implementation with a
   similar API, implemented by storing all data in a single vector.

   Unlike `sharded_slab`, inserting and removing elements from the slab
   requires  mutable access. This means that if the slab is accessed
   concurrently by multiple threads, it is necessary for it to be protected
   by a `Mutex` or `RwLock`. Items may not be inserted or removed (or
   accessed, if a `Mutex` is used) concurrently, even when they are
   unrelated. In many cases, the lock can become a significant bottleneck. On
   the other hand, this crate allows separate indices in the slab to be
   accessed, inserted, and removed concurrently without requiring a global
   lock. Therefore, when the slab is shared across multiple threads, this
   crate offers significantly better performance than `slab`.

   However, the lock free slab introduces some additional constant-factor
   overhead. This means that in use-cases where a slab is _not_ shared by
   multiple threads and locking is not required, this crate will likely offer
   slightly worse performance.

   In summary: `sharded-slab` offers significantly improved performance in
   concurrent use-cases, while `slab` should be preferred in single-threaded
   use-cases.

 [`slab`]: https://crates.io/crates/loom

 # Safety and Correctness

 Most implementations of lock-free data structures in Rust require some
 amount of unsafe code, and this crate is not an exception. In order to catch
 potential bugs in this unsafe code, we make use of [`loom`], a
 permutation-testing tool for concurrent Rust programs. All `unsafe` blocks
 this crate occur in accesses to `loom` `UnsafeCell`s. This means that when
 those accesses occur in this crate's tests, `loom` will assert that they are
 valid under the C11 memory model across multiple permutations of concurrent
 executions of those tests.

 In order to guard against the [ABA problem][aba], this crate makes use of
 _generational indices_. Each slot in the slab tracks a generation counter
 which is incremented every time a value is inserted into that slot, and the
 indices returned by [`Slab::insert`] include the generation of the slot when
 the value was inserted, packed into the high-order bits of the index. This
 ensures that if a value is inserted, removed,  and a new value is inserted
 into the same slot in the slab, the key returned by the first call to
 `insert` will not map to the new value.

 Since a fixed number of bits are set aside to use for storing the generation
 counter, the counter will wrap  around after being incremented a number of
 times. To avoid situations where a returned index lives long enough to see the
 generation counter wrap around to the same value, it is good to be fairly
 generous when configuring the allocation of index bits.

 [`loom`]: https://crates.io/crates/loom
 [aba]: https://en.wikipedia.org/wiki/ABA_problem
 [`Slab::insert`]: struct.Slab.html#method.insert

 # Performance

 These graphs were produced by [benchmarks] of the sharded slab implementation,
 using the [`criterion`] crate.

 The first shows the results of a benchmark where an increasing number of
 items are inserted and then removed into a slab concurrently by five
 threads. It compares the performance of the sharded slab implementation
 with a `RwLock<slab::Slab>`:

 <img width="1124" alt="Screen Shot 2019-10-01 at 5 09 49 PM" src="https://user-images.githubusercontent.com/2796466/66078398-cd6c9f80-e516-11e9-9923-0ed6292e8498.png">

 The second graph shows the results of a benchmark where an increasing
 number of items are inserted and then removed by a _single_ thread. It
 compares the performance of the sharded slab implementation with an
 `RwLock<slab::Slab>` and a `mut slab::Slab`.

 <img width="925" alt="Screen Shot 2019-10-01 at 5 13 45 PM" src="https://user-images.githubusercontent.com/2796466/66078469-f0974f00-e516-11e9-95b5-f65f0aa7e494.png">

 These benchmarks demonstrate that, while the sharded approach introduces
 a small constant-factor overhead, it offers significantly better
 performance across concurrent accesses.

 [benchmarks]: https://github.com/hawkw/sharded-slab/blob/master/benches/bench.rs
 [`criterion`]: https://crates.io/crates/criterion

 # Implementation Notes

 See [this page](crate::implementation) for details on this crate's design
 and implementation.


## Modules

## Module `implementation`

Notes on `sharded-slab`'s implementation and design.

# Design

The sharded slab's design is strongly inspired by the ideas presented by
Leijen, Zorn, and de Moura in [Mimalloc: Free List Sharding in
Action][mimalloc]. In this report, the authors present a novel design for a
memory allocator based on a concept of _free list sharding_.

Memory allocators must keep track of what memory regions are not currently
allocated ("free") in order to provide them to future allocation requests.
The term [_free list_][freelist] refers to a technique for performing this
bookkeeping, where each free block stores a pointer to the next free block,
forming a linked list. The memory allocator keeps a pointer to the most
recently freed block, the _head_ of the free list. To allocate more memory,
the allocator pops from the free list by setting the head pointer to the
next free block of the current head block, and returning the previous head.
To deallocate a block, the block is pushed to the free list by setting its
first word to the current head pointer, and the head pointer is set to point
to the deallocated block. Most implementations of slab allocators backed by
arrays or vectors use a similar technique, where pointers are replaced by
indices into the backing array.

When allocations and deallocations can occur concurrently across threads,
they must synchronize accesses to the free list; either by putting the
entire allocator state inside of a lock, or by using atomic operations to
treat the free list as a lock-free structure (such as a [Treiber stack]). In
both cases, there is a significant performance cost — even when the free
list is lock-free, it is likely that a noticeable amount of time will be
spent in compare-and-swap loops. Ideally, the global synchronzation point
created by the single global free list could be avoided as much as possible.

The approach presented by Leijen, Zorn, and de Moura is to introduce
sharding and thus increase the granularity of synchronization significantly.
In mimalloc, the heap is _sharded_ so that each thread has its own
thread-local heap. Objects are always allocated from the local heap of the
thread where the allocation is performed. Because allocations are always
done from a thread's local heap, they need not be synchronized.

However, since objects can move between threads before being deallocated,
_deallocations_ may still occur concurrently. Therefore, Leijen et al.
introduce a concept of _local_ and _global_ free lists. When an object is
deallocated on the same thread it was originally allocated on, it is placed
on the local free list; if it is deallocated on another thread, it goes on
the global free list for the heap of the thread from which it originated. To
allocate, the local free list is used first; if it is empty, the entire
global free list is popped onto the local free list. Since the local free
list is only ever accessed by the thread it belongs to, it does not require
synchronization at all, and because the global free list is popped from
infrequently, the cost of synchronization has a reduced impact. A majority
of allocations can occur without any synchronization at all; and
deallocations only require synchronization when an object has left its
parent thread (a relatively uncommon case).

[mimalloc]: https://www.microsoft.com/en-us/research/uploads/prod/2019/06/mimalloc-tr-v1.pdf
[freelist]: https://en.wikipedia.org/wiki/Free_list
[Treiber stack]: https://en.wikipedia.org/wiki/Treiber_stack

# Implementation

A slab is represented as an array of [`MAX_THREADS`] _shards_. A shard
consists of a vector of one or more _pages_ plus associated metadata.
Finally, a page consists of an array of _slots_, head indices for the local
and remote free lists.

```text
┌─────────────┐
│ shard 1     │
│             │    ┌─────────────┐        ┌────────┐
│ pages───────┼───▶│ page 1      │        │        │
├─────────────┤    ├─────────────┤  ┌────▶│  next──┼─┐
│ shard 2     │    │ page 2      │  │     ├────────┤ │
├─────────────┤    │             │  │     │XXXXXXXX│ │
│ shard 3     │    │ local_head──┼──┘     ├────────┤ │
└─────────────┘    │ remote_head─┼──┐     │        │◀┘
      ...          ├─────────────┤  │     │  next──┼─┐
┌─────────────┐    │ page 3      │  │     ├────────┤ │
│ shard n     │    └─────────────┘  │     │XXXXXXXX│ │
└─────────────┘          ...        │     ├────────┤ │
                   ┌─────────────┐  │     │XXXXXXXX│ │
                   │ page n      │  │     ├────────┤ │
                   └─────────────┘  │     │        │◀┘
                                    └────▶│  next──┼───▶  ...
                                          ├────────┤
                                          │XXXXXXXX│
                                          └────────┘
```


The size of the first page in a shard is always a power of two, and every
subsequent page added after the first is twice as large as the page that
preceeds it.

```text

pg.
┌───┐   ┌─┬─┐
│ 0 │───▶ │ │
├───┤   ├─┼─┼─┬─┐
│ 1 │───▶ │ │ │ │
├───┤   ├─┼─┼─┼─┼─┬─┬─┬─┐
│ 2 │───▶ │ │ │ │ │ │ │ │
├───┤   ├─┼─┼─┼─┼─┼─┼─┼─┼─┬─┬─┬─┬─┬─┬─┬─┐
│ 3 │───▶ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │ │
└───┘   └─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
```

When searching for a free slot, the smallest page is searched first, and if
it is full, the search proceeds to the next page until either a free slot is
found or all available pages have been searched. If all available pages have
been searched and the maximum number of pages has not yet been reached, a
new page is then allocated.

Since every page is twice as large as the previous page, and all page sizes
are powers of two, we can determine the page index that contains a given
address by shifting the address down by the smallest page size and
looking at how many twos places necessary to represent that number,
telling us what power of two page size it fits inside of. We can
determine the number of twos places by counting the number of leading
zeros (unused twos places) in the number's binary representation, and
subtracting that count from the total number of bits in a word.

The formula for determining the page number that contains an offset is thus:

```rust,ignore
WIDTH - ((offset + INITIAL_PAGE_SIZE) >> INDEX_SHIFT).leading_zeros()
```

where `WIDTH` is the number of bits in a `usize`, and `INDEX_SHIFT` is

```rust,ignore
INITIAL_PAGE_SIZE.trailing_zeros() + 1;
```

[`MAX_THREADS`]: https://docs.rs/sharded-slab/latest/sharded_slab/trait.Config.html#associatedconstant.MAX_THREADS

```rust
pub mod implementation { /* ... */ }
```

## Module `pool`

A lock-free concurrent object pool.

See the [`Pool` type's documentation][pool] for details on the object pool API and how
it differs from the [`Slab`] API.

[pool]: ../struct.Pool.html
[`Slab`]: ../struct.Slab.html

```rust
pub mod pool { /* ... */ }
```

### Types

#### Struct `Pool`

A lock-free concurrent object pool.

Slabs provide pre-allocated storage for many instances of a single type. But, when working with
heap allocated objects, the advantages of a slab are lost, as the memory allocated for the
object is freed when the object is removed from the slab. With a pool, we can instead reuse
this memory for objects being added to the pool in the future, therefore reducing memory
fragmentation and avoiding additional allocations.

This type implements a lock-free concurrent pool, indexed by `usize`s. The items stored in this
type need to implement [`Clear`] and `Default`.

The `Pool` type shares similar semantics to [`Slab`] when it comes to sharing across threads
and storing mutable shared data. The biggest difference is there are no [`Slab::insert`] and
[`Slab::take`] analouges for the `Pool` type. Instead new items are added to the pool by using
the [`Pool::create`] method, and marked for clearing by the [`Pool::clear`] method.

# Examples

Add an entry to the pool, returning an index:
```
# use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let key = pool.create_with(|item| item.push_str("hello world")).unwrap();
assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Create a new pooled item, returning a guard that allows mutable access:
```
# use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let mut guard = pool.create().unwrap();
let key = guard.key();
guard.push_str("hello world");

drop(guard); // release the guard, allowing immutable access.
assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Pool entries can be cleared by calling [`Pool::clear`]. This marks the entry to
be cleared when the guards referencing to it are dropped.
```
# use sharded_slab::Pool;
let pool: Pool<String> = Pool::new();

let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Mark this entry to be cleared.
pool.clear(key);

// The cleared entry is no longer available in the pool
assert!(pool.get(key).is_none());
```
# Configuration

Both `Pool` and [`Slab`] share the same configuration mechanism. See [crate level documentation][config-doc]
for more details.

[`Slab::take`]: crate::Slab::take
[`Slab::insert`]: crate::Slab::insert
[`Pool::create`]: Pool::create
[`Pool::clear`]: Pool::clear
[config-doc]: crate#configuration
[`Clear`]: crate::Clear
[`Slab`]: crate::Slab

```rust
pub struct Pool<T, C = crate::cfg::DefaultConfig> {
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
  Returns a new `Pool` with the default configuration parameters.

- ```rust
  pub fn new_with_config<C: cfg::Config>() -> Pool<T, C> { /* ... */ }
  ```
  Returns a new `Pool` with the provided configuration parameters.

- ```rust
  pub fn create(self: &Self) -> Option<RefMut<''_, T, C>> { /* ... */ }
  ```
  Creates a new object in the pool, returning an [`RefMut`] guard that

- ```rust
  pub fn create_owned(self: Arc<Self>) -> Option<OwnedRefMut<T, C>> { /* ... */ }
  ```
  Creates a new object in the pool, returning an [`OwnedRefMut`] guard that

- ```rust
  pub fn create_with</* synthetic */ impl FnOnce(&mut T): FnOnce(&mut T)>(self: &Self, init: impl FnOnce(&mut T)) -> Option<usize> { /* ... */ }
  ```
  Creates a new object in the pool with the provided initializer,

- ```rust
  pub fn get(self: &Self, key: usize) -> Option<Ref<''_, T, C>> { /* ... */ }
  ```
  Return a borrowed reference to the value associated with the given key.

- ```rust
  pub fn get_owned(self: Arc<Self>, key: usize) -> Option<OwnedRef<T, C>> { /* ... */ }
  ```
  Return an owned reference to the value associated with the given key.

- ```rust
  pub fn clear(self: &Self, key: usize) -> bool { /* ... */ }
  ```
  Remove the value using the storage associated with the given key from the pool, returning

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

#### Struct `Ref`

A guard that allows access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard references is
currently being accessed. If the item is removed from the pool while the guard exists, the
removal will be deferred until all guards are dropped.

```rust
pub struct Ref<''a, T, C = crate::cfg::DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access this guard

###### Trait Implementations

- **Send**
- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Freeze**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

#### Struct `RefMut`

A guard that allows exclusive mutable access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard
references is currently being accessed. If the item is removed from the pool
while a guard exists, the removal will be deferred until the guard is
dropped. The slot cannot be accessed by other threads while it is accessed
mutably.

```rust
pub struct RefMut<''a, T, C = crate::cfg::DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access the guard.

- ```rust
  pub fn downgrade(self: Self) -> Ref<''a, T, C> { /* ... */ }
  ```
  Downgrades the mutable guard to an immutable guard, allowing access to

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Receiver**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `OwnedRef`

An owned guard that allows shared immutable access to an object in a pool.

While the guard exists, it indicates to the pool that the item the guard references is
currently being accessed. If the item is removed from the pool while the guard exists, the
removal will be deferred until all guards are dropped.

Unlike [`Ref`], which borrows the pool, an `OwnedRef` clones the `Arc`
around the pool. Therefore, it keeps the pool from being dropped until all
such guards have been dropped. This means that an `OwnedRef` may be held for
an arbitrary lifetime.


# Examples

```
# use sharded_slab::Pool;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let value = pool.clone().get_owned(key).unwrap();

// Now, the original `Arc` clone of the pool may be dropped, but the
// returned `OwnedRef` can still access the value.
assert_eq!(value, String::from("hello world"));
```

Unlike [`Ref`], an `OwnedRef` may be stored in a struct which must live
for the `'static` lifetime:

```
# use sharded_slab::Pool;
use sharded_slab::pool::OwnedRef;
use std::sync::Arc;

pub struct MyStruct {
    pool_ref: OwnedRef<String>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let pool_ref = pool.clone().get_owned(key).unwrap();
let my_struct = MyStruct {
    pool_ref,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedRef`s may be sent between threads:

```
# use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());
let key = pool.create_with(|item| item.push_str("hello world")).unwrap();

// Look up the created `Key`, returning an `OwnedRef`.
let value = pool.clone().get_owned(key).unwrap();

thread::spawn(move || {
    assert_eq!(value, String::from("hello world"));
    // ...
}).join().unwrap();
```

[`Ref`]: crate::pool::Ref

```rust
pub struct OwnedRef<T, C = crate::cfg::DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access this guard

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Receiver**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
#### Struct `OwnedRefMut`

An owned guard that allows exclusive, mutable access to an object in a pool.

An `OwnedRefMut<T>` functions more or less identically to an owned
`Box<T>`: it can be passed to functions, stored in structure fields, and
borrowed mutably or immutably, and can be owned for arbitrary lifetimes.
The difference is that, unlike a `Box<T>`, the memory allocation for the
`T` lives in the `Pool`; when an `OwnedRefMut` is created, it may reuse
memory that was allocated for a previous pooled object that has been
cleared. Additionally, the `OwnedRefMut` may be [downgraded] to an
[`OwnedRef`] which may be shared freely, essentially turning the `Box`
into an `Arc`.

This is returned by [`Pool::create_owned`].

While the guard exists, it indicates to the pool that the item the guard
references is currently being accessed. If the item is removed from the pool
while the guard exists, theremoval will be deferred until all guards are
dropped.

Unlike [`RefMut`], which borrows the pool, an `OwnedRefMut` clones the `Arc`
around the pool. Therefore, it keeps the pool from being dropped until all
such guards have been dropped. This means that an `OwnedRefMut` may be held for
an arbitrary lifetime.

# Examples

```rust
# use sharded_slab::Pool;
# use std::thread;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new pooled item, returning an owned guard that allows mutable
// access to the new item.
let mut item = pool.clone().create_owned().unwrap();
// Return a key that allows indexing the created item once the guard
// has been dropped.
let key = item.key();

// Mutate the item.
item.push_str("Hello");
// Drop the guard, releasing mutable access to the new item.
drop(item);

/// Other threads may now (immutably) access the item using the returned key.
thread::spawn(move || {
   assert_eq!(pool.get(key).unwrap(), String::from("Hello"));
}).join().unwrap();
```

```rust
# use sharded_slab::Pool;
use std::sync::Arc;

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new item, returning an owned, mutable guard.
let mut value = pool.clone().create_owned().unwrap();

// Now, the original `Arc` clone of the pool may be dropped, but the
// returned `OwnedRefMut` can still access the value.
drop(pool);

value.push_str("hello world");
assert_eq!(value, String::from("hello world"));
```

Unlike [`RefMut`], an `OwnedRefMut` may be stored in a struct which must live
for the `'static` lifetime:

```
# use sharded_slab::Pool;
use sharded_slab::pool::OwnedRefMut;
use std::sync::Arc;

pub struct MyStruct {
    pool_ref: OwnedRefMut<String>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

// Create a new item, returning a mutable owned reference.
let pool_ref = pool.clone().create_owned().unwrap();

let my_struct = MyStruct {
    pool_ref,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedRefMut`s may be sent between threads:

```
# use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

let mut value = pool.clone().create_owned().unwrap();
let key = value.key();

thread::spawn(move || {
    value.push_str("hello world");
    // ...
}).join().unwrap();

// Once the `OwnedRefMut` has been dropped by the other thread, we may
// now access the value immutably on this thread.

assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
```

Downgrading from a mutable to an immutable reference:

```
# use sharded_slab::Pool;
use std::{thread, sync::Arc};

let pool: Arc<Pool<String>> = Arc::new(Pool::new());

let mut value = pool.clone().create_owned().unwrap();
let key = value.key();
value.push_str("hello world");

// Downgrade the mutable owned ref to an immutable owned ref.
let value = value.downgrade();

// Once the `OwnedRefMut` has been downgraded, other threads may
// immutably access the pooled value:
thread::spawn(move || {
    assert_eq!(pool.get(key).unwrap(), String::from("hello world"));
}).join().unwrap();

// This thread can still access the pooled value through the
// immutable owned ref:
assert_eq!(value, String::from("hello world"));
```

[`Pool::create_owned`]: crate::Pool::create_owned
[`RefMut`]: crate::pool::RefMut
[`OwnedRefMut`]: crate::pool::OwnedRefMut
[downgraded]: crate::pool::OwnedRefMut::downgrade

```rust
pub struct OwnedRefMut<T, C = crate::cfg::DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access this guard

- ```rust
  pub fn downgrade(self: Self) -> OwnedRef<T, C> { /* ... */ }
  ```
  Downgrades the owned mutable guard to an owned immutable guard, allowing

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Sync**
## Types

### Struct `Slab`

A sharded slab.

See the [crate-level documentation](crate) for details on using this type.

```rust
pub struct Slab<T, C: cfg::Config = DefaultConfig> {
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
  Returns a new slab with the default configuration parameters.

- ```rust
  pub fn new_with_config<C: cfg::Config>() -> Slab<T, C> { /* ... */ }
  ```
  Returns a new slab with the provided configuration parameters.

- ```rust
  pub fn insert(self: &Self, value: T) -> Option<usize> { /* ... */ }
  ```
  Inserts a value into the slab, returning the integer index at which that

- ```rust
  pub fn vacant_entry(self: &Self) -> Option<VacantEntry<''_, T, C>> { /* ... */ }
  ```
  Return a handle to a vacant entry allowing for further manipulation.

- ```rust
  pub fn remove(self: &Self, idx: usize) -> bool { /* ... */ }
  ```
  Remove the value at the given index in the slab, returning `true` if a

- ```rust
  pub fn take(self: &Self, idx: usize) -> Option<T> { /* ... */ }
  ```
  Removes the value associated with the given key from the slab, returning

- ```rust
  pub fn get(self: &Self, key: usize) -> Option<Entry<''_, T, C>> { /* ... */ }
  ```
  Return a reference to the value associated with the given key.

- ```rust
  pub fn get_owned(self: Arc<Self>, key: usize) -> Option<OwnedEntry<T, C>> { /* ... */ }
  ```
  Return an owned reference to the value at the given index.

- ```rust
  pub fn contains(self: &Self, key: usize) -> bool { /* ... */ }
  ```
  Returns `true` if the slab contains a value for the given key.

- ```rust
  pub fn unique_iter(self: &mut Self) -> iter::UniqueIter<''_, T, C> { /* ... */ }
  ```
  Returns an iterator over all the items in the slab.

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `Entry`

A handle that allows access to an occupied entry in a [`Slab`].

While the guard exists, it indicates to the slab that the item the guard
references is currently being accessed. If the item is removed from the slab
while a guard exists, the removal will be deferred until all guards are
dropped.

```rust
pub struct Entry<''a, T, C: cfg::Config = DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access the guard.

##### Trait Implementations

- **Freeze**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `VacantEntry`

A handle to a vacant entry in a [`Slab`].

`VacantEntry` allows constructing values with the key that they will be
assigned to.

# Examples

```
# use sharded_slab::Slab;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry().unwrap();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab.get(hello).unwrap().0);
assert_eq!("hello", slab.get(hello).unwrap().1);
```

```rust
pub struct VacantEntry<''a, T, C: cfg::Config = DefaultConfig> {
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
  pub fn insert(self: Self, val: T) { /* ... */ }
  ```
  Insert a value in the entry.

- ```rust
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Return the integer index at which this entry will be inserted.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Struct `OwnedEntry`

An owned reference to an occupied entry in a [`Slab`].

While the guard exists, it indicates to the slab that the item the guard
references is currently being accessed. If the item is removed from the slab
while the guard exists, the  removal will be deferred until all guards are
dropped.

Unlike [`Entry`], which borrows the slab, an `OwnedEntry` clones the [`Arc`]
around the slab. Therefore, it keeps the slab from being dropped until all
such guards have been dropped. This means that an `OwnedEntry` may be held for
an arbitrary lifetime.

# Examples

```
# use sharded_slab::Slab;
use std::sync::Arc;

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let value = slab.clone().get_owned(key).unwrap();

// Now, the original `Arc` clone of the slab may be dropped, but the
// returned `OwnedEntry` can still access the value.
assert_eq!(value, "hello world");
```

Unlike [`Entry`], an `OwnedEntry` may be stored in a struct which must live
for the `'static` lifetime:

```
# use sharded_slab::Slab;
use sharded_slab::OwnedEntry;
use std::sync::Arc;

pub struct MyStruct {
    entry: OwnedEntry<&'static str>,
    // ... other fields ...
}

// Suppose this is some arbitrary function which requires a value that
// lives for the 'static lifetime...
fn function_requiring_static<T: 'static>(t: &T) {
    // ... do something extremely important and interesting ...
}

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let entry = slab.clone().get_owned(key).unwrap();
let my_struct = MyStruct {
    entry,
    // ...
};

// We can use `my_struct` anywhere where it is required to have the
// `'static` lifetime:
function_requiring_static(&my_struct);
```

`OwnedEntry`s may be sent between threads:

```
# use sharded_slab::Slab;
use std::{thread, sync::Arc};

let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
let key = slab.insert("hello world").unwrap();

// Look up the created key, returning an `OwnedEntry`.
let value = slab.clone().get_owned(key).unwrap();

thread::spawn(move || {
    assert_eq!(value, "hello world");
    // ...
}).join().unwrap();
```

[`get`]: Slab::get
[`Arc`]: std::sync::Arc

```rust
pub struct OwnedEntry<T, C = DefaultConfig> {
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
  pub fn key(self: &Self) -> usize { /* ... */ }
  ```
  Returns the key used to access this guard

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **Receiver**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `Config`

```rust
pub use self::cfg::Config;
```

### Re-export `DefaultConfig`

```rust
pub use self::cfg::DefaultConfig;
```

### Re-export `Clear`

```rust
pub use self::clear::Clear;
```

### Re-export `UniqueIter`

```rust
pub use self::iter::UniqueIter;
```

### Re-export `Pool`

**Attributes:**

- `#[doc(inline)]`

```rust
pub use pool::Pool;
```

