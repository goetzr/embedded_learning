#! /bin/zsh

docker create -v ~/projects/embedded_learning/esp/esp32/boards/ESP32-DevKitC/projects/blinky:/project --name esp32 espressif/idf-rust:esp32_latest
