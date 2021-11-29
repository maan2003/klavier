# klavier

_Remap all the keys!_

## Running

1. Edit `DEVICE_PATH` in `src/main.rs`
1. Change `rules` in `src/main.rs` to your liking.
1. `cargo build`
1. `sudo ./target/debug/klavier`

## Rules

Rules are objects translating one input event to zero or more input events. 

Most rules are defined in `src/rules.rs` and `src/rules/*.rs`
