use worker::{SecureTransport, Socket, postgres_tls::PassthroughTls};

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
    pub async fn connect(config: &ConnectionConfig) -> anyhow::Result<Self> {
        let socket = Socket::builder()
            .secure_transport(SecureTransport::StartTls)
            .connect(&config.host, config.port)?;

        let (client, connection) = tokio_postgres::Config::new()
            .dbname(&config.database)
            .user(&config.username)
            .password(&config.password)
            .connect_raw(socket, PassthroughTls)
            .await?;

        wasm_bindgen_futures::spawn_local(async move {
            connection.await.ok();
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

    pub async fn delete_base(&self, base_id: &BaseId) -> anyhow::Result<()> {
        self.client
            .execute(
                "DELETE FROM noco_bases WHERE base_id = $1",
                &[&base_id.to_string()],
            )
            .await?;
        Ok(())
    }

    pub async fn set_migration(&self, base_id: &BaseId, migration: &Version) -> anyhow::Result<()> {
        self.client
            .execute(
                "
                    INSERT INTO
                        noco_migrations (base, version)
                    SELECT
                        noco_bases.id,
                        $2
                    FROM
                        noco_bases
                    WHERE
                        noco_bases.base_id = $1
                ",
                &[
                    &base_id.to_string(),
                    &i32::try_from(u32::from(migration))
                        .expect("migration version integer out of range"),
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_current_migration(&self) -> anyhow::Result<Version> {
        let row = self
            .client
            .query_opt(
                "
                    SELECT
                        noco_migrations.version
                    FROM
                        noco_migrations
                    JOIN
                        noco_bases ON noco_migrations.base = noco_bases.id
                    ORDER BY
                        noco_bases.sequence DESC,
                        noco_migrations.version DESC
                    LIMIT
                        1
                ",
                &[],
            )
            .await?;

        if let Some(row) = row {
            Ok(u32::try_from(row.get::<_, i32>("version"))
                .expect("migration version integer out of range")
                .into())
        } else {
            Ok(Version::INITIAL)
        }
    }
}
