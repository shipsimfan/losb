use crate::commands::Command;
use std::path::{Path, PathBuf};

pub struct Options {
    command: Command,
    path: PathBuf,
}

impl Options {
    pub fn command(&self) -> Command {
        self.command
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn set_command(&mut self, command: Command) {
        self.command = command;
    }

    pub(super) fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            command: Command::default(),
            path: PathBuf::new(),
        }
    }
}
