#!/bin/sh
print_title()
{
  printf "\e[34;47m%s\e[39;49m\n" "$1"
}
print_success()
{
  printf "\e[32;107msuccess\e[39;49m\n"
}
print_failure()
{
  printf "\e[31;107m%s\e[39;49m\n" "$1"
}

if [ ! -f "target/x86_64-unknown-uefi/release/untitled_os.efi" ]; then
  print_failure "missing file: target/x86_64-unknown-uefi/release/untitled_os.efi"
  print_failure "Please build release"
  exit
fi

print_title "copying OVMF files" &&
cp /usr/share/OVMF/OVMF_CODE.fd qemu &&
cp /usr/share/OVMF/OVMF_VARS.fd qemu &&

print_title "copying EFI file" &&
cp target/x86_64-unknown-uefi/release/untitled_os.efi qemu/esp/efi/boot/bootx64.efi &&

print_title "launching QEMU" &&
qemu-system-x86_64 -enable-kvm \
-drive if=pflash,format=raw,readonly=on,file=qemu/OVMF_CODE.fd \
-drive if=pflash,format=raw,readonly=on,file=qemu/OVMF_VARS.fd  \
-drive format=raw,file=fat:rw:qemu/esp &&

print_success &&
exit 0

print_failure "something went wrong"
exit 1
