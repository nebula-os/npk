use std::fmt::{Debug, Error, Formatter};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub enum CpuArch {
    x86,
    x86_64,
    AArch64,
    ARMv6,
    ARMv7,
}

pub fn get_cpu_architecture() -> CpuArch {
    let mut arch = CpuArch::x86;
    if cfg!(target_arch = "x86_64") {
        arch = CpuArch::x86_64;
    } else if cfg!(target_arch = "aarch64") {
        arch = CpuArch::AArch64;
    } else if cfg!(target_arch = "arm") {
        arch = CpuArch::ARMv6;
    } else if cfg!(target_arch = "armv7") {
        arch = CpuArch::ARMv7;
    }

    arch
}

impl CpuArch {
    pub fn get_default_triple_name(&self) -> String {
        match self {
            x86 => "i686-unknown-linux-musl",
            x86_64 => "x86_64-unknown-linux-musl",
            AArch64 => "aarch64-unknown-linux-musl",
            ARMv6 => "arm-unknown-linux-musleabihf",
            ARMv7 => "armv7-unknown-linux-musleabihf",
        }
        .to_owned()
    }
}

impl FromStr for CpuArch {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let owned = string.to_owned();
        let owned = owned.trim();
        let string_arch = owned.to_lowercase();

        if string_arch == "x86" {
            Ok(CpuArch::x86)
        } else if string_arch == "x86_64" || string_arch == "amd64" || string_arch == "x64" {
            Ok(CpuArch::x86_64)
        } else if string_arch == "aarch64" || string_arch == "armv8" {
            Ok(CpuArch::AArch64)
        } else if string_arch == "armv7" || string_arch == "armhf" {
            Ok(CpuArch::ARMv7)
        } else if string_arch == "armv6" || string_arch == "arm" {
            Ok(CpuArch::ARMv6)
        } else {
            Err(format!("\"{}\" is not a valid architecture", string))
        }
    }
}
