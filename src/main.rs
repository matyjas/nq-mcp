mod nature_iq;
mod openaq;

use crate::nature_iq::NatureIq;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Nature IQ MCP server");

    // Create an instance of our Natural IQ router
    let service = NatureIq::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })
        .expect("Nature IQ router should start");

    service
        .waiting()
        .await
        .expect("Nature IQ service should be waiting");
}
