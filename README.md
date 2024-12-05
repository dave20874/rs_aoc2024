# Advent of Code 2024 in Rust
by David Wheeler

If you don't know what Advent of Code is, it's an annual programming challenge hosted here:

https://adventofcode.com

# Build Instructions
To build and run this code, you will need to clone both this repository and the 
repo representing the daily inputs.  (Your clone of the latter will be empty but
you can insert your own inputs.)

First clone this repo into a convenient folder:
```bash
mkdir aoc2024
cd aoc2024
git clone https://github.com/dave20874/rs_aoc2024
```

Next, clone the associated data repo as it's sibling:
```
git clone https://github.com/dave20874/data_aoc2024
```

Now, assuming you have a Rust toolchain and the Cargo tool installed,
you should be able to build and run the code:
```
cd rs_aoc2024
cargo test --release
cargo run --release
```
