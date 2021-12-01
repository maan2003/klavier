# klavier

_Remap all the keys!_

## Running

0. Use Linux and Install Rust
1. Edit `DEVICE_PATH` in `src/main.rs`
1. Change `rules` in `src/main.rs` to your liking.
1. `cargo build`
1. `sudo ./target/debug/klavier`

## Rules

Rules are objects translating one input event to zero or more input events. 

Most rules are defined in `src/rules/*.rs`

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

