use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::{Executor, Limits, Mount, Namespaces};

#[derive(Deserialize)]
struct SandboxPolicy {
    limits: Limits,
    mounts: Vec<Mount>,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        SandboxPolicy {
            limits: Limits::default(),
            mounts: [
                ("/bin", "bin"),
                ("/lib", "lib"),
                ("/lib64", "lib64"),
                ("/usr/bin", "usr/bin"),
                ("/usr/lib", "usr/lib"),
                ("/usr/lib64", "usr/lib64"),
            ]
            .iter()
            .filter_map(|(source, target)| {
                if Path::new(&source).exists() {
                    Some(Mount {
                        source: PathBuf::from(source),
                        target: PathBuf::from(target),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        }
    }
}

#[derive(Default)]
pub struct Sandbox {
    policy: SandboxPolicy,
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            ..Default::default()
        }
    }

    pub fn command<SA: AsRef<str>>(&self, prog: &str, argv: &[SA]) -> Executor {
        let mut executor = Executor::new(prog, argv);
        executor
            .limits(self.policy.limits.clone())
            .namespaces(Namespaces::default())
            .mounts(self.policy.mounts.clone());
        executor
    }
}
