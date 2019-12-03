use crate::environment::Environment;
use std::fmt::{Debug, Error, Formatter};
use std::str::FromStr;
use target_lexicon::{Aarch64Architecture, Architecture, ArmArchitecture, OperatingSystem, Triple};

pub fn get_cpu_architecture() -> Architecture {
    let arch = Architecture::host();
    arch
}

pub fn get_triple_for_arch(arch: &Architecture) -> Triple {
    let arch = arch.clone();
    let mut triple = Triple::default();
    triple.architecture = arch.clone();

    // Operating system
    triple.operating_system = if arch == Architecture::Wasm32 {
        OperatingSystem::Wasi
    } else {
        OperatingSystem::Linux
    };

    // Environment
    triple.environment = if arch == Architecture::Wasm32 {
        target_lexicon::Environment::Unknown
    } else if let Architecture::Arm(_) = arch {
        target_lexicon::Environment::Musleabihf
    } else {
        target_lexicon::Environment::Musl
    };

    triple
}
