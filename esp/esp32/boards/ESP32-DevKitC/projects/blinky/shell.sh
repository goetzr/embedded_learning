#! /bin/zsh

IMAGE=espressif/idf-rust:esp32_latest

docker run  \
    --rm    \
    --interactive \
    --tty   \
    --mount type=volume,source=esp32_cargo_cache,target=/home/esp/.cargo \
    --mount type=bind,source=$PWD,target=/project \
    $IMAGE \
    /bin/bash
