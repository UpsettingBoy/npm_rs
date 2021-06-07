//! This crate provides an abstraction over [`Command`] with manual `npm` commands
//! in a simple and easy package with fluent API.
//!
//! `npm_rs` exposes [NpmEnv] to configure the npm execution enviroment and
//! [Npm] to use said enviroment to execute npm commands.
//!
//! # Example
//! ```
//! let exit_status = NpmEnv::default()
//!        .with_env("NODE_ENV", "production")
//!        .init()
//!        .install(None)
//!        .run("build")
//!        .exec()?;
//! ```

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
const NPM_UNINSTALL: &str = "uninstall";
const NPM_RUN: &str = "run";

/// This struct is used to create the enviroment in which npm will execute commands.
/// `NpmEnv` uses [`Command`] so it takes all the env variables in your system.
///
/// After the environment is configured, use [`NpmEnv::init()`] to start issuing commands to [`Npm`].
/// # Example
/// ```
/// let npm = NpmEnv::default()
///                  .with_env("NODE_ENV", "production")
///                  .init();
/// ```
pub struct NpmEnv(Command);

/// This struct is used to execute npm commands.
/// Can be created from [`NpmEnv`] of using [`Default`].
///
/// After queuing the desired commands, use [`Npm::exec()`] to execute them.
/// # Example
/// ```
/// Npm::default().install(&["tailwindcss"]).exec()?
/// ```
pub struct Npm {
    cmd: Command,
    args: Vec<String>,
}

impl Default for NpmEnv {
    fn default() -> Self {
        let mut cmd = Command::new(CMD);
        cmd.arg(OPT);
        cmd.current_dir(std::env::current_dir().unwrap());

        Self(cmd)
    }
}

impl NpmEnv {
    /// Inserts or updates a enviroment variable mapping.
    pub fn with_env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.env(key, val);
        self
    }

    /// Inserts or updates multiple environment variable mappings.
    pub fn with_envs<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.0.envs(vars);
        self
    }

    /// Clears the entire environment map for [`Npm`].
    pub fn clear_envs(mut self) -> Self {
        self.0.env_clear();
        self
    }

    /// Removes an enviroment variable mapping.
    pub fn remove_env<K>(mut self, key: K) -> Self
    where
        K: AsRef<OsStr>,
    {
        self.0.env_remove(key);
        self
    }

    /// Sets the working directory for [`Npm`].
    pub fn set_path<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.0.current_dir(path);
        self
    }

    /// Initilizes [`Npm`] with the configured environment.
    ///
    /// This method will be `NpmEnv::init(&self)` when [`Command`] derives [`Clone`].
    pub fn init(self) -> Npm {
        Npm {
            cmd: self.0,
            args: Default::default(),
        }
    }
}

impl Default for Npm {
    fn default() -> Self {
        NpmEnv::default().init()
    }
}

impl Npm {
    fn npm_append(&mut self, npm_cmd: &str, chain: &[&str]) {
        self.args.push(
            [NPM, npm_cmd]
                .iter()
                .chain(chain)
                .copied()
                .collect::<Vec<_>>()
                .join(" "),
        );
    }

    /// Same behaviour as [npm-install](https://docs.npmjs.com/cli/v7/commands/npm-install).
    /// - If `args =`[`None`]: Installs all the dependencies listed in `package.json` into the local `node_modules` folder.
    /// - If `args =`[`Some`]: Installs any package in `args` into the local `node_modules` folder.
    pub fn install(mut self, args: Option<&[&str]>) -> Self {
        self.npm_append(NPM_INSTALL, args.unwrap_or_default());
        self
    }

    /// Same behaviour as [npm-uninstall](https://docs.npmjs.com/cli/v7/commands/npm-uninstall).
    /// Uninstalls the given packages in `pkg`.
    pub fn uninstall(mut self, pkg: &[&str]) -> Self {
        self.npm_append(NPM_UNINSTALL, pkg);
        self
    }

    /// Same behaviour as [npm-run-script](https://docs.npmjs.com/cli/v7/commands/npm-run-script).
    /// Runs an arbitrary `command` from `package.json`'s "scripts" object.
    pub fn run(mut self, command: &str) -> Self {
        self.args.push([NPM, NPM_RUN, command].join(" "));
        self
    }

    /// Runs a custom npm command.
    ///
    /// # Arguments
    /// - `command`: command to execute.
    /// - `args`: arguments of `command`.
    ///
    /// # Example
    /// ```
    /// Npm::default().custom("audit", None).exec()?; // Equivalent to `npm audit`.
    /// ```
    pub fn custom(mut self, command: &str, args: Option<&[&str]>) -> Self {
        self.npm_append(command, args.unwrap_or_default());
        self
    }

    /// Executes all the commands in the invokation order used, waiting for its completion status.
    ///
    /// # Example
    /// ```
    /// let status = Npm::default().install(None).run("build").exec()?; // Executes npm install && npm run build.
    /// assert!(status.success()); // Will `panic` if not completed successfully.
    /// ```
    pub fn exec(mut self) -> Result<ExitStatus, std::io::Error> {
        self.cmd.arg(self.args.join(" && "));
        self.cmd.status()
    }
}
