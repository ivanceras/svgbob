#!/bin/sh

# script to publish the crates

set -ev
cd crates/svgbob && cargo publish && cd - && \
cd crates/svgbob_cli && cargo publish && cd -
