use tracing_subscriber::EnvFilter;

fn main() {
    // Set up the tracing subscriber with an environment filter to control log levels
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .init();

    tracing::info!("Hello, world!");
    tracing::debug!("This is a debug log");
}
