pub fn get_parser_configuration_by_file_path(
    file_path: &std::path::Path,
) -> Result<parsing::ParserConfiguration, String> {
    file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .and_then(|extension| match extension {
            "java" => Some(model::Language::Java),
            _ => None,
        })
        .map(parsing::ParserConfiguration::from)
        .ok_or(format!(
            "Could not retrieve parsing configuration for file {}",
            file_path.display()
        ))
}

#[cfg(test)]
mod tests {
    use crate::parser_configuration::get_parser_configuration_by_file_path;

    #[test]
    fn if_the_file_extension_has_no_parser_available_it_returns_error() {
        let file_path = std::path::PathBuf::from("/path/without/extension");
        assert!(get_parser_configuration_by_file_path(&file_path).is_err())
    }

    #[test]
    fn if_the_file_extension_has_a_parser_available_it_returns_a_parser_configuration() {
        let file_path = std::path::PathBuf::from("/path/for/java/file/Example.java");
        assert!(get_parser_configuration_by_file_path(&file_path).is_ok())
    }
}
