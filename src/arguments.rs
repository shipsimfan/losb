pub fn parse_command_line() -> Result<Option<crate::Command>, crate::Error> {
    parse(std::env::args().collect())
}

pub fn parse(arguments: Vec<String>) -> Result<Option<crate::Command>, crate::Error> {
    match arguments.len() {
        1 => Ok(None),
        2 => Ok(Some(crate::Command::parse(&arguments[1])?)),
        _ => Err(crate::Error::TooManyArguments(arguments[0].to_string())),
    }
}
