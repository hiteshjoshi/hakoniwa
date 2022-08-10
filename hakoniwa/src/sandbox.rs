use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{collections::HashMap, str};

use crate::{contrib, Executor, Limits, Mount, Result, Seccomp};

lazy_static! {
    static ref SANDBOX_POLICY_HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("os_env", Box::new(contrib::handlebars::os_env_helper));
        handlebars
    };
}

#[derive(Deserialize, Default, Debug)]
pub struct SandboxPolicy {
    uid: Option<u32>,
    gid: Option<u32>,
    hostname: Option<String>,
    #[serde(default)]
    limits: Limits,
    #[serde(default)]
    mounts: Vec<Mount>,
    #[serde(default)]
    env: HashMap<String, String>,
    #[serde(default)]
    seccomp: Seccomp,
}

impl SandboxPolicy {
    pub fn from_str(data: &str) -> Result<Self> {
        let data = SANDBOX_POLICY_HANDLEBARS.render_template(data, &())?;
        let policy: Self = toml::from_str(&data)?;
        Ok(policy)
    }
}

#[derive(Default)]
pub struct Sandbox {
    policy: Option<SandboxPolicy>,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_policy(&mut self, policy: SandboxPolicy) -> &mut Self {
        self.policy = Some(policy);
        self
    }

    pub fn command<SA: AsRef<str>>(&self, prog: &str, argv: &[SA]) -> Executor {
        let mut executor = Executor::new(prog, argv);
        let policy = match &self.policy {
            Some(val) => val,
            None => return executor,
        };

        if let Some(id) = policy.uid {
            executor.uid(id);
        }
        if let Some(id) = policy.gid {
            executor.gid(id);
        }
        if let Some(hostname) = &policy.hostname {
            executor.hostname(hostname);
        }

        executor
            .limits(policy.limits.clone())
            .mounts(policy.mounts.clone());

        for (k, v) in policy.env.iter() {
            executor.setenv(k, v);
        }

        executor
    }
}