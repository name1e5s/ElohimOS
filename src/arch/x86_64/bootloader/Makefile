MODE ?= release
EFI := target/x86_64-unknown-uefi/$(MODE)/xi-bootloader.efi
OVMF := OVMF-pure-efi.fd
ESP := esp
QEMU_ARGS := -net none

ifeq (${MODE}, release)
	BUILD_ARGS += --release
endif

.PHONY: build run

build:
	cargo xbuild --target x86_64-unknown-uefi $(BUILD_ARGS)

run:
	mkdir -p $(ESP)/EFI/Boot
	cp $(EFI) $(ESP)/EFI/Boot/BootX64.efi
	qemu-system-x86_64 \
		-drive if=pflash,format=raw,file=${OVMF},readonly=on \
		-drive format=raw,file=fat:rw:${ESP} \
		$(QEMU_ARGS)
