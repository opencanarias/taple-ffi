#!/bin/bash

LIB_NAME=taple_sdk
CRATE_NAME=taple_ffi
GREEN='\033[1;32m'
NC='\033[0;0m'

architectures=(x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-ios-macabi x86_64-apple-ios-macabi)

echo -e "${GREEN}Compiling sources ..."
cd ../../UniFFI
cargo build --features ios --lib --release --target x86_64-apple-ios
cargo build --features ios --lib --release --target aarch64-apple-ios
cargo +nightly build --features ios --lib --release --target aarch64-apple-ios-sim
cargo +nightly build -Z build-std --features ios --lib --release --target aarch64-apple-ios-macabi
cargo +nightly build -Z build-std --features ios --lib --release --target x86_64-apple-ios-macabi

echo -e "${GREEN}Generating swift binding ..."
cd ../UniFFI-Bindgen
cargo run --bin uniffi-bindgen generate  ../UniFFI/src/${LIB_NAME}.udl --out-dir ../UniFFI/target/ --language swift 
sed -i '' 's/module\ ${LIB_NAME}FFI/framework\ module\ ${LIB_NAME}FFI/' ../UniFFI/target/${LIB_NAME}FFI.modulemap

echo -e "${GREEN}Copying resources..."
cd ../UniFFI/target

for ((i=0;i<5;i++))
do
    cd ${architectures[i]}/release 
    rm -rf ${LIB_NAME}FFI.framework || echo "skip removing"
    mkdir -p ${LIB_NAME}FFI.framework && cd ${LIB_NAME}FFI.framework
    mkdir Headers Modules Resources
    cp ../../../${LIB_NAME}FFI.modulemap ./Modules/module.modulemap
    cp ../../../${LIB_NAME}FFI.h ./Headers
    cp ../lib${CRATE_NAME}.a ./${LIB_NAME}FFI
    cp ../../../../../scripts/ios/resources/Info.plist ./Resources
    cd ../../../.
done


#Creacion de targets
echo -e "${GREEN}Target creation ...${NC}"
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


echo -e "${GREEN}Copying swift binding into the ios lib ...${NC}"

rm -rf ../ios || echo "skip removing"

mkdir ../ios
mkdir ../ios/${LIB_NAME}
mkdir ../ios/${LIB_NAME}/Sources
mkdir ../ios/${LIB_NAME}/Sources/${LIB_NAME}

cp ../scripts/ios/resources/Package.swift ../ios/${LIB_NAME}
cp -r ../scripts/ios/resources/Tests ../ios/${LIB_NAME}
cp -r target/${LIB_NAME}FFI.xcframework ../ios/${LIB_NAME}/Sources
cp target/${LIB_NAME}.swift ../ios/${LIB_NAME}/Sources/${LIB_NAME}/

echo -e "${GREEN}Finish !"