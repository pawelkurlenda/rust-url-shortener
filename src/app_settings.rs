use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct AppSettings {
    #[validate(range(min = 0, max = 9999))]
    pub port: u16,
    #[validate(range(min = 1, max = 7))]
    pub max_shortened_len: usize,
    pub store_dir: String,
}

//static APP_SETTINGS: OnceLock<AppSettings> = OnceLock::new();

pub fn resolve_settings() -> Result<(), Box<dyn std::error::Error>> {
    let builder = config::Config::builder()
        .add_source(config::File::with_name("AppSettings"))
        .build()?;

    // todo? support environment-specific settings
    // if let Ok(env) = std::env::var("APP__ENV") {
    //     builder =
    //         builder.add_source(File::with_name(&format!("appsettings.{env}")).required(false));
    // }

    let settings = builder.try_deserialize::<AppSettings>()?;

    settings.validate()?;

    settings::init(settings);
    Ok(())
}

pub mod settings {
    use super::AppSettings;
    use std::sync::OnceLock;

    static APP_SETTINGS: OnceLock<AppSettings> = OnceLock::new();

    pub fn init(s: AppSettings) {
        APP_SETTINGS
            .set(s)
            .expect("APP_SETTINGS already initialized");
    }

    #[inline]
    pub fn get() -> &'static AppSettings {
        APP_SETTINGS.get().unwrap()
    }

    #[inline]
    pub fn max_shortened_len() -> usize {
        get().max_shortened_len
    }

    #[inline]
    pub fn port() -> u16 {
        get().port
    }
}
