use std::{ffi::OsString, fmt::Display, path::Path};

use assert_cmd::{assert::Assert, Command};
use assert_fs::{fixture::FixtureError, prelude::PathChild, TempDir};
use fs_extra::dir::CopyOptions;
pub use wasm::Wasm;

mod wasm;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to create temporary directory")]
    TempDir(FixtureError),

    #[error(transparent)]
    FsError(#[from] fs_extra::error::Error),
}

/// Default
pub struct Nebula {
    pub temp_dir: TempDir,
}

impl Default for Nebula {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl Nebula {
    pub fn with_default<F: FnOnce(&Nebula)>(f: F) {
        let nebula = Nebula::default();
        f(&nebula)
    }
    pub fn new() -> Result<Nebula, Error> {
        TempDir::new()
            .map_err(Error::TempDir)
            .map(|temp_dir| Nebula { temp_dir })
    }
    pub fn new_cmd(&self, name: &str) -> Command {
        let mut this = Command::cargo_bin("soroban").unwrap_or_else(|_| Command::new("soroban"));
        this.arg(name);
        this.current_dir(&self.temp_dir);
        this
    }

    pub fn dir(&self) -> &TempDir {
        &self.temp_dir
    }

    pub fn gen_test_identity(&self) {
        self.new_cmd("config")
            .arg("identity")
            .arg("generate")
            .arg("--seed")
            .arg("0000000000000000")
            .arg("test")
            .assert()
            .success();
    }

    pub fn test_address(&self, hd_path: usize) -> String {
        self.new_cmd("config")
            .args("identity address test --hd-path".split(' '))
            .arg(format!("{hd_path}"))
            .assert()
            .stdout_as_str()
    }

    pub fn fork(&self) -> Result<Nebula, Error> {
        let this = Nebula::new()?;
        self.save(&this.temp_dir)?;
        Ok(this)
    }

    /// Save the current state of the nebula to the given directory.
    pub fn save(&self, dst: &Path) -> Result<(), Error> {
        fs_extra::dir::copy(&self.temp_dir, dst, &CopyOptions::new())?;
        Ok(())
    }
}

pub fn temp_ledger_file() -> OsString {
    TempDir::new()
        .unwrap()
        .child("ledger.json")
        .as_os_str()
        .into()
}

pub trait AssertExt {
    fn stdout_as_str(&self) -> String;
}

impl AssertExt for Assert {
    fn stdout_as_str(&self) -> String {
        String::from_utf8(self.get_output().stdout.clone())
            .expect("failed to make str")
            .trim()
            .to_owned()
    }
}
pub trait CommandExt {
    fn json_arg<A>(&mut self, j: A) -> &mut Self
    where
        A: Display;
}

impl CommandExt for Command {
    fn json_arg<A>(&mut self, j: A) -> &mut Self
    where
        A: Display,
    {
        self.arg(OsString::from(j.to_string()))
    }
}