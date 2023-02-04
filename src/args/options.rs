use crate::commands::Command;
use std::path::{Path, PathBuf};

pub struct Options {
    command: Command,
    path: PathBuf,
    sysroot: PathBuf,
    full_sysroot: PathBuf,
    debug: bool,
}

const DEFAULT_SYSROOT: &'static str = "sysroot";

impl Options {
    pub fn command(&self) -> Command {
        self.command
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn sysroot(&self) -> &Path {
        &self.full_sysroot
    }

    #[allow(unused)]
    pub fn is_debug(&self) -> bool {
        self.debug
    }

    pub fn is_release(&self) -> bool {
        !self.debug
    }

    pub(super) fn set_command(&mut self, command: Command) {
        self.command = command;
    }

    pub(super) fn set_path(&mut self, path: PathBuf) {
        self.path = path;
        self.update_sysroot();
    }

    pub(super) fn set_sysroot(&mut self, sysroot: PathBuf) {
        self.sysroot = sysroot;
        self.update_sysroot();
    }

    pub(super) fn set_debug(&mut self) {
        self.debug = true;
    }

    pub(super) fn set_release(&mut self) {
        self.debug = false;
    }

    fn update_sysroot(&mut self) {
        if self.sysroot.is_absolute() {
            self.full_sysroot = self.sysroot.clone();
            return;
        }

        self.full_sysroot = self.path.join(&self.sysroot);
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            command: Command::default(),
            path: PathBuf::new(),
            sysroot: PathBuf::from(DEFAULT_SYSROOT),
            full_sysroot: PathBuf::from(DEFAULT_SYSROOT),
            #[cfg(debug_assertions)]
            debug: true,
            #[cfg(not(debug_assertions))]
            debug: false,
        }
    }
}
