# PKCS11 Client

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/project-template/badge.svg)](https://docs.rs/{{project-name}})
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/duclos-cavalcanti/rust-project-template/LICENSE)

## Installation

### Dependencies
- [SoftHSM](https://github.com/softhsm/SoftHSMv2)
- [Protobufs](https://protobuf.dev/)
- [Tokio](https://github.com/tokio-rs/tokio)

```bash 
sudo apt update
sudo apt install softhsm2
sudo apt install libprotobuf-c-dev libprotobuf-dev protobuf-compiler protobuf-codegen
```

### Setup
0. Verify library path: 
```bash 
find /usr/lib/ -name "libsofthsm2.so"
```

1. Initialize token:
```bash 
softhsm2-util --init-token --slot 0 --label "MyToken" --pin 1234 --so-pin 1234
```

2. Verify token slots:
```bash 
pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so --list-slots
```


## Development
1. `cargo build`: builds project and creates target folder with outputted files and executable.
2. `cargo build --release`: stores optimized executable in `target/release`.
3. `cargo run`: builds project and runs executable.
4. `cargo check`: checks if project compiles without building it.
5. `cargo fmt`: formats all rust files according to `rustfmt.toml`
6. `cargo test`: runs unit-tests.

## Documentation
- [PKCS11](https://thalesdocs.com/gphsm/ptk/5.9/docs/Content/PTK-C_Program/intro_PKCS11.htm)
- [Tokio](https://docs.rs/tokio/latest/tokio/)

