#!/bin/bash

PKG_NAME="$OSTYPE"
if [[ "$PKG_NAME" == "linux-gnu"* ]]; then
  PKG_NAME="fxa-x86_64-unknown-linux-musl.tar.gz"
elif [[ "$PKG_NAME" == "darwin"* ]]; then
  PKG_NAME="fxa-x86_64-apple-darwin.tar.gz"
else
  echo "The installer is not designed for this OS version, follow the installation instructions from https://github.com/tonykolomeytsev/bagimages"
  exit 1
fi

cleanup() {
  printf "Cleaning up... "
  rm -rf "$PKG_NAME"
  rm -rf dist
  echo "Done"
}

trap 'cleanup' EXIT

echo "Downloading... "
sudo curl -LJO "https://github.com/tonykolomeytsev/fxa/releases/latest/download/$PKG_NAME"
echo "Unpacking... "
tar -xzf "$PKG_NAME"
if [[ "$OSTYPE" == "darwin"* ]]; then
  sudo xattr -d com.apple.quarantine ./dist/bagimages
fi
sudo mv ./dist/bagimages /usr/local/bin/bagimages
echo "Installed"