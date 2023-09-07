#!/bin/bash

LIB_NAME=test

# Definiendo las arquitecturas
architectures=(x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-ios-macabi x86_64-apple-ios-macabi)

# Compilando la librería para las distintas arquitecturas
echo "Compilando ..."
cd UniFFI
cargo build --features ios --lib --release --target x86_64-apple-ios
cargo build --features ios --lib --release --target aarch64-apple-ios
cargo +nightly build --features ios --lib --release --target aarch64-apple-ios-sim
cargo +nightly build -Z build-std --features ios --lib --release --target aarch64-apple-ios-macabi
cargo +nightly build -Z build-std --features ios --lib --release --target x86_64-apple-ios-macabi


# Generando la traducción a swift
echo "Generando traducción a swift ..."
cd ../UniFFI-Bindgen
cargo run --features=uniffi/cli --bin uniffi-bindgen generate  ../UniFFI/src/${LIB_NAME}.udl --out-dir ../UniFFI/target/ --language swift 
sed -i '' 's/module\ ${LIB_NAME}FFI/framework\ module\ ${LIB_NAME}FFI/' ../UniFFI/target/${LIB_NAME}FFI.modulemap

# Copiando librerías compiladas en el proyecto ios
echo "Copiando recursos UniFFI..."
cd ../UniFFI/target

for ((i=0;i<4;i++))
do
    cd ${architectures[i]}/release 
    rm -rf ${LIB_NAME}FFI.framework || echo "rm failed"
    mkdir -p ${LIB_NAME}FFI.framework && cd ${LIB_NAME}FFI.framework
    mkdir Headers Modules Resources
    cp ../../../../target/${LIB_NAME}FFI.modulemap ./Modules/module.modulemap
    cp ../../../../target/${LIB_NAME}FFI.h ./Headers
    cp ../lib${LIB_NAME}.a ./${LIB_NAME}FFI
    cp ../../../../../recursos/ios-res/Info.plist ./Resources
    cd ../../../.
done


#Creacion de targets
echo "Creacion de targets ..."
cd ..

lipo -create target/x86_64-apple-ios/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI \
    target/aarch64-apple-ios-sim/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI \
    -output target/aarch64-apple-ios-sim/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI

lipo -create target/x86_64-apple-ios-macabi/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI \
    target/aarch64-apple-ios-macabi/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI \
    -output target/aarch64-apple-ios-macabi/release/${LIB_NAME}FFI.framework/${LIB_NAME}FFI
	
rm -rf target/${LIB_NAME}FFI.xcframework || echo "skip removing"
	
xcodebuild -create-xcframework \
    -framework target/aarch64-apple-ios/release/${LIB_NAME}FFI.framework \
    -framework target/aarch64-apple-ios-sim/release/${LIB_NAME}FFI.framework \
    -framework target/aarch64-apple-ios-macabi/release/${LIB_NAME}FFI.framework \
    -output target/${LIB_NAME}FFI.xcframework


# Copiando contenido en el proyecto ios
echo "Copiando el .swift en el proyecto ios ..."

mkdir ../ios
mkdir ../ios/${LIB_NAME}
mkdir ../ios/${LIB_NAME}/Sources
mkdir ../ios/${LIB_NAME}/Sources/${LIB_NAME}

cp ../recursos/ios-res/Package.swift ../ios/${LIB_NAME}
cp -r ../recursos/ios-res/Tests ../ios/${LIB_NAME}
cp -r target/${LIB_NAME}FFI.xcframework ../ios/${LIB_NAME}/Sources
cp target/${LIB_NAME}.swift ../ios/${LIB_NAME}/Sources/${LIB_NAME}/

#FIN
echo "FIN :D"