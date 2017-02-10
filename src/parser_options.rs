#[derive(RustcDecodable)]
pub struct ParserOptions {
    pub default: ParsingOptions,
}

#[derive(RustcDecodable)]
pub struct ParsingOptions {
    pub regex_server_value: String,
    pub regex_wrong_database: String,
}