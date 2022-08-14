use crate::config::CanyonConfig;

mod config;

/// Contains the public APIs for working with the base set of features
/// provided by The Canyon Framework
mod http;

/// TODO def for the types mod
mod core;

/// The entry point of any Canyon program. 
///
/// Sets up the initial environment for run the embedded http server
/// for handling the client-server connections.
/// 
/// The fire() function acts 
/// as a facede, abstracting away the need to expose the internal state
/// of the configuration loader and the server.
pub fn fire() {
    // Retrieves the user defined config from the config-file
    let config_data: String = config::load();
    // Map the content on the str to the Canyon configuration struct.
    let config: CanyonConfig = toml::from_str(config_data.as_str())
        .expect("Error generating the configuration for Canyon");
    // Starts the event loop of the HttpServer
    http::server::HttpServer::run(config);

    // TODO For further discussion. What about replicate different server instances
    // on different threads? Every server w'd be preceded by a central server, that just
    // pass data to the rest, acting as a load balancer.
    // This w'd be like a multipod configuration, simulating a real cluster scenario
    // (the one that it's typically provided by the deployment platforms) but directly provided
    // in the Canyon framework.
}