# Rust Barriers

A barrier spin lock implementation in rust!

## Installing

Just add the below to your `Cargo.toml` dependencies.

```
barriers = "<version number>"
```

## Usage

A barrier must first be initialized before using. A new barrier can be created by using the `init` method and specifying the count. This barrier is a simple counting barrier that spins on a `sense` variable. 

```rust
let barr = barrier::Barrier::init(4); // 4 is the number of threads
```

Since barriers are generally shared between threads, it is a good idea to use an `Arc`

```rust
let barr = Arc::new(barrier::Barrier::init(4));
```

and then clone it before moving it to a new thread

```rust
for _ in 0..4 {
    let barr_clone = Arc::clone(barr);
    thread::spawn(move || {
        barr_clone.arrive();
    });
}
```

To pick a synchronization point, simply call the `arrive` method.

```rust
barr_clone.arrive();
```