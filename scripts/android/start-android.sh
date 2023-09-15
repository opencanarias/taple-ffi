#!/bin/bash

# Compilation mode: release/debug
mode="release"

# Architectures to compile. Comment on those you do not want to compile
declare -A architectures
architectures[aarch64]="arm64-v8a"
#architectures[x86_64]="x86_64"
#architectures[i686]="x86"

lib_name="libtaple_ffi.so"
root_dir=$(pwd)
final_dir=$root_dir/target/android
taple_dir=$root_dir/taple
bindgen_dir=$root_dir/uniffi-bindgen
udl_path=$taple_dir/src/taple_sdk.udl

echo "Compiling ..."
cd $taple_dir
for key in "${!architectures[@]}"
do
    rust_target="$key-linux-android"
    android_target="${architectures[$key]}"
    lib_path="target/$rust_target/$mode/$lib_name"
    lib_final_path="$final_dir/libs/$android_target"

    echo "Compiling architecture: $rust_target/$android_target"
    cross build --features android --target "$rust_target" --"$mode"

    echo "Copying $lib_name for $android_target to $lib_final_path"
    mkdir -p $lib_final_path
    cp $lib_path $lib_final_path
done

echo "Generating Kotling bindings"
cd $bindgen_dir
cargo run --bin uniffi-bindgen generate $udl_path --out-dir $final_dir --language kotlin


