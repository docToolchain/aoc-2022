#!/bin/bash
MIN=0 MAX=1 ./target/release/aoc2022 &
MIN=1 MAX=2 ./target/release/aoc2022 &
MIN=2 MAX=3 ./target/release/aoc2022 &
MIN=3 MAX=4 ./target/release/aoc2022 &

wait
