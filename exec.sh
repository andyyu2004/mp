#!/bin/bash

# dev script

function cleanup() {
    rm -f /tmp/mp-server
}

cleanup

/home/andyyu2004/dev/rust/mp-server/target/release/mp-server
