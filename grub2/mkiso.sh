#!/bin/sh

cp ../src/kernel/anzu.elf isofiles/boot/
grub-mkrescue -o anzu.iso isofiles/

