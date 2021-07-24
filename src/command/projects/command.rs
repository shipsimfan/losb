pub struct Command {
    command: String,
    args: Vec<String>,
    env: Option<Vec<(String, String)>>,
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

        let env = match object.remove("env") {
            None => None,
            Some(object) => {
                let mut env = Vec::new();
                let vec = object.to_vec()?;
                let mut iter = vec.iter();
                while let Some(object) = iter.next() {
                    let key = object.clone().to_string()?;
                    let value = match iter.next() {
                        Some(value) => value.clone().to_string()?,
                        None => return Err(Box::new(crate::error::Error::InvalidCommand(key))),
                    };

                    env.push((key, value));
                }

                Some(env)
            }
        };

        Ok(Command::new(command, args, env))
    }

    fn new(command: String, args: Vec<String>, env: Option<Vec<(String, String)>>) -> Self {
        Command {
            command: command,
            args: args,
            env: env,
        }
    }

    pub fn create_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new(self.command.to_string());
        command.args(self.args.clone());

        match &self.env {
            Some(env) => {
                for env in env {
                    command.env(&env.0, &env.1);
                }
            }
            None => {}
        }

        command.stdout(std::process::Stdio::inherit());
        command.stderr(std::process::Stdio::inherit());
        command.stdin(std::process::Stdio::inherit());

        command
    }
}
