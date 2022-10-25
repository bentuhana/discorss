use std::env;

use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

pub struct Logger;
impl Logger {
    pub fn set_logger() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
        let file_appender = tracing_appender::rolling::daily(
            env::var("LOGS_STORE_FOLDER").unwrap_or("./logs".to_string()),
            "discorss.log",
        );
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let subscriber = tracing_subscriber::registry()
            .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
            .with(fmt::Layer::new().with_writer(std::io::stdout))
            .with(fmt::Layer::new().with_writer(non_blocking));

        tracing::subscriber::set_global_default(subscriber)
    }
}
