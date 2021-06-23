pub struct Configuration(pub Vec<String>);

impl Configuration {
    pub fn parse(object: json::Object) -> Result<Self, Box<dyn std::error::Error>> {
        let configuration = object.to_vec()?;
        let mut projects = Vec::new();
        for project in configuration {
            projects.push(project.to_string()?);
        }

        Ok(Configuration(projects))
    }
}
