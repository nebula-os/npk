use crate::environment::arch::CpuArch::aarch64;
use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum CpuArch {
    x86,
    x86_64,
    aarch64,
    arm,
}

pub fn get_cpu_architecture() -> CpuArch {
    let mut arch = CpuArch::x86;
    if cfg!(target_arch = "x86_64") {
        arch = CpuArch::x86_64;
    }
    if cfg!(target_arch = "aarch64") {
        arch = CpuArch::aarch64;
    }
    if cfg!(target_arch = "arm") {
        arch = CpuArch::arm;
    }

    arch
}
