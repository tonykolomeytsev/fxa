#!/bin/bash

cleanup() {
  printf "Cleaning up... "
  rm -rf fxa-x86_64-apple-darwin.tar.gz
  rm -rf dist
  echo "Done"
}

trap 'cleanup' EXIT

sudo curl -LJO https://github.com/tonykolomeytsev/fxa/releases/latest/download/fxa-x86_64-apple-darwin.tar.gz
tar -xzvf fxa-x86_64-apple-darwin.tar.gz
sudo xattr -d com.apple.quarantine ./dist/fxa
sudo mv ./dist/fxa /usr/local/bin/fxa
