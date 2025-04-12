use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, env = "CHROMA_CLIENT_TYPE", default_value = "ephemeral")]
    #[arg(value_enum)]
    pub client_type: ClientType,

    #[arg(long, env = "CHROMA_DATA_DIR")]
    pub data_dir: Option<PathBuf>,

    #[arg(long, env = "CHROMA_HOST")]
    pub host: Option<String>,

    #[arg(long, env = "CHROMA_PORT")]
    pub port: Option<u16>,

    #[arg(long, env = "CHROMA_CUSTOM_AUTH_CREDENTIALS")]
    pub custom_auth_credentials: Option<String>,

    #[arg(long, env = "CHROMA_TENANT")]
    pub tenant: Option<String>,

    #[arg(long, env = "CHROMA_DATABASE")]
    pub database: Option<String>,

    #[arg(long, env = "CHROMA_API_KEY")]
    pub api_key: Option<String>,

    #[arg(long, env = "CHROMA_SSL", default_value = "true")]
    pub ssl: bool,

    #[arg(long, env = "CHROMA_DOTENV_PATH", default_value = ".chroma_env")]
    pub dotenv_path: PathBuf,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ClientType {
    Http,
    Cloud,
    Persistent,
    Ephemeral,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self.client_type {
            ClientType::Http => {
                if self.host.is_none() {
                    anyhow::bail!("Host must be provided for HTTP client");
                }
            }
            ClientType::Cloud => {
                if self.tenant.is_none() {
                    anyhow::bail!("Tenant must be provided for cloud client");
                }
                if self.database.is_none() {
                    anyhow::bail!("Database must be provided for cloud client");
                }
                if self.api_key.is_none() {
                    anyhow::bail!("API key must be provided for cloud client");
                }
            }
            ClientType::Persistent => {
                if self.data_dir.is_none() {
                    anyhow::bail!("Data directory must be provided for persistent client");
                }
            }
            ClientType::Ephemeral => {}
        }
        Ok(())
    }
}

