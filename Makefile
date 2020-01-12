MODE ?= release
EFI ?= src/arch/x86_64/bootloader/target/x86_64-unknown-uefi/release/bootloader.efi
ELF ?= target/x86_64/$(MODE)/elohim
OVMF := OVMF-pure-efi.fd
ESP := esp
QEMU_ARGS := -net none -serial mon:stdio 
ROOT := `pwd`

ifeq (${MODE}, release)
	BUILD_ARGS += --release
endif

.PHONY: build build_bootloader run

all: build build_bootloader

build:
	cargo xbuild --target src/arch/x86_64/target/x86_64.json $(BUILD_ARGS)

build_bootloader:
	cd src/arch/x86_64/bootloader && make build

run:
	mkdir -p $(ESP)/EFI/Boot
	cp $(EFI) $(ESP)/EFI/Boot/BootX64.efi
	mkdir -p $(ESP)/EFI/Elohim
	cp $(ELF) $(ESP)/EFI/Elohim/Elohim.efi
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,file=${OVMF},readonly=on \
		-drive format=raw,file=fat:rw:${ESP} \
		$(QEMU_ARGS)
