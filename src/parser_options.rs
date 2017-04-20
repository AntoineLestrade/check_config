#[derive(Deserialize)]
pub struct ParserOptions {
    pub default: ParsingOptions,
}

#[derive(Deserialize)]
pub struct ParsingOptions {
    pub regex_server_value: String,
    pub regex_server_inverse: bool,
    pub regex_database_value: String,
    pub regex_database_inverse: bool,
}