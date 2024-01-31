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

if [ $# -eq 0 ]; then
  print_failure "Please provide one argument to select the target device, ex: /dev/sdb"
  exit
fi
DEVICE=$1

if [ ! -f "target/x86_64-unknown-uefi/release/untitled_os.efi" ]; then
  print_failure "missing file: target/x86_64-unknown-uefi/release/untitled_os.efi"
  print_failure "Please build release"
  exit
fi

print_title "trying to unmount device"
umount "${DEVICE}"

print_title "creating GPT boot partition" &&
sgdisk -z "${DEVICE}" && # zap anything existing
sgdisk -o "${DEVICE}" && # write a new GPT partition with protective MBR
sgdisk -n 1:0:-0 /dev/sdb # create partition 1
sgdisk -t 1:C12A7328-F81F-11D2-BA4B-00A0C93EC93B /dev/sdb # Set partition type to ESP
sgdisk -A 1:set:2 /dev/sdb # Turn legacy uefi_boot attribute on

print_title "creating FAT32 fs" &&
mkfs.fat -F32 -n UNTITLED_OS "${DEVICE}1" &&

print_title "showing result" &&
lsblk -T -o NAME,SIZE,TYPE,FSSIZE,FSTYPE,LABEL,PARTN,PARTTYPE,PARTLABEL,PARTFLAGS "${DEVICE}" &&

print_title "mounting EFI partition" &&
mount "${DEVICE}1" bootable/esp &&

print_title "creating boot directory" &&
mkdir bootable/esp/efi &&
mkdir bootable/esp/efi/uefi_boot &&

print_title "copying EFI file" &&
cp target/x86_64-unknown-uefi/release/untitled_os.efi bootable/esp/efi/uefi_boot/bootx64.efi &&

print_title "unmounting EFI partition" &&
umount bootable/esp &&

print_success &&
exit 0

print_failure "something went wrong"
exit 1
