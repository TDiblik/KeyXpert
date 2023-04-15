#!/bin/bash

echo='echo -e \n '

$echo "---- Cleaning previous build ----"
rm -rf release_build
mkdir release_build

$echo "---- Installing build dependencies ----"
declare -a supported_targets=(
    "x86_64-pc-windows-msvc"
    "x86_64-unknown-linux-gnu"
    "x86_64-apple-darwin"
)
for target in "${supported_targets[@]}"
do
    rustup target add $target
done

$echo "---- Compiling mapper service ----"
cd ./mapper_service
for target in "${supported_targets[@]}"
do
    rm -rf target
    cargo clean
    cargo build --release --target $target
    cp ./target/$target/release/mapper_service* ../release_build/
done
cd ..
rm ./release_build/mapper_service.d

$echo "---- Cleaning UI----"
cd ./ui
rm -rf node_modules
npm i
cd ./src-tauri
rm -rf target
cargo clean
cd ..

$echo "---- Compiling UI ----"
npm run tauri build -- -b deb appimage msi app
# TODO: Copy msi/deb/appimage/msi/app into release_build directory

cd ..
cp ./LICENSE ./release_build/

$echo "---- Done (see release_build) ----"