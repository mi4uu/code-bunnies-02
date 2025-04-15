# Crate Documentation

**Version:** 0.12.3

**Format Version:** 43

# Module `parking_lot`

This library provides implementations of `Mutex`, `RwLock`, `Condvar` and
`Once` that are smaller, faster and more flexible than those in the Rust
standard library. It also provides a `ReentrantMutex` type.

## Re-exports

### Re-export `Condvar`

```rust
pub use self::condvar::Condvar;
```

### Re-export `WaitTimeoutResult`

```rust
pub use self::condvar::WaitTimeoutResult;
```

### Re-export `const_fair_mutex`

```rust
pub use self::fair_mutex::const_fair_mutex;
```

### Re-export `FairMutex`

```rust
pub use self::fair_mutex::FairMutex;
```

### Re-export `FairMutexGuard`

```rust
pub use self::fair_mutex::FairMutexGuard;
```

### Re-export `MappedFairMutexGuard`

```rust
pub use self::fair_mutex::MappedFairMutexGuard;
```

### Re-export `const_mutex`

```rust
pub use self::mutex::const_mutex;
```

### Re-export `MappedMutexGuard`

```rust
pub use self::mutex::MappedMutexGuard;
```

### Re-export `Mutex`

```rust
pub use self::mutex::Mutex;
```

### Re-export `MutexGuard`

```rust
pub use self::mutex::MutexGuard;
```

### Re-export `Once`

```rust
pub use self::once::Once;
```

### Re-export `OnceState`

```rust
pub use self::once::OnceState;
```

### Re-export `RawFairMutex`

```rust
pub use self::raw_fair_mutex::RawFairMutex;
```

### Re-export `RawMutex`

```rust
pub use self::raw_mutex::RawMutex;
```

### Re-export `RawRwLock`

```rust
pub use self::raw_rwlock::RawRwLock;
```

### Re-export `const_reentrant_mutex`

```rust
pub use self::remutex::const_reentrant_mutex;
```

### Re-export `MappedReentrantMutexGuard`

```rust
pub use self::remutex::MappedReentrantMutexGuard;
```

### Re-export `RawThreadId`

```rust
pub use self::remutex::RawThreadId;
```

### Re-export `ReentrantMutex`

```rust
pub use self::remutex::ReentrantMutex;
```

### Re-export `ReentrantMutexGuard`

```rust
pub use self::remutex::ReentrantMutexGuard;
```

### Re-export `const_rwlock`

```rust
pub use self::rwlock::const_rwlock;
```

### Re-export `MappedRwLockReadGuard`

```rust
pub use self::rwlock::MappedRwLockReadGuard;
```

### Re-export `MappedRwLockWriteGuard`

```rust
pub use self::rwlock::MappedRwLockWriteGuard;
```

### Re-export `RwLock`

```rust
pub use self::rwlock::RwLock;
```

### Re-export `RwLockReadGuard`

```rust
pub use self::rwlock::RwLockReadGuard;
```

### Re-export `RwLockUpgradableReadGuard`

```rust
pub use self::rwlock::RwLockUpgradableReadGuard;
```

### Re-export `RwLockWriteGuard`

```rust
pub use self::rwlock::RwLockWriteGuard;
```

### Re-export `lock_api`

```rust
pub use ::lock_api;
```

