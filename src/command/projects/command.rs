pub struct Command {
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn parse(object: json::Object) -> Result<Self, Box<dyn std::error::Error>> {
        let mut object = object.to_hash()?;

        let command = match object.remove("command") {
            Some(object) => object.to_string()?,
            None => return Err(Box::new(crate::Error::NotFound("command".to_string()))),
        };

        let mut args = Vec::new();
        match object.remove("args") {
            None => {}
            Some(object) => {
                let vec = object.to_vec()?;
                for object in vec {
                    args.push(object.to_string()?);
                }
            }
        }

        Ok(Command::new(command, args))
    }

    fn new(command: String, args: Vec<String>) -> Self {
        Command {
            command: command,
            args: args,
        }
    }

    pub fn create_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new(self.command.to_string());
        command.args(self.args.clone());

        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());
        command.stdin(std::process::Stdio::inherit());

        command
    }
}
