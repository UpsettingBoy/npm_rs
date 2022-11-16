//! This crate provides an abstraction over [`Command`] to use `npm`
//! in a simple and easy package with fluent API.
//!
//! `npm_rs` exposes [`NpmEnv`] to configure the `npm` execution enviroment and
//! [`Npm`] to use said enviroment to execute `npm` commands.
//!
//! # Examples
//! ## Manual `NODE_ENV` setup
//! ```no_run
//! // build.rs
//!
//! use npm_rs::*;
//!
//! let exit_status = NpmEnv::default()
//!        .with_node_env(&NodeEnv::Production)
//!        .with_env("FOO", "bar")
//!        .init_env()
//!        .install(None)
//!        .run("build")
//!        .exec()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Automatic `NODE_ENV` setup
//! ```no_run
//! // build.rs
//!
//! use npm_rs::*;
//!
//! let exit_status = NpmEnv::default()
//!        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
//!        .with_env("FOO", "bar")
//!        .init_env()
//!        .install(None)
//!        .run("build")
//!        .exec()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
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

const NODE_ENV: &str = "NODE_ENV";

const NPM: &str = "npm";
const NPM_INIT: &str = "init";
const NPM_INSTALL: &str = "install";
const NPM_UNINSTALL: &str = "uninstall";
const NPM_UPDATE: &str = "update";
const NPM_RUN: &str = "run";

/// This enum is used to determine the desired `NODE_ENV` variable value. Its value by [`Default`] is [`NodeEnv::Development`]
///
/// Can be retrieved from Cargo env var `PROFILE` using [`NodeEnv::from_cargo()`](NodeEnv::from_cargo) or created manually.
pub enum NodeEnv {
    Development,
    Production,
    Custom(String),
}

/// This struct is used to create the enviroment in which npm will execute commands.
/// [`NpmEnv`] uses [`Command`] so it takes all the env variables in your system.
///
/// After the environment is configured, use [`NpmEnv::init_env()`](NpmEnv::init_env) to start issuing commands to [`Npm`].
/// # Example
/// ```no_run
/// use npm_rs::*;
///
/// let npm = NpmEnv::default()
///                  .with_node_env(&NodeEnv::Production)
///                  .with_env("FOO", "bar")
///                  .init_env();
/// ```
pub struct NpmEnv(Command);

/// This struct is used to execute npm commands.
/// Can be created from [`NpmEnv`] of using [`Default`].
///
/// After queuing the desired commands, use [`Npm::exec()`] to execute them.
/// # Example
/// ```no_run
/// use npm_rs::*;
///
/// Npm::default().install(Some(&["tailwindcss"])).exec()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Npm {
    cmd: Command,
    args: Vec<String>,
}

impl Default for NodeEnv {
    fn default() -> Self {
        NodeEnv::Development
    }
}

impl NodeEnv {
    /// Creates a [`NodeEnv`] enum from the Cargo `PROFILE` enviroment variable as follows:
    /// - If `PROFILE` = `debug` then [`NodeEnv::Development`].
    /// - If `PROFILE` = `release` then [`NodeEnv::Production`].
    /// - Else [`NodeEnv::Custom(String)`] with the value of `PROFILE`.
    ///
    /// This function is roughly equivalent to `NpmEnv::with_env("NODE_ENV", PROFILE)`.
    pub fn from_cargo_profile() -> Result<Self, std::env::VarError> {
        Ok(match &std::env::var("PROFILE")?[..] {
            "debug" => Self::Development,
            "release" => Self::Production,
            x => Self::Custom(x.to_string()),
        })
    }
}

impl Default for NpmEnv {
    fn default() -> Self {
        let mut cmd = Command::new(CMD);
        cmd.arg(OPT);
        cmd.current_dir(std::env::current_dir().unwrap());

        Self(cmd)
    }
}

impl Clone for NpmEnv {
    fn clone(&self) -> Self {
        let mut cmd = Command::new(self.0.get_program());
        cmd.args(self.0.get_args());
        cmd.current_dir(self.0.get_current_dir().unwrap());

        Self(cmd)
    }
}

impl NpmEnv {
    /// Inserts or updates the `NODE_ENV` envoriment variable.
    pub fn with_node_env(self, node_env: &NodeEnv) -> Self {
        let env = match node_env {
            NodeEnv::Development => "development",
            NodeEnv::Production => "production",
            NodeEnv::Custom(c) => c,
        };

        self.with_env(NODE_ENV, env)
    }

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
    /// This method will be `NpmEnv::init_env(&self)` when [`Command`] derives [`Clone`].
    /// For now, use `features = ["nightly"]` to clone the enviroment configuration.
    pub fn init_env(self) -> Npm {
        Npm {
            cmd: self.0,
            args: Default::default(),
        }
    }
}

impl Default for Npm {
    fn default() -> Self {
        NpmEnv::default().init_env()
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

    /// Same behaviour as [npm-init -y](https://docs.npmjs.com/cli/v7/commands/npm-init#yes).
    /// Initializes a package, creating a `package.json` file with the default template.
    pub fn init(mut self) -> Self {
        self.npm_append(NPM_INIT, &["-y"]);
        self
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

    /// Same behaviour as [npm-update](https://docs.npmjs.com/cli/v7/commands/npm-update).
    /// - If `args =`[`None`]: Updates all the local dependencies (local `node_modules` folder).
    /// - If `args =`[`Some`]: Updates any package in `pkg`.
    pub fn update(mut self, pkg: Option<&[&str]>) -> Self {
        self.npm_append(NPM_UPDATE, pkg.unwrap_or_default());
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
    /// ```no_run
    /// use npm_rs::*;
    ///
    /// Npm::default().custom("audit", None).exec()?; // Equivalent to `npm audit`.
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn custom(mut self, command: &str, args: Option<&[&str]>) -> Self {
        self.npm_append(command, args.unwrap_or_default());
        self
    }

    /// Executes all the commands in the invokation order used, waiting for its completion status.
    ///
    /// # Example
    /// ```no_run
    /// use npm_rs::*;
    ///
    /// let status = Npm::default().install(None).run("build").exec()?; // Executes npm install && npm run build.
    /// assert!(status.success()); // Will `panic` if not completed successfully.
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn exec(mut self) -> Result<ExitStatus, std::io::Error> {
        self.cmd.arg(self.args.join(" && "));
        self.cmd.status()
    }
}
