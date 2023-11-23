#!/bin/sh

set -eu

bindgen --use-core  ./wrapper.h -o src/fbsd15-amd64.rs