#!/bin/sh

set -eu
bindgen --use-core \
    --default-macro-constant-type signed \
    --output src/fbsd15_amd64.rs \
    ./wrapper.h
