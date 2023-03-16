use secrecy::Secret;
use serde_aux::prelude::deserialize_number_from_string;

// general env variable struct
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub firebase: Firebase,
    pub logstach: Logstach,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Firebase {
    pub project_id: Secret<String>,
    pub credential: Secret<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct Logstach {
    pub host: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specific values, override value if they were set before.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    // E.g. `APP_DATABASE__DATABASE_NAME=postgres would set `Settings.database.database_name`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

#[derive(PartialEq, Debug)]
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
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_as_str() {
        assert_eq!(Environment::Local.as_str(), "local");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn test_try_from_string() {
        let env = Environment::try_from(String::from("local")).unwrap();
        assert_eq!(env, Environment::Local);

        let env = Environment::try_from(String::from("production")).unwrap();
        assert_eq!(env, Environment::Production);

        let err = Environment::try_from(String::from("invalid")).unwrap_err();
        assert_eq!(
            err,
            "invalid is not a supported environment. Use either `local` or `production`."
        );
    }
}
