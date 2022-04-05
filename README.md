# Gemor kernel

## Introduction

**Welcome to the Gemor kernel repository!**

This repository is intended to be my sandbox for diving into a kernel
development along with Rust programming language.

Many tutorials that I've watched, were compromising complex kernel development
with simplicity for the sake of introduction. This will not take place here. I
want to create a system with up-to-date standards which include UEFI and 64-bit
processors.

## Building

Composing a kernel image requires more than just compilation.
`simple_boot` package is used to create a bootable image.
The usage if this package during build is configured in .cargo/config.toml file.

New awailable commands are:
* kbuild - build kernel
* kimage - create bootable image without running
* krun - creata a bootable image and run it

### Build

Build the kernel with cargo:
```bash
cargo kbuild
```
## Run

To test the kernel using qemu emulation run the following command:
```bash
$ cargo krun
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
