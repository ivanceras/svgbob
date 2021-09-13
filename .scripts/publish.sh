#!/bin/sh

# script to publish the crates 
# we added a 5s sleep in between publish to give time for dependency crate to propagate to crates.io
#

set -ev
cd packages/svgbob && cargo publish && cd - && \
echo "sleeping" && sleep 20 &&\
cd packages/cli && cargo publish && cd - 
