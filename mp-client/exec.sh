#!/bin/bash

rm -f /tmp/mp-client
~/dev/rust/mp-client/target/release/mp-client $@
