#! /bin/zsh

docker start esp32
docker exec --workdir /project esp32 cargo build
