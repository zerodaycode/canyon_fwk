use serde::Deserialize;

/// Stores all the types that holds the configuration retrieved on every section on the
/// configuration file.
#[derive(Deserialize)]
pub struct CanyonConfig<'a> {
    #[serde(borrow)]
    server: ServerConfig<'a>
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
#[derive(Deserialize)]
pub struct ServerConfig<'a> {
    ip: &'a str,
    port: &'a str,
}
