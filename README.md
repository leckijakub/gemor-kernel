# Gemor kernel

## Introduction

**Welcome to Germor kernel repository!**

This repository is intended to be my sandbox for diving into kernel development along
with Rust programming language.

Many tutorials that I've watched, were compromising complex kernel development
with simplicity for the sake of introduction.
This will not take place here. I want to create a system with up-to-date
standards which include UEFI and 64-bit processors.


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

## Build

### Docker environment
A custom docker container is provided for Gemor kernel development. To build/run
the docker container, run script `dev-env.py` under `development/docker`
directory.

```bash
$ development/scripts/build-docker-image.sh
$ development/scripts/enter-dev-env.sh
```
