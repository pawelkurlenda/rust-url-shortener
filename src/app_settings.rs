use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings {
    //#[serde(default = "default_port")]
    pub port: u16,
    //#[serde(default = "default_max_shortened_len")]
    pub max_shortened_len: usize,
}

static APP_SETTINGS: OnceLock<AppSettings> = OnceLock::new();

pub fn resolve_settings() -> Result<(), config::ConfigError> {
    let builder = config::Config::builder()
        .add_source(config::File::with_name("AppSettings"))
        .build()?;

    // todo? support environment-specific settings
    // if let Ok(env) = std::env::var("APP__ENV") {
    //     builder =
    //         builder.add_source(File::with_name(&format!("appsettings.{env}")).required(false));
    // }

    let settings = builder.try_deserialize::<AppSettings>().unwrap();
    APP_SETTINGS.set(settings).unwrap();

    Ok(())
}
