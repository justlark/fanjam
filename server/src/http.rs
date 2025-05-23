use std::sync::LazyLock;

static client: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub fn get_client() -> reqwest::Client {
    client.clone()
}

pub async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        return Err(anyhow::anyhow!(
            "{} for ({}) with response body:\n{}",
            status,
            url,
            resp.text()
                .await
                .unwrap_or("Could not decode response body".to_string()),
        ));
    }

    Ok(resp)
}
