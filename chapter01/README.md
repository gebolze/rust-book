# Chapter 1 - Getting Started
## Installation

The recommended way to install rust is by using `rustup`, which allows management of rust versions and associated tools.

To install the lastest stable rust version for linux execute:
```bash
$ curl --proto 'https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
In addition to this you may need a linker and c compiler.

To install rust for windows visit https://www.rust-lang.org/tools/install and
follow the instructions. The easiest way to get the build tools is to install
[Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

Rust gets is frequently updated to update to the lastest version use `rustup update`.

To uninstall rust you can execute `rustup self uninstall`.

An convient feature it that rust comes bundled with the documentation, which can be opened at anytime by executing `rustup doc`.

## Hello, World!

to compile the hello world sample excute:
```bash
$ rustc ./hello_world/main.rs
$ ./main
Hello, world!
```

To avoid code formatting wars rust provides `rustfmt` a code formatter, that will format your code. For details on howto configure and use rustfmt please refer to the online documentation.

## Hello, Cargo!

Rust has it's own build and package manager `cargo`. It's the defacto standard tool used for rust project. By using `rustup` carg should already be installed for you, to verify this execute:
```bash
$ cargo --version
cargo 1.46.0 (149022b1d 2020-07-17)
```

To create a new rust project can be created using:
```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

Cargo uses the `Cargo.toml` for its configuration. The file uses the [TOML](https://github.com/toml-lang/toml) format.

To build a project managed with cargo:
```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (/home/gebolze/develop/private/rust-book/chapter01/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
```

To execute the builded program use:
```bash
$ ./target/debug/hello_cargo
Hello, world!
```
or to rebuild if needed use:
```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```

There is also a `cargo check` command, that only check the source and makes sure that it compiles, without actually generating the executable. This is quicker than `cargo build` or `cargo run`.

To build for release use: `cargo build --release`

For further information about cargo that it's [documentation](https://doc.rust-lang.org/cargo/).