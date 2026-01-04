#!/usr/bin/bash

# set -ex
# trap "exit" INT TERM
# trap "kill 0" EXIT

cargo build --release
BIN=../../target/release/demo-net

# PROCS=()
for i in 0 1 2 3
do
  RUST_LOG=info $BIN "./data/4" $i &
  # pid=$!
  # PROCS+=("$pid")
done
# jobs -pr

# for pid in $PROCS
# do
#   jobs -pr
#   wait $pid
#   jobs -pr
# done

# echo done
