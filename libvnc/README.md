# libvnc

## What's this

[libvncserver](https://github.com/LibVNC/libvncserver) safe bindings for Rust. Although its name is libvncserver, it actually provides both server and client functions

## Quick Start

`Build from source` is not implemented yet, so we need to install the pre-built libvncserver package.

### Install libvncserver package

- Ubuntu
```bash 
sudo apt-get install libvncserver-dev
```

- Centos
```bash
sudo yum install libvncserver-devel
```

- Macos
```bash
brew install libvncserver
```

### Run

examples can be found at `examples`

```bash
#!! Start a vnc server at 127.0.0.1:5900 before running the example
cargo run --bin image_capture
```