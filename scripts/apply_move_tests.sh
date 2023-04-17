#!/bin/bash

set -e

cargo run --release --bin test_apply_move -- testcases/apply_move/pawns.yaml
cargo run --release --bin test_apply_move -- testcases/apply_move/standard.yaml
cargo run --release --bin test_apply_move -- testcases/apply_move/castling.yaml
cargo run --release --bin test_apply_move -- testcases/apply_move/famous.yaml
cargo run --release --bin test_apply_move -- testcases/apply_move/promotions.yaml
cargo run --release --bin test_apply_move -- testcases/apply_move/taxing.yaml
