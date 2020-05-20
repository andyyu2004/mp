#!/bin/bash

function cleanup() {
    rm -f /tmp/mp-server
}

cleanup

cargo b
if [[ $? -ne 0 ]]; then
    exit 1
fi

trap cleanup INT

cargo r --release
