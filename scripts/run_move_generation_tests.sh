#!/bin/bash

set -e

cargo run --release --bin test_move_generation -- testcases/pawns.yaml
cargo run --release --bin test_move_generation -- testcases/standard.yaml
cargo run --release --bin test_move_generation -- testcases/castling.yaml
cargo run --release --bin test_move_generation -- testcases/famous.yaml
cargo run --release --bin test_move_generation -- testcases/promotions.yaml
cargo run --release --bin test_move_generation -- testcases/taxing.yaml
