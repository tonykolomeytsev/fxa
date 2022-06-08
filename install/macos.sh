#!/bin/bash

cleanup() {
  rm -rf fxa-x86_64-apple-darwin.tar.gz
  rm -rf dist
}

trap 'cleanup' EXIT

echo "Downloading... "
sudo curl -sSLJO https://github.com/tonykolomeytsev/fxa/releases/latest/download/fxa-x86_64-apple-darwin.tar.gz
echo "Unpacking... "
tar -xzf fxa-x86_64-apple-darwin.tar.gz
sudo xattr -d com.apple.quarantine ./dist/fxa
sudo mv ./dist/fxa /usr/local/bin/fxa
echo "Installed"
