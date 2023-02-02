use crate::commands::Command;

pub struct Options {
    command: Command,
}

impl Options {
    pub fn command(&self) -> Command {
        self.command
    }

    pub(super) fn set_command(&mut self, command: Command) {
        self.command = command;
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            command: Command::default(),
        }
    }
}
