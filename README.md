# Nebula Package Keeper
The package manager for Nebula OS 📦

### Goals
* Optimize for multi-core systems
* Utilize Linux sandboxing features
* Compiles statically

### Features
* Fast, granular and parallel task-based model based on [shred](https://crates.io/crates/shred)
* Fast, modern parallel algorithms like [LZ4](https://github.com/lz4/lz4), [Blake2](https://blake2.net/) and [SeaHash](https://crates.io/crates/seahash)
* Secure sandboxing of build and install scripts using [ia_sandbox](https://gitlab.com/adrian.budau/ia-sandbox)
* Dynamic package definitions based on [Deno](https://deno.land/) or [Gluon](https://github.com/gluon-lang/gluon)
