#!/bin/bash
# Script to dump the IR and assembly of examples/hello.ma
# Shows Marietta IR, Cranelift IR, and compiled assembly

set -e

EXAMPLE="examples/hello.ma"

if [ ! -f "$EXAMPLE" ]; then
    echo "Error: $EXAMPLE not found"
    exit 1
fi

echo "Building Marietta compiler..."
cargo build --quiet --bin marietta

echo ""
echo "========================================="
echo "Running: $EXAMPLE with --dump-ir --dump-clir"
echo "========================================="
echo ""

cargo run --quiet --bin marietta -- run "$EXAMPLE" --dump-ir --dump-clir
