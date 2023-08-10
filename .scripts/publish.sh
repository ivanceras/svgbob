#!/bin/sh

# script to publish the crates 

set -ev
cd packages/svgbob && cargo publish && cd - && \
cd packages/svgbob_cli && cargo publish && cd - 
