#!/bin/bash

set -e

cargo run --bin test_move_generation -- testcases/pawns.json
cargo run --bin test_move_generation -- testcases/standard.json
cargo run --bin test_move_generation -- testcases/castling.json
cargo run --bin test_move_generation -- testcases/famous.json
cargo run --bin test_move_generation -- testcases/promotions.json
cargo run --bin test_move_generation -- testcases/taxing.json 
