#!/bin/sh

set -e

svd2rust -i EFM32GG11B820F2048GL192.svd --target cortex-m
rm -rf src
form -i lib.rs -o src
rm lib.rs
cargo fmt
