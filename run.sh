#!/bin/bash

function cleanup() {
    rm -f /tmp/mp-server
}

cleanup

cargo watch -s "rm -f /tmp/mp-server && cargo run --release"

if [[ $? -ne 0 ]]; then
    exit 1
fi

trap cleanup INT

cargo r --release
