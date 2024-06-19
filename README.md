# LUM-MARINER
LUM/MARINER attempts to aim to be a clone of XNU/Darwin written in Rust just without the Apple stuff. A kernel and Operating System.

Credits to [phil-opp](https://github.com/phil-opp) for the base code of this kernel.

## What I plan for this?

I plan for LUM/MARINER to become a Unix based kernel in Rust that is almost like a clone of XNU/Darwin to some degree. I plan on making this compatible with Linux/Unix applications and making the kernel modular to allow for easier modifications.

## So, how do you even build this?

### Mac:
1. Git clone this repo.
2. Install brew from [brew.sh](https://brew.sh).
3. Run 'brew install rustup'.
4. Run 'rustup toolchain install nightly'.
5. Run 'cargo install bootimage'.
6. In LUM-MARINER/LUM, run 'cargo bootimage && qemu-system-x86_64 --drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.

### Linux (Currently only tested on Ubuntu, follow at your own risk if on other distros):
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
12. In LUM-MARINER/LUM, run 'cargo bootimage && qemu-system-x86_64 --drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.

### Windows:
1. Git clone this repo.
2. Install rustup from [rustup.rs](https://rustup.rs).
3. Hit 2 to customise during install.
4. Hit enter.
5. Type nightly and hit enter.
6. Type complete and hit enter.
7. Type y for PATH var modification and hit enter.
8. Hit enter again.
9. Run 'cargo install bootimage'
10. In LUM-MARINER/LUM and in CMD (must be CMD as Powershell doesn't like this command), run 'cargo bootimage && "C:\Program Files\qemu\qemu-system-x86_64.exe" --drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-LUM.bin'. This currently only works on BIOS QEMU, not UEFI QEMU.

## Why?
I've always found XNU and Darwin really interesting to me and I've always wanted to do stuff with it but due to the way XNU/Darwin is, you can't really do much. Therefore, LUM/MARINER sprung into existence. And well... because why not? It seemed like a fun idea.

## What does LUM stand for?
"Like Unix... Maybe?"

## What does MARINER stand for?
It's a reference to something. I may say it in the future.

## Star History
<a href="https://star-history.com/#AFellowSpeedrunner/LUM-MARINER&Timeline">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=AFellowSpeedrunner/LUM-MARINER&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=AFellowSpeedrunner/LUM-MARINER&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=AFellowSpeedrunner/LUM-MARINER&type=Date" />
 </picture>
</a>


Oh, and I should probably point out that I'm doing this for fun and learning, before anyone attempts to destroy me for this... I know some people will.
