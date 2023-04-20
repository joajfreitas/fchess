#!/bin/bash

set -e

cargo run --release --bin test_movegen -- testcases/movegen/standard.yaml
cargo run --release --bin test_movegen -- testcases/movegen/promotion.yaml
cargo run --release --bin test_movegen -- testcases/movegen/enpassant.yaml
