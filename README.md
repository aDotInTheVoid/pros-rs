# pros-rs

[![Build Status](https://travis-ci.com/aDotInTheVoid/pros-rs.svg?branch=master)](https://travis-ci.com/aDotInTheVoid/pros-rs)

This is a **_HIGHLY_** unstable/WIP atempt to be able to run rust code on the vex v5 throught the PROS kernal/API.

It is the spirital sucessor to [this project of a similer name](https://github.com/aDotInTheVoid/pros.rs/blob/link_archieve/README.md)

## Requirements
- The latest stable version of [rust](https://rustup.rs/) via rustup
- Rust target `armv7r-none-eabi` (run `rustup target add armv7r-none-eabi`)
- [PROS Requirements](https://pros.cs.purdue.edu/v5/getting-started/installation.html)
  
## Useage
In general, you should be able to build the software through runing `make all`. However, this will rarely work.
In fact, I havn't been able to get it to compile on linux. If you can, let me know.

## Project Structure
- `.cargo`: cargo config
- `c_src`: The equavent of `src` in most PROS projects. It contains a minimal wrapper that shold call rust code
- `cortex_a9_types`: A rust crate with types for the vex V5
- `firmware`: The PROS linker scripts and binary blob, because [PROS isn't free](https://github.com/purduesigbots/pros/#hey-why-cant-i-build-the-pros-3-kernel)
- `include`: PROS headers
- `src`: Rust source
  
## Contributing
Pull Requests are encouraged. By opening a pull request, you agree to licence your contribution under the 
[Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

