# LUM-MARINER
LUM/MARINER attempts to aim to be a clone of XNU/Darwin written in Rust just without the Apple stuff. A kernel and Operating System.

# What I plan for this?

I plan for LUM/MARINER to become a Unix based kernel in Rust that is almost like a clone of XNU/Darwin to some degree. I plan on making this compatible with Linux/Unix applications and making the kernel modular to allow for easier modifications.

# So, how do you even build this?

1. Git clone this repo.
2. Have rust nightly installed using rustup.
3. Run cargo install bootimage
4. In LUM, run cargo bootimage.
5. Load the image in a emulator or virtualiser with a BIOS, not UEFI as we don't have that yet. (But I plan for it eventually when this gets somewhere usable.)

# Why?
I've always found XNU and Darwin really interesting to me and I've always wanted to do stuff with it but due to the way XNU/Darwin is, you can't really do much. Therefore, LUM/MARINER sprung into existence. And well... because why not? It seemed like a fun idea.

# What does LUM stand for?
"Like Unix... Maybe?"

# What does MARINER stand for?
It's a reference to something. I may say it in the future.



Oh, and I should probably point out that I'm doing this for fun and learning, before anyone attempts to destroy me for this... I know some people will.
