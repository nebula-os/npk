# Nebula Package Keeper
The package manager 📦 for Nebula OS and beyond

### Goals
* Optimized for multi-core systems
* Utilizes Linux sandboxing features
* Compiles into a static binary

### Features
* Universal, platform-agnostic, secure runtime based on [Wasmtime](https://github.com/CraneStation/wasmtime)
* Fast, modern parallel algorithms like [LZ4](https://github.com/lz4/lz4), [Blake2](https://blake2.net/) and [SeaHash](https://crates.io/crates/seahash)
* Secure sandboxing of applications using [gaol](https://github.com/servo/gaol)
* Dynamic, secure package definitions with [TypeScript](https://github.com/swc-project/swc) or JavaScript via [Boa](https://github.com/jasonwilliams/boa)
* Internal parallelization via [shred](https://crates.io/crates/shred)
