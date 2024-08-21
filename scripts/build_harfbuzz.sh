#!/bin/bash

source "$(dirname $0)/util.sh"

cd bin
download "https://github.com/mesonbuild/meson/releases/download/1.2.1/meson-1.2.1.tar.gz" "meson-1.2.1.tar.gz.sha256" "meson-1.2.1.tar.gz"
tar xzvf "meson-1.2.1.tar.gz"
cd ..

MESON="$PWD/bin/meson-1.2.1/meson.py"
PCNG=$(which pcng)

cd wasm-micro-runtime

cmake -B build \
    -DWAMR_BUILD_AOT=1 \
    -DWAMR_BUILD_REF_TYPES=1 \
    -DWAMR_CONFIGUABLE_BOUNDS_CHECKS=1 \
    -DWAMR_BUILD_SIMD=1 || exit 1
cmake --build build --parallel -- VERBOSE=0 || exit 1

cd wamr-compiler
$PCNG ./build_llvm.sh || exit 1
cmake -B build || exit 1
cmake --build build --parallel -- VERBOSE=0 || exit 1

cd ../..

export CPLUS_INCLUDE_PATH=$PWD/wasm-micro-runtime/core/iwasm/include/
export LIBRARY_PATH=$PWD/wasm-micro-runtime/build/
export LD_LIBRARY_PATH="$LIBRARY_PATH"

cd harfbuzz

"$MESON" setup --reconfigure build -Dwasm=enabled || exit 1
"$MESON" compile -C build || exit 1
