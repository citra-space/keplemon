#!/bin/bash
# Script to run GPU test and extract dpper output values

set -e

echo "=== Compiling with DEBUG_PRINT enabled ==="
cd /home/thebo/ngsx/keplemon
~/.cargo/bin/cargo test --features cuda test_intermediate_trace --no-run 2>&1 | tail -20

echo ""
echo "=== Running GPU test to extract dpper values ==="
echo "Note: This requires CUDA runtime to be available"
echo ""

export LD_LIBRARY_PATH=/usr/local/cuda-12.6/lib64:/usr/local/cuda-12.6/targets/x86_64-linux/lib:$LD_LIBRARY_PATH

~/.cargo/bin/cargo test --features cuda test_intermediate_trace -- --nocapture 2>&1 | grep -A20 "DPPER OUTPUT"
