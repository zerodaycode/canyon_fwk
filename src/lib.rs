use crate::canyon_config::CanyonConfig;

mod canyon_config;

/// Contains the public APIs for working with the base set of features
/// provided by The Canyon Framework
mod http;

/// The entry point of any Canyon program.
///
/// Sets up the initial environment for run the embedded http server
/// for handling the client-server connections.
pub fn fire() {
    // Retrieves the user defined config from the config-file
    let config: CanyonConfig = canyon_config::load();
    // Loads an instance of the HttpServer
    http::server::HttpServer::run(config);
}