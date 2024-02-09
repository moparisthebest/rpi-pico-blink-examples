#!/bin/bash
set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

mkdir -p /tmp/8
cd /tmp/8

cmake "$SCRIPT_DIR"

make

picotool info
picotool load blink.uf2
picotool reboot

