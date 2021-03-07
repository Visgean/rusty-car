#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace


readonly TARGET_HOST=pi
readonly TARGET_PATH=/home/pi/rusty-car
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf
#readonly SOURCE_PATH=./target/release/rusty-car
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/rusty-car


#cargo build --release
#cargo build --release --target=${TARGET_ARCH}
#rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
#rsync -azP ./vendor-code ${TARGET_HOST}:/home/pi/


rm -fr target/
rsync -azP ./ ${TARGET_HOST}:/home/pi/p/rusty-car
#ssh -t ${TARGET_HOST} "cd /home/pi/p/rusty-car ; cargo run"


# run the code
#ssh -t ${TARGET_HOST} ${TARGET_PATH}


#ssh -t ${TARGET_HOST} sudo systemctl restart rusty-car.service