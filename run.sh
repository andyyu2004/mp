#!/bin/bash

function cleanup() {
    rm -f /tmp/mpclient.sock
    rm -f /tmp/mpstream.sock
}

cleanup

cargo b
if [[ $? -ne 0 ]]; then
    exit 1
fi

trap cleanup INT

cargo r --release
