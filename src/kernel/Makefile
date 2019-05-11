TARGET = anzu.elf

CC = /usr/local/i386elfgcc/bin/i386-elf-gcc
STRIP = /usr/local/i386elfgcc/bin/i386-elf-strip
LN_S = ln -s
RANLIB = ranlib
INCLUDES = -I./include -I.
FLAGS = -m32 -ffreestanding -fno-common -fno-builtin -fomit-frame-pointer -O2 -c
LD = /usr/local/i386elfgcc/bin/i386-elf-ld  -melf_i386  -Ttext=0x100000 --oformat elf32-i386 -o
KERNLIB = -l anzu
KERNLIBDIR = -L ../../target/i686-unknown-linux-gnu/debug

.S.o:
	${CC} ${INCLUDES} ${FLAGS} $<
.c.o:
	${CC} ${INCLUDES} ${FLAGS} $<

BOOT_S = boot.S
BOOT_C = kernel.c

#BOOT_OBJ=${BOOT_S:.S=.o} ${BOOT_C:.c=.o}
BOOT_OBJ=${BOOT_S:.S=.o}

all:    anzu

anzu:   ${BOOT_OBJ}
	#${LD} ${TARGET} ${BOOT_OBJ}
	${LD} ${TARGET} ${BOOT_OBJ} ${KERNLIBDIR} ${KERNLIB}
	${STRIP} ${TARGET}

${BOOT_OBJ}:    ${BOOT_SRC}

clean::
	-${RM}  -f *~ *.lo *.o
