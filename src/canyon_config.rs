use std::fs;
use serde::Deserialize;

const CONFIG_FILE_IDENTIFIER: &'static str = "canyon.rs";

/// Loads the data defined in the Canyon configuration file, returning
/// an [`String`] with the content of the file
pub fn load() -> String {
    fs::read_to_string(CONFIG_FILE_IDENTIFIER)
        .expect("Error opening or reading the Canyon configuration file")
}

/// Stores all the types that holds the configuration retrieved on every section on the
/// configuration file.
#[derive(Deserialize, Debug)]
pub struct CanyonConfig<'a> {
    #[serde(borrow)]
    pub server: ServerConfig<'a>
}

/// Retrieves the user defined properties on the configuration file for the
/// server section
/// ```
#[test]
fn load_server_config() {
    const CONFIG_FILE_MOCK: &'static str = r#"
        [server]
        ip = '127.0.0.1'
        port = '7878'
    "#;
    let config: CanyonConfig = toml::from_str(CONFIG_FILE_MOCK)
        .expect("A failure happened retrieving the [server] section");
    assert_eq!(config.server.ip, "127.0.0.1");
    assert_eq!(config.server.port, "7878");
}
/// ```
#[derive(Deserialize, Debug)]
pub struct ServerConfig<'a> {
    pub ip: &'a str,
    pub port: &'a str,
}
