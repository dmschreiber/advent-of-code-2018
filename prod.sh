#!/bin/bash
if [ $# -ne 1 ]; then
  echo "expected puzzle number argument"
  exit -1
fi
cargo test puzzle$1_prod --release -- --nocapture