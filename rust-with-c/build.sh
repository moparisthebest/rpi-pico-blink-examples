#!/bin/bash
set -euxo pipefail

# run: rustup target add thumbv6m-none-eabi
# run: cargo install flip-link

CARGO_TARGET_DIR=/tmp/cargo-target cargo build -vv --release --target thumbv6m-none-eabi

picotool info
mv /tmp/cargo-target/thumbv6m-none-eabi/release/rblink /tmp/rp.elf
picotool load /tmp/rp.elf
picotool reboot

