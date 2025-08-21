use worker::console_error;

use crate::{
    env,
    noco::{BaseId, Version},
};

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl From<env::Config> for Option<ConnectionConfig> {
    fn from(config: env::Config) -> Self {
        Some(ConnectionConfig {
            host: config.config_db_host?,
            port: config.config_db_port?,
            database: config.config_db_name?,
            username: config.config_db_user?,
            password: config.config_db_password?,
        })
    }
}

#[derive(Debug)]
pub struct Client {
    client: tokio_postgres::Client,
}

impl Client {
    pub async fn connect(config: &ConnectionConfig) -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::Config::new()
            .host(&config.host)
            .port(config.port)
            .dbname(&config.database)
            .user(&config.username)
            .password(&config.password)
            .connect(tokio_postgres::NoTls)
            .await?;

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(error) = connection.await {
                console_error!("Postgres connection error: {:?}", error);
            }
        });

        Ok(Client { client })
    }

    pub async fn set_base(&self, base_id: &BaseId) -> anyhow::Result<()> {
        self.client
            .execute(
                "INSERT INTO noco_bases (base_id) VALUES ($1)",
                &[&base_id.to_string()],
            )
            .await?;
        Ok(())
    }

    pub async fn get_base(&self) -> anyhow::Result<Option<BaseId>> {
        let row = self
            .client
            .query_opt(
                "SELECT base_id FROM noco_bases ORDER BY sequence DESC LIMIT 1",
                &[],
            )
            .await?;

        if let Some(row) = row {
            Ok(Some(row.get::<_, String>("base_id").into()))
        } else {
            Ok(None)
        }
    }

    pub async fn set_migration(&self, migration: &Version) -> anyhow::Result<()> {
        self.client
            .execute(
                "INSERT INTO noco_migrations (version) VALUES ($1)",
                &[&u32::from(migration)],
            )
            .await?;
        Ok(())
    }

    pub async fn get_migration(&self) -> anyhow::Result<Version> {
        let row = self
            .client
            .query_opt(
                "SELECT version FROM noco_migrations ORDER BY version DESC LIMIT 1",
                &[],
            )
            .await?;

        if let Some(row) = row {
            Ok(row.get::<_, u32>("version").into())
        } else {
            Ok(Version::INITIAL)
        }
    }
}
