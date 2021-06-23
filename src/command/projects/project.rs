use std::path::{Path, PathBuf};

pub struct Project {
    build_command: Option<super::command::Command>,
    clean_command: Option<super::command::Command>,
    path: PathBuf,
    output_path: PathBuf,
    install_path: PathBuf,
}

impl Project {
    pub fn parse(object: json::Object) -> Result<Self, Box<dyn std::error::Error>> {
        let mut object = object.to_hash()?;

        let build_command = match object.remove("build_command") {
            None => None,
            Some(object) => Some(super::command::Command::parse(object)?),
        };

        let clean_command = match object.remove("clean_command") {
            None => None,
            Some(object) => Some(super::command::Command::parse(object)?),
        };

        let path = match object.remove("path") {
            None => return Err(Box::new(crate::Error::NotFound("path".to_string()))),
            Some(object) => PathBuf::from(object.to_string()?),
        };

        let output_path = match object.remove("output_path") {
            None => return Err(Box::new(crate::Error::NotFound("output_path".to_string()))),
            Some(object) => PathBuf::from(object.to_string()?),
        };

        let install_path = match object.remove("install_path") {
            None => return Err(Box::new(crate::Error::NotFound("install_path".to_string()))),
            Some(object) => PathBuf::from(object.to_string()?),
        };

        Ok(Project::new(
            build_command,
            clean_command,
            path,
            output_path,
            install_path,
        ))
    }

    fn new(
        build_command: Option<super::command::Command>,
        clean_command: Option<super::command::Command>,
        path: PathBuf,
        output_path: PathBuf,
        install_path: PathBuf,
    ) -> Self {
        Project {
            build_command: build_command,
            clean_command: clean_command,
            path: path,
            output_path: output_path,
            install_path: install_path,
        }
    }

    pub fn build(
        &self,
        default_command: &Option<super::command::Command>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command = match &self.build_command {
            Some(command) => command,
            None => match default_command {
                Some(command) => command,
                None => return Err(Box::new(crate::Error::NoCommand("build"))),
            },
        };

        let output = command
            .create_command()
            .current_dir(self.path.clone())
            .output()?;

        if !output.status.success() {
            return Err(Box::new(crate::Error::BuildError(output.status)));
        }

        let output_path = self.path.join(self.output_path.clone());
        let target_path = Path::new(crate::config::SYSROOT_DIR).join(self.install_path.clone());

        match target_path.parent() {
            Some(parent_path) => std::fs::create_dir_all(parent_path)?,
            None => {}
        }

        std::fs::copy(output_path, target_path)?;

        Ok(())
    }

    pub fn clean(
        &self,
        default_command: &Option<super::command::Command>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let command = match &self.clean_command {
            Some(command) => command,
            None => match default_command {
                Some(command) => command,
                None => return Err(Box::new(crate::Error::NoCommand("clean"))),
            },
        };

        let output = command
            .create_command()
            .current_dir(self.path.clone())
            .output()?;

        if !output.status.success() {
            Err(Box::new(crate::Error::BuildError(output.status)))
        } else {
            Ok(())
        }
    }
}
