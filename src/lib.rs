use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, ExitStatus},
};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_family = "windows")] {
        const CMD: &str = "cmd.exe";
        const OPT: &str = "/C";
    } else {
        const CMD: &str = "bash";
        const OPT: &str = "-c";
    }
}

const NPM: &str = "npm";
const NPM_INSTALL: &str = "install";
const NPM_RUN: &str = "run";

pub struct NpmEnvCfg(Command);

pub struct Npm {
    cmd: Command,
    args: Vec<String>,
}

impl Default for NpmEnvCfg {
    fn default() -> Self {
        let mut cmd = Command::new(CMD);
        cmd.arg(OPT);
        cmd.current_dir(std::env::current_dir().unwrap());

        Self(cmd)
    }
}

impl NpmEnvCfg {
    pub fn with_env<S>(mut self, key: S, val: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        self.0.env(key, val);
        self
    }

    pub fn with_envs<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.envs(vars);
        self
    }

    pub fn clear_envs(mut self) -> Self {
        self.0.env_clear();
        self
    }

    pub fn remove_env<K>(mut self, key: K) -> Self
    where
        K: AsRef<OsStr>,
    {
        self.0.env_remove(key);
        self
    }

    pub fn set_path<S>(mut self, path: S) -> Self
    where
        S: AsRef<Path>,
    {
        self.0.current_dir(path);
        self
    }

    pub fn configure(self) -> Npm {
        Npm {
            cmd: self.0,
            args: Default::default(),
        }
    }
}

impl Default for Npm {
    fn default() -> Self {
        NpmEnvCfg::default().configure()
    }
}

impl Npm {
    pub fn install(mut self, packages: Option<&[&str]>) -> Self {
        self.args.push(
            [NPM, NPM_INSTALL]
                .iter()
                .chain(packages.unwrap_or_default())
                .copied()
                .collect::<Vec<_>>()
                .join(" "),
        );

        self
    }

    pub fn run(mut self, runner: &str) -> Self {
        self.args.push([NPM, NPM_RUN, runner].join(" "));

        self
    }

    pub fn custom(mut self, command: &str, args: Option<&[&str]>) -> Self {
        self.args.push(
            [NPM, command]
                .iter()
                .chain(args.unwrap_or_default())
                .copied()
                .collect::<Vec<_>>()
                .join(" "),
        );

        self
    }

    pub fn exec(mut self) -> Result<ExitStatus, std::io::Error> {
        self.cmd.arg(self.args.join(" && "));
        dbg!(&self.cmd);

        self.cmd.status()
    }
}
