#!/bin/bash

set -euxo pipefail

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32imac eclic-mode-hack.S -o bin/eclic-mode-hack.o
ar crs bin/gd32vf103xx-hal.a bin/eclic-mode-hack.o

rm bin/eclic-mode-hack.o
