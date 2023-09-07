#!/bin/bash
# Definiendo las arquitecturas
architectures=(x86_64 aarch64 i686)
architectures_android=(x86_64 arm64-v8a x86)

# Compilando la librería para las distintas arquitecturas (x86-64, arm64-v8a, x86)
echo "Iniciando compilacion ..."
cd UniFFI
for ((i=0;i<3;i++))
do
    echo "Compilando librería para arquitectura ${architectures[i]}"
    cross build --features android --target "${architectures[i]}-linux-android" --release 
done

# Copiando librerías compiladas en el proyecto Android
echo "Copiando librerías en el proyecto Android"
mkdir ../android
mkdir ../android/jniLibs
for ((i=0;i<3;i++))
do
    mkdir ../android/jniLibs/"${architectures_android[i]}"
    cp target/"${architectures[i]}-linux-android"/release/libtaple_ffi.so ../android/jniLibs/"${architectures_android[i]}"/libuniffi_taple_sdk.so
done

# Generando la traducción a Kotlin
echo "Generando traducción a Kotlin"
cd ../UniFFI-Bindgen
cargo run --features=uniffi/cli --bin uniffi-bindgen generate ../UniFFI/src/taple_sdk.udl --out-dir ../UniFFI/target/ --language kotlin

# Copiando .kt en el proyecto Android
echo "Copiando el .kt en el proyecto Android"
cd ../UniFFI
cp target/uniffi/taple_sdk/taple_sdk.kt ../android
