pub fn get_language_from_name(name: &str) -> Result<model::Language, &'static str> {
    match name {
        "java" => Ok(model::Language::Java),
        _ => Err("Invalid language provided"),
    }
}
