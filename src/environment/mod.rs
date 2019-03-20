use crate::environment::arch::{get_cpu_architecture, CpuArch};

pub mod arch;

#[derive(Debug)]
pub struct Environment {
    pub arch: CpuArch,
}

impl Environment {
    pub fn current() -> Self {
        let arch = get_cpu_architecture();

        Environment { arch }
    }
}
