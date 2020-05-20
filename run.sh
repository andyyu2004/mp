#!/bin/bash

function cleanup() {
    rm -f /tmp/mp-client
    rm -f /tmp/mp-client-stream
}

cleanup

cargo b
if [[ $? -ne 0 ]]; then
    exit 1
fi

trap cleanup INT

cargo r --release -- $@
