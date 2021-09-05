#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc
readonly TARGET_HOST=pi@192.168.0.11
readonly TARGET_PATH=/home/pi/client
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/client

cargo build --release --target=${TARGET_ARCH}

echo "Package built"

rsync -P ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}

echo "Package copied to target"
