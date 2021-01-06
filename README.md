# Sliproad

**Merge filesystems together**

## About

Sliproad aims to merge together various filesystem providers, such as local filesystems and S3-compatible APIs, into a
single application. Originally written to combine external disks into a single interface for a Raspberry Pi, it is 
written for speed, simplicity and extensibility.

## Running

The Rust branch is currently still in development, and should not be considered stable. That said, if you still wish to
run it, ensure you have [installed Rust and Cargo](https://www.rust-lang.org/tools/install). Development is done against
the nightly branch, which is considered "stable enough" for daily use, although I aim to keep the codebase compatible
with stable where possible.

From there, it should be as simple as adding a simple `config.toml` file

```toml
[providers.disk]
provider = "disk"
path = ""

[general]
port = 8080
```

then running `cargo run` within the project root.

## Configuration

More to be done here as development progresses and features solidify.
