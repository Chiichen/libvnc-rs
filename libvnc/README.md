# libvnc

## What's this

Higher level safe bindings of [libvncserver](https://github.com/LibVNC/libvncserver) for Rust. Although its name is libvncserver, it actually provides both server and client

## Quick Start

We build libvncserver from source as default. If you want to use prebuilt package, you can install it by following the instructions below and enable feature `pkg`.

### Install libvncserver package (Optional)

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