#!/bin/bash

cleanup() {
  printf "Cleaning up... "
  rm -rf fxa-v0.1.0-test9-x86_64-unknown-linux-musl.tar.gz
  rm -rf dist
  echo "Done"
}

trap 'cleanup' EXIT

sudo curl -LJO https://github.com/tonykolomeytsev/fxa/releases/download/v0.1.0-test9/fxa-v0.1.0-test9-x86_64-unknown-linux-musl.tar.gz
tar -xzvf fxa-v0.1.0-test9-x86_64-unknown-linux-musl.tar.gz
sudo mv ./dist/fxa /usr/local/bin/fxa
