#!/bin/bash

echo='echo -e \n '

echo "---- Make sure that you're running this script from the project root ----"

$echo "---- Performing version bump (leave empty to skip) ----"
read -p "From version: " old_version
read -p "To version: " new_version
sed -i "0,/version = \"$old_version\"/{s/$old_version/$new_version/}" ./mapper_service/Cargo.toml
sed -i "0,/version = \"$old_version\"/{s/$old_version/$new_version/}" ./ui/src-tauri/Cargo.toml
sed -i "0,/\"version\": \"$old_version\"/{s/$old_version/$new_version/}" ./ui/src-tauri/tauri.conf.json

$echo "---- Cleaning previous build ----"
rm -rf release_build
mkdir release_build
mkdir ./release_build/mapper_service

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
    SHOULD_ATTACH_MANIFEST=true cargo build --release --target $target

    # Windows
    cp ./target/$target/release/mapper_service.exe ../release_build/mapper_service/mapper_service-$target.exe
    # Linux / Mac
    cp ./target/$target/release/mapper_service ../release_build/mapper_service/mapper_service-$target
done

$echo "---- Cleaning UI----"
cd ../ui
rm -rf node_modules
npm i
cd ./src-tauri
rm -rf target
cargo clean
cp ../../LICENSE .
cd ..

$echo "---- Compiling UI ----"
npm run tauri build
find ./src-tauri/target/release/bundle -type f -exec cp {} ../release_build/ \;

cd ..
cp ./LICENSE ./release_build/

$echo "---- Done (see release_build) ----"