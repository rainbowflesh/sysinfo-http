<!-- [<img src="https://img.shields.io/github/actions/workflow/status/rainbowflesh/sysinfo-http/ci.yml?branch=master&style=round-square&logo=github" alt="CI status">](https://github.com/ClementTsang/sysinfo/actions?query=branch%3Amaster) -->

[<img src="https://img.shields.io/crates/v/sysinfo-http.svg?style=round-square&labelColor=FFFFFF" alt="crates.io link">](https://crates.io/crates/sysinfo-http)
[<img src="https://img.shields.io/badge/docs-nightly-66c2a5?style=round-square&labelColor=FFFFFF&logoColor=white&color=purple" alt="documentation">](https://github.com/rainbowflesh/sysinfo-http/blob/develop/API.md)

# sysinfo-http

`sysinfo-http` used to get a system's information through http.

## Supported OSes

It currently supports the following OSes (alphabetically sorted):

- Linux
- Raspberry Pi
- Rockchip-BSP
- Windows

You can still run a `sysinfo-http` server on non-supported OSes, it'll simply do nothing and always return
empty values. You can check in your browser directly if an OS is supported by run the server then:

```bash
curl -X GET '127.0.0.1:8000/support'
```

The minimum-supported version of `rustc` is **1.59**.

## Usage

```bash
cargo install sysinfo-http
sysinfo-http --help
```

## API

See _[API.md](./API.md)_

## Build

### Base on your PC Architect

```bash
git clone https://github.com/rainbowflesh/sysinfo-http.git
cd sysinfo-info
cargo build
```

### Cross compile

```bash
cargo build --target armv7-unknown-linux-gnueabihf
cargo build --target aarch64-unknown-linux-gnu
# or use cross-rs
cross build --target armv7-unknown-linux-gnueabihf
cross build --target aarch64-unknown-linux-gnu
```

> Notice:
> Minimal GCC is `8.3.0`.

## Run

### Run in develop

```bash
cargo run -- --help
```

### Test

```bash
cargo test
# or use nextest
cargo nextest run --no-capture
```

## Contribute

WIP
