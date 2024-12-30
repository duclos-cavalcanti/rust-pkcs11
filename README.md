# PKCS11 Client

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/project-template/badge.svg)](https://docs.rs/{{project-name}})
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/duclos-cavalcanti/rust-project-template/LICENSE)

## Installation

### Dependencies
- [SoftHSM](https://github.com/softhsm/SoftHSMv2)
- [Cryptoki](https://github.com/parallaxsecond/rust-cryptoki)
- [Tokio](https://github.com/tokio-rs/tokio)

```bash 
sudo apt update
sudo apt install softhsm2 opensc
```

### Setup
0. Verify library path: 
```bash 
find /usr/lib/ -name "libsofthsm2.so"
```

1. Add user to `softhsm` group:
```bash
sudo usermod -aG softhsm $(whoami)
```

2. Initialize token:
```bash 
softhsm2-util --init-token --slot 0 --label "MyToken" --pin 1234 --so-pin 1234
```

3. Verify token slots:
```bash 
softhsm2-util --show-slots
```

4. Generate key-pair 
```bash 
pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so --login --pin 1234 --keypairgen --key-type rsa:2048 --id 01 --label "TestKey"
```

5. Verify key-pair 
```bash 
pkcs11-tool --module /usr/lib/softhsm/libsofthsm2.so --login --pin 1234 --list-objects
```

## Client
### Dependencies

```bash 
sudo apt-get install libncurses5-dev libncursesw5-dev
```

## Documentation
- [PKCS11](https://thalesdocs.com/gphsm/ptk/5.9/docs/Content/PTK-C_Program/intro_PKCS11.htm)
- [Tokio](https://docs.rs/tokio/latest/tokio/)

