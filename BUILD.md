# So, how do you even build this?

## GitHub Releases:
You can simply download the latest compile from releases.

## Mac:
1. Git clone this repo.
2. Install brew from [brew.sh](https://brew.sh).
3. Run 'brew install rustup'.
4. Run 'rustup toolchain install nightly'.
5. Run 'cargo install bootimage'.
6. In LUM-MARINER/LUM, run 'cargo bootimage --release && qemu-system-x86_64 --drive format=raw,file=target/x86_64-unknown-none/release/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.

## Linux (Currently only tested on Ubuntu, follow at your own risk if on other distros):
1. Git clone this repo.
2. Install rustup from [rustup.rs](https://rustup.rs). (If you are on Ubuntu, snap rustup is broken and will not work with this.)
3. Hit 2 to customise during install.
4. Hit enter.
5. Type nightly and hit enter.
6. Type complete and hit enter.
7. Type y for PATH var modification and hit enter.
8. Hit enter again.
9. Run '. "$HOME/.cargo/env"'.
10. Run 'cargo install bootimage'.
11. Run 'rustup component add llvm-tools-preview'.
12. In LUM-MARINER/LUM, run 'cargo bootimage --release && qemu-system-x86_64 --drive format=raw,file=target/x86_64-unknown-none/release/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.

## Windows:
1. Git clone this repo.
2. Install rustup from [rustup.rs](https://rustup.rs).
3. Hit 2 to customise during install.
4. Hit enter.
5. Type nightly and hit enter.
6. Type complete and hit enter.
7. Type y for PATH var modification and hit enter.
8. Hit enter again.
9. Run 'cargo install bootimage'
10. In LUM-MARINER/LUM and in CMD (must be CMD as Powershell doesn't like this command), run 'cargo bootimage --release && "C:\Program Files\qemu\qemu-system-x86_64.exe" --drive format=raw,file=target/x86_64-unknown-none/release/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.
