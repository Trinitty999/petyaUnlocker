#!/bin/bash

echo -e "\e[1;36mChecking files..."

if [[ ! -d "esp" ]]
then

    mkdir -p esp/efi/boot
    
fi

echo -e "\e[36mCompiling..."

cargo build --target x86_64-unknown-uefi 

echo -e "\e[32mDone"

echo -e "\e[36mCopying files..."

cp target/x86_64-unknown-uefi/debug/petyaunlocker.efi esp/efi/boot/bootx64.efi

echo -e "\e[32mDone"

echo -e "\e[36mInitializing virtual machine"

qemu-system-x86_64 -enable-kvm \
 -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
 -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
 -drive format=raw,file=fat:rw:esp

echo -e "\e[32mDone"