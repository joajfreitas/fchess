# fchess

![https://github.com/joajfreitas/fchess/actions/workflows/main.yaml/badge.svg](https://github.com/joajfreitas/fchess/actions/workflows/main.yaml/badge.svg)

fchess is a work in progress chess engine written in Rust.


## Implementation status


## Running

fchess provides a uci interface:
```
cargo run --bin uci
```


Alternativelly you can play directly in the terminal:

```
cargo run --bin cli
```

## Tests

```
cargo test
./scripts/run_move_generation_tests.sh
```
