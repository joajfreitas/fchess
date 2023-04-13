#!/bin/bash

set -e

cargo run --release --bin test_movegen -- testcases/movegen/standard.yaml
