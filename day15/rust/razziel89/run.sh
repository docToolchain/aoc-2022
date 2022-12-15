#!/bin/bash
trap 'kill "${pids[@]}"' EXIT

pids=()
MIN=0 MAX=1 cargo run --release &
pids+=($!)
MIN=1 MAX=2 cargo run --release &
pids+=($!)
MIN=2 MAX=3 cargo run --release &
pids+=($!)
MIN=3 MAX=4 cargo run --release &
pids+=($!)

# Wait for the first one to finish, then kill all of them.
wait -n
