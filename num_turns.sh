#!/bin/bash

# ./target/release/wars | tail -n 1 | awk -F"," '{printf $1"\n"}'
./target/release/wars | sed -n 2p
