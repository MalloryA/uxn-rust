# uxn-rust

## tal

`tal` is an uxn assembler (equivalent to `uxnasm`).

### Goals

1. Produce identical ROMs for all valid `.tal` files.
1. Produce useful error messages for all invalid `.tal` files.
1. Have some fun with Rust.

"Valid `.tal` file" means any `.tal` file for which `uxnasm` produces a ROM
file. "Invalid `.tal` file" means any `.tal` file for which `uxnasm` does not
produce a ROM file.

### Copyright note

`tal/tests/roms/projects` contains source files from
[uxn](https://git.sr.ht/~rabbits/uxn) that are copyright Devine Lu Linvega and
released under the MIT license. It also contains compiled `.rom` files which
were assembled using `uxnasm`. This is for the purpose of testing this
assembler.

The remainder of this repository is also released under the MIT license.
