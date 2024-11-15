# AVRA-RS

*Assembler for Microchip AVR Microcontroller family (similar to Atmel).*

AVRA-RS is a Rust rewrite of [AVRA](https://github.com/hsoft/avra). I have created AVRA-RS to fix
various shortcomings of the original software. This project now became something more.

First of all, this is not a complete implementation of the original assembler. It also has some
important differences, for example, it is tested as much as possible.


## Differences between AVRA-RS and AVRA

* AVRA-RS contains full implementation of assembler opcodes and directives. It does not, however,
  support full command-line options of AVRA.
* AVRA-RS supports ATmega and ATtiny MCUs.

## Build and Install

To build the `avra-rs` you can use `cargo build` and `cargo install`, or you can install
`avra-rs` from https://crates.io

## Usage

To compile source file you need to run `avra-rs` with argument `-s` with path to the
source and optionally you can provide output path with `-o`. To provide another place in
EEPROM store you can use `-e`.

For more verbose output you can use `-v`.

Other options aren't supported.
Detail information of assembler will be added in near future.

## MSRV

`cargo msrv` puts it at 1.80.1.

## Change log

See [CHANGELOG.md](CHANGELOG.md).
