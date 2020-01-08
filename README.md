<h1 align="center">ElohimOS</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.0.0-blue.svg?cacheSeconds=2592000" />
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html" target="_blank">
    <img alt="License: GPLv3" src="https://img.shields.io/badge/License-GPLv3-yellow.svg" />
  </a>
</p>

> OS project in the 2020's

## 📋 Requirements

To build and test the ElohimOS, you need the following packages installed:

- The awesome [rust](https://www.rust-lang.org/) toolchain

- Cross compile helper: [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild)

- The processor emulator: [QEMU](https://www.qemu.org/)

## ⚙️ Building and Running

Just type

```bash
> make
```

and 

```bash
> make run
```

**Note:** `compiler-builtins` has a bug about UEFI which is fixed on Jan 6th, 2020. It will cause `__rust_probestack` not found when we build our bootloader. To build it without error, we need to build our own `cargo-xbuild` and change `compiler-builtins` version in `src/sysroot.rs` to `0.1.21`.

## 📝 License

Copyright © 2020 [Name1e5s](https://github.com/name1e5s).<br />
This project is [GPL](https://www.gnu.org/licenses/gpl-3.0.en.html) licensed.

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_