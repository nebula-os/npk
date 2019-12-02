use std::fmt::{Debug, Error, Formatter};
use std::str::FromStr;
use target_lexicon::Architecture;

pub fn get_cpu_architecture() -> Architecture {
    let arch = Architecture::host();
    arch
}
