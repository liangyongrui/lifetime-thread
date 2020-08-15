<h1 align="center">Lifetime Thread</h1>
<div align="center">
 <strong>
    A thread with a lifetime. Divide a value into master and slave. After the lifetime of the master value ends, the slave value will not be accessible.
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/lifetime-thread">
    <img src="https://img.shields.io/crates/v/lifetime-thread.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/lifetime-thread">
    <img src="https://img.shields.io/crates/d/lifetime-thread.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/lifetime-thread">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- ci -->
  <a href="https://docs.rs/lifetime-thread">
    <img src="https://github.com/liangyongrui/lifetime-thread/workflows/Rust/badge.svg"
      alt="ci" />
  </a>
  <!-- coverage -->
  <a href="https://codecov.io/gh/liangyongrui/lifetime-thread">
    <img src="https://codecov.io/gh/liangyongrui/lifetime-thread/branch/master/graph/badge.svg" />
  </a>
</div>

<br/>

## Introduction

A thread with a lifetime. Divide a value into master and slave. After the lifetime of the master value ends, the slave value will not be accessible.

scenes to be used:

- An operation needs to be performed in the background, but the lifetime is not static
- ...

## Basic usage

```rust
use std::{thread, time::Duration};

#[test]
fn it_works() {
    let s = String::from("xxx");
    let outer = lifetime_thread::spawn(s, |inner| {
        println!("begin");
        while let Some(t) = inner.get() {
            println!("ok! {}", t);
            assert_eq!(t, "xxx")
        }
        println!("over")
    });
    thread::sleep(Duration::from_millis(1));
    assert_eq!(*outer, "xxx")
}
```

output:

```text
begin
ok! xxx
ok! xxx
...
...
ok! xxx
over
```

## Features

- [ ] Different runtime
  - [x] sync (Multithreading): lifetime_thread::spawn
  - [x] async-std: lifetime_thread::async_spawn
  - [ ] tokio

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions
