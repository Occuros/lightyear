use std::time::Duration;

use tracing::Level;

use crate::TickConfig;

#[derive(Clone)]
pub struct SharedConfig {
    pub enable_replication: bool,
    /// how often does the client send updates to the server?
    pub client_send_interval: Duration,
    /// how often does the server send updates to the client?
    pub server_send_interval: Duration,
    pub tick: TickConfig,
    pub log: LogConfig,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            enable_replication: false,
            // 0 means that we send updates every frame
            client_send_interval: Duration::from_millis(0),
            server_send_interval: Duration::from_millis(0),
            tick: TickConfig::new(Duration::from_millis(16)),
            log: LogConfig::default(),
        }
    }
}

#[derive(Clone)]
pub struct LogConfig {
    /// Filters logs using the [`EnvFilter`] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            filter: "wgpu=error,wgpu_hal=error,naga=warn,bevy_app=info".to_string(),
            level: Level::INFO,
        }
    }
}
