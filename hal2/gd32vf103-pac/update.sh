#!/usr/bin/env bash

# Before running this script, install the required software:
# cargo install svd2rust --version=0.17.0
# cargo install form --version=0.7.0
# pip3 install --upgrade --user svdtools

set -x
set -e

rm -rf src
mkdir src
svd patch patches/gd32vf103.yaml
svd2rust --target riscv -i GD32VF103.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt
grep -E 'feature = "rt"|extern crate riscv_rt' src/lib.rs | tee librs-patch
grep -Ev 'feature = "rt"|extern crate riscv_rt' src/lib.rs > librs-temp && mv librs-temp src/lib.rs
