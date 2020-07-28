#!/bin/bash

echo "Building..."
cargo build --release

echo "Running..."
./target/release/convolution -t 1 > results.txt
./target/release/convolution -t 2 >> results.txt
./target/release/convolution -t 3 >> results.txt
./target/release/convolution -t 4 >> results.txt

echo "Done. See results.txt"

 
