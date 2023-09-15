#!/bin/bash

# Compilation mode: release/debug
mode="release"

# Architectures to compile. Comment on those you do not want to compile
declare -A architectures
architectures[aarch64]="arm64-v8a"
#architectures[x86_64]="x86_64"
#architectures[i686]="x86"

echo "Compiling ..."
cd taple
for key in "${!architectures[@]}"
do
    rust_target="$key-linux-android"
    android_target="${architectures[$key]}"
    lib_name="libtaple_ffi.so"
    lib_compiled_path="target/$rust_target/$mode/$lib_name"
    lib_end_path="../target/android/$android_target"

    echo "Compiling architecture: $rust_target"
    cross build --features android --locked --target "$rust_target" --"$mode"

    echo "Copying lib to $lib_end_path"
    mkdir -p $lib_end_path
    cp $lib_compiled_path $lib_end_path
done

echo "Generating Kotling bindings"
cd ../uniffi-bindgen
cargo run --bin uniffi-bindgen generate ../taple/src/taple_sdk.udl --out-dir ../taple/target/ --language kotlin

: << com


echo "Copiando el .kt en el proyecto Android"
cd ../taple
cp target/uniffi/taple_sdk/taple_sdk.kt ../android
com

