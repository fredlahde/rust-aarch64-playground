# aarch64 rust build-std=core gentoo

This my playground for getting rust to build a aarch64-android no-std shared 
object on gentoo

you'll need the NDK installed

```
# Gentoo config:
USE="clippy rustfmt rls llvm_targets_X86 llvm_targets_AArch64 llvm_targets_ARM nightly"

emerge --sync && emerge --ask --verbose --update --deep --changed-use --newuse @world

# verify
rustc --print target-list  
```

```bash
# generate aarch64 android toolchain
$NDK/build/tools/make_standalone_toolchain.py --api 30 --arch arm64 --install-dir NDK/arm64

# set path to linux-android-gcc in aarch64.json as linker
# build:
cargo build -Z build-std=core --target aarch64.json

# verify:
file target/aarch64/debug/libembed.so
NDK/arm64/bin/llvm-readelf --symbols target/aarch64/debug/libembed.so | grep foo
```

## resources
https://doc.rust-lang.org/nomicon/ffi.html
https://docs.rust-embedded.org/embedonomicon/custom-target.html
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
