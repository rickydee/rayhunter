#!/bin/env bash

set -e

mkdir build
cd build 
curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar"
curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar.sha256"
if ! sha256sum -c --quiet release.tar.sha256; then 
    echo "Download corrupted! (╯°□°)╯︵ ┻━┻"
    exit 1
fi

platform="$(uname)"
tar -xf release.tar
if [ "$platform" == "Linux" ]; then
    ./install-linux.sh
elif [ "$platform" == "Darwin" ]; then
    ./install-macos.sh
fi

cd ..
rm -rf build
