#!/bin/bash

cleanup() {
  printf "Cleaning up... "
  rm -rf fxa-x86_64-unknown-linux-musl.tar.gz
  rm -rf dist
  echo "Done"
}

trap 'cleanup' EXIT

echo "Downloading... "
sudo curl -LJO https://github.com/tonykolomeytsev/fxa/releases/latest/download/fxa-x86_64-unknown-linux-musl.tar.gz
echo "Unpacking... "
tar -xzvf fxa-x86_64-unknown-linux-musl.tar.gz
sudo mv ./dist/fxa /usr/local/bin/fxa
echo "Installed"