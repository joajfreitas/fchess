#!/bin/bash

set -e

cargo run --release --bin test_apply_move -- testcases/pawns.yaml
cargo run --release --bin test_apply_move -- testcases/standard.yaml
cargo run --release --bin test_apply_move -- testcases/castling.yaml
cargo run --release --bin test_apply_move -- testcases/famous.yaml
cargo run --release --bin test_apply_move -- testcases/promotions.yaml
cargo run --release --bin test_apply_move -- testcases/taxing.yaml
