use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use `local` or `production`.",
                other
            )),
        }
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    // Cloud SQL Unix socket connections use a host path (e.g. "/cloudsql/project:region:instance")
    // instead of a host:port pair, so the connection string has to be built differently.
    pub fn connection_string(&self) -> SecretString {
        if self.host.starts_with('/') {
            SecretString::from(format!(
                "postgres://{}:{}@/{}?host={}",
                self.username,
                self.password.expose_secret(),
                self.database_name,
                self.host
            ))
        } else {
            SecretString::from(format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            ))
        }
    }

    pub fn connection_string_without_db(&self) -> SecretString {
        if self.host.starts_with('/') {
            SecretString::from(format!(
                "postgres://{}:{}@/?host={}",
                self.username,
                self.password.expose_secret(),
                self.host
            ))
        } else {
            SecretString::from(format!(
                "postgres://{}:{}@{}:{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port
            ))
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory!");
    let configuration_dir = base_path.join("configuration");

    // switch envs on env var
    let env: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let env_filename = format!("{}.yaml", env.as_str());

    let settings = Config::builder()
        .add_source(config::File::from(configuration_dir.join("base.yaml")))
        .add_source(config::File::from(configuration_dir.join(env_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
