<!-- [<img src="https://img.shields.io/github/actions/workflow/status/rainbowflesh/sysinfo-http/ci.yml?branch=master&style=round-square&logo=github" alt="CI status">](https://github.com/ClementTsang/sysinfo/actions?query=branch%3Amaster) -->

[<img src="https://img.shields.io/crates/v/sysinfo-http.svg?style=round-square&labelColor=FFFFFF" alt="crates.io link">](https://crates.io/crates/sysinfo-http)
[<img src="https://img.shields.io/badge/docs-nightly-66c2a5?style=round-square&labelColor=FFFFFF&logoColor=white&color=purple" alt="documentation">](https://sysinfo-http.github.io/doc/)

# sysinfo-http

`sysinfo-http` used to get a system's information through http.

## Supported OSes

It currently supports the following OSes (alphabetically sorted):

- Linux
- Raspberry Pi
- Rockchip-BSP
- Windows

You can still run a `sysinfo-http` server on non-supported OSes, it'll simply do nothing and always return
empty values. You can check in your browser directly if an OS is supported by visit `http://yourserver.domain/support` or:

```bash
curl http://yourserver.domain/support
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

```bash
git clone https://github.com/rainbowflesh/sysinfo-http.git
cd sysinfo-info
cargo build
```

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
