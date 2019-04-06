use crate::package::manifest::Manifest;
use ion_shell::{shell::variables::Value, Capture, Shell, ShellBuilder};

pub struct ManifestEvaluator {
    shell: Shell,
    manifest: Manifest,
}

impl ManifestEvaluator {
    pub fn new(manifest: &Manifest) -> Self {
        let mut shell = ShellBuilder::new().as_binary();

        // Set all the variables
        shell.variables.set("version", &manifest.info.version);
        shell.variables.set("name", &manifest.info.name);

        ManifestEvaluator {
            shell,
            manifest: manifest.clone(),
        }
    }

    pub fn evaluate_string(&self, source: &str) -> Value {
        let pid = self
            .shell
            .fork(Capture::Both, |shell| {
                shell.execute_script(&format!("let result = \"{}\"", source));
            })
            .unwrap()
            .pid;
        let result = nix::sys::wait::waitpid(pid, None).unwrap();
    }
}
