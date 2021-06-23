pub fn parse_command_line() -> Result<(Option<crate::Command>, Option<String>), crate::Error> {
    parse(std::env::args().collect())
}

pub fn parse(
    arguments: Vec<String>,
) -> Result<(Option<crate::Command>, Option<String>), crate::Error> {
    match arguments.len() {
        1 => Ok((None, None)),
        2 => Ok((Some(crate::Command::parse(&arguments[1])?), None)),
        3 => Ok((
            Some(crate::Command::parse(&arguments[1])?),
            Some(arguments[2].to_string()),
        )),
        _ => Err(crate::Error::TooManyArguments(arguments[0].to_string())),
    }
}
