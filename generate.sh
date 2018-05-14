#!/usr/bin/env nix-shell
#!nix-shell -i bash

set -e

svd2rust -i EFM32GG11B820F2048GL192.svd
cat lib.rs | form -o src
rm lib.rs
rustfmt build.rs src/lib.rs
