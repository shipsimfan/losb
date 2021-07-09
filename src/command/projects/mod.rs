use std::{collections::HashMap, path::Path};

mod command;
mod configuration;
mod project;

pub struct Projects {
    default_build_command: Option<command::Command>,
    default_clean_command: Option<command::Command>,
    projects: HashMap<String, project::Project>,
    configurations: HashMap<String, configuration::Configuration>,
}

impl Projects {
    pub fn parse_from_file(filepath: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Ok(Projects::parse(&json)?)
    }

    pub fn parse(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut base_object = json::parse(json)?.to_hash()?;

        // Parse default build command
        let default_build_command = match base_object.remove("default_build") {
            None => None,
            Some(object) => Some(command::Command::parse(object)?),
        };

        // Parse default clean command
        let default_clean_command = match base_object.remove("default_clean") {
            None => None,
            Some(object) => Some(command::Command::parse(object)?),
        };

        // Parse projects
        let projects_hash = match base_object.remove("projects") {
            None => return Err(Box::new(crate::Error::NotFound("projects".to_string()))),
            Some(object) => object.to_hash()?,
        };
        let mut projects = HashMap::new();
        for (project_name, project_object) in projects_hash {
            projects.insert(project_name, project::Project::parse(project_object)?);
        }

        // Parse configurations
        let configurtion_hash = match base_object.remove("configurations") {
            None => {
                return Err(Box::new(crate::Error::NotFound(
                    "configurations".to_string(),
                )))
            }
            Some(object) => object.to_hash()?,
        };
        let mut configurations = HashMap::new();
        for (configuration_name, configuration_object) in configurtion_hash {
            configurations.insert(
                configuration_name,
                configuration::Configuration::parse(configuration_object)?,
            );
        }

        Ok(Projects {
            default_build_command: default_build_command,
            default_clean_command: default_clean_command,
            projects: projects,
            configurations: configurations,
        })
    }

    pub fn build_configuration(
        &self,
        configuration_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let configuration = match self.configurations.get(configuration_name) {
            Some(configuration) => configuration,
            None => {
                return Err(Box::new(crate::Error::NotFound(
                    configuration_name.to_string(),
                )))
            }
        };

        let sysroot_path = Path::new(crate::config::SYSROOT_DIR);
        let library_path = sysroot_path.join(crate::config::LIBRARY_DIR);
        let include_path = sysroot_path.join(crate::config::INCLUDE_DIR);

        std::fs::create_dir_all(library_path)?;
        std::fs::create_dir_all(include_path)?;

        for project_name in &configuration.0 {
            println!(
                "    \x1B[36;1mBuilding\x1B[0;1m {}\x1B[0m. . .",
                project_name
            );

            let project = match self.projects.get(project_name) {
                Some(project) => project,
                None => return Err(Box::new(crate::Error::NotFound(project_name.to_string()))),
            };

            project.build(&self.default_build_command)?;

            println!("    \x1B[32;1mFinished\x1B[0;1m {}\x1B[0m\n", project_name);
        }

        println!(
            "    \x1B[32;1mFinished\x1B[0m building {}",
            configuration_name
        );

        Ok(())
    }

    pub fn clean_configuration(
        &self,
        configuration_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let configuration = match self.configurations.get(configuration_name) {
            Some(configuration) => configuration,
            None => {
                return Err(Box::new(crate::Error::NotFound(
                    configuration_name.to_string(),
                )))
            }
        };

        for project_name in &configuration.0 {
            println!(
                "    \x1B[36;1mCleaning\x1B[0;1m {}\x1B[0m. . .",
                project_name
            );

            let project = match self.projects.get(project_name) {
                Some(project) => project,
                None => return Err(Box::new(crate::Error::NotFound(project_name.to_string()))),
            };

            project.clean(&self.default_clean_command)?;

            println!("    \x1B[32;1mFinished\x1B[0;1m {}\x1B[0m\n", project_name);
        }

        match std::fs::remove_dir_all(crate::config::SYSROOT_DIR) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => return Err(Box::new(error)),
            },
        }

        println!("    \x1B[32;1mFinished\x1B[0m {}", configuration_name);

        Ok(())
    }
}
