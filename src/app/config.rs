use anyhow::anyhow;
use ini::{Ini, Properties};
use std::{fmt::Display, net::IpAddr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct IgdbConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl IgdbConfig {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
    pub fn from_init_section(igdb_section: Option<&Properties>) -> anyhow::Result<Self> {
        match igdb_section {
            Some(section) => {
                let client_id: String = section
                    .get("client_id")
                    .ok_or_else(|| anyhow!("client_id is required"))?
                    .into();
                let client_secret: String = section
                    .get("client_secret")
                    .ok_or_else(|| anyhow!("client_secret is required"))?
                    .into();
                Ok(IgdbConfig {
                    client_id,
                    client_secret,
                })
            }
            None => Err(anyhow!("igdb settings are required")),
        }
    }
}

#[derive(Default)]
pub struct ServerConfigBuilder {
    pub port: Option<u16>,
    pub address: Option<String>,
}

impl ServerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_port(mut self, port: Option<String>) -> anyhow::Result<Self> {
        if let Some(port) = port {
            let port = port.parse::<u16>()?;
            self.port = Some(port);
        }
        Ok(self)
    }
    pub fn set_address(mut self, address: Option<String>) -> anyhow::Result<Self> {
        if let Some(add) = &address {
            let _test_address: IpAddr = add.parse()?;
            self.address = address;
        }
        Ok(self)
    }
    pub fn build(self) -> ServerConfig {
        let mut server_config = ServerConfig::default();
        if let Some(port) = self.port {
            server_config.port = port;
        }
        if let Some(address) = self.address {
            server_config.address = address;
        }
        server_config
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub port: u16,
    pub address: String,
}
impl ServerConfig {
    pub fn from_init_section(server_section: Option<&Properties>) -> anyhow::Result<Self> {
        let config_builder = ServerConfigBuilder::new();
        match server_section {
            Some(server_section) => {
                let port = server_section.get("port").map(|x| x.to_string());
                let address = server_section.get("address").map(|x| x.to_string());
                Ok(config_builder.set_port(port)?.set_address(address)?.build())
            }
            None => Ok(config_builder.build()),
        }
    }
}
// this form will be used with bind in axum
impl Display for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            address: String::from("127.0.0.1"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub igdb_config: IgdbConfig,
    pub server_config: ServerConfig,
}

impl AppConfig {
    pub fn from_init_file(init_file_path: &str) -> anyhow::Result<Self> {
        let config_file = Ini::load_from_file(init_file_path)?;
        let igdb_config = IgdbConfig::from_init_section(config_file.section(Some("Igdb")))?;
        let server_config = ServerConfig::from_init_section(config_file.section(Some("Server")))?;
        Ok(Self {
            igdb_config,
            server_config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_well_formed_config_file() {
        let app_config = match AppConfig::from_init_file("tests/config_file/pobsd-server.conf") {
            Ok(app_config) => app_config,
            Err(_) => AppConfig::default(),
        };
        assert_ne!(app_config, AppConfig::default());
        let server_config = app_config.server_config;
        let server_config_test = ServerConfig {
            port: 5000,
            address: String::from("0.0.0.0"),
        };
        assert_eq!(server_config, server_config_test);
        let igdb_config = app_config.igdb_config;
        let igdb_config_test = IgdbConfig {
            client_id: String::from("super_client_id_123456789"),
            client_secret: String::from("super_client_secret_987654321"),
        };
        assert_eq!(igdb_config, igdb_config_test);
    }
    #[test]
    fn load_not_well_formed_config_file_with_invalid_port() {
        let app_config =
            AppConfig::from_init_file("tests/config_file/pobsd-server-port-is-string.conf");
        assert!(app_config.is_err());
    }
    #[test]
    fn load_not_well_formed_config_file_with_invalid_address() {
        let app_config =
            AppConfig::from_init_file("tests/config_file/pobsd-server-address-is-incorrect.conf");
        assert!(app_config.is_err());
    }
    #[test]
    fn load_no_existing_file() {
        let app_config = AppConfig::from_init_file("tests/config_file/doesnotexist.conf");
        assert!(app_config.is_err());
    }
    #[test]
    fn load_config_file_with_missing_client_id() {
        let app_config =
            AppConfig::from_init_file("tests/config_file/pobsd-server-missing-client_id.conf");
        assert!(app_config.is_err());
    }
    #[test]
    fn load_config_file_with_missing_client_secret() {
        let app_config =
            AppConfig::from_init_file("tests/config_file/pobsd-server-missing-client_secret.conf");
        assert!(app_config.is_err());
    }
}
