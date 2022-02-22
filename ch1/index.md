# Getting Started

## 1. Installation

- MAC:

```
$ curl https://sh.rustup.rs -sSf | sh
```

- Window:
  https://www.rust-lang.org/en-US/install.html

And Check version

```
$ rustc --version
```

## Hello, World!

- with rustc

```rust
fn main() {
    println!("Hello, world!");
}
```

and compile the source

```
$ rustc main.rs
$ ./main
Hello, world!
```

- with cargo

Cargo is Rust’s build system and package manager.(like npm)

```
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

cargo.toml

```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

build: cargo build
run: cargo run
check: cargo compile -> just complie doesn’t produce an executable
