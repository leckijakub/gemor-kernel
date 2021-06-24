
CFLAGS:=$(CFLAGS) -ffreestanding -O2 -nostdlib

OBJS=\
boot.o \
kernel.o \

LINK_LIST=\
$(LIBS) \
$(OBJS) \

.PHONY: all clean install install-headers install-kernel
.SUFFIXES: .o .c .s

all: gemor.kernel

gemor.kernel: $(OBJS) linker.ld
	$(CC) -T linker.ld -o $@ $(CFLAGS) $(LINK_LIST)
	grub-file --is-x86-multiboot gemor.kernel

.c.o:
	$(CC) -MD -c $< -o $@ $(CFLAGS)

.s.o:
	$(CC) -MD -c $< -o $@ $(CFLAGS)

clean:
	rm -f gemor.kernel
	rm -f $(OBJS) *.o */*.o */*/*.o
	rm -f $(OBJS:.o=.d) *.d */*.d */*/*.d
