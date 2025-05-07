use std::fmt::{self, Display};

use reqwest::{Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::console_log;

use crate::config;

const DATE_FORMAT: &str = "YYYY-MM-DD";
const TIME_FORMAT: &str = "HH:mm";
const IS_TIME_12HR: bool = true;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BaseId(String);

impl Display for BaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
struct TableId(String);

impl Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ApiToken(SecretString);

impl From<String> for ApiToken {
    fn from(api_token: String) -> Self {
        Self(SecretString::from(api_token))
    }
}

#[derive(Debug, Default)]
struct ByTable<T> {
    schedule: T,
    rooms: T,
    people: T,
    tags: T,
}

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            schedule: f(self.schedule),
            rooms: f(self.rooms),
            people: f(self.people),
            tags: f(self.tags),
        }
    }
}

type Tables = ByTable<TableId>;

async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        msg: String,
        errors: serde_json::Value,
    }

    let status = resp.status();
    let url = resp.url().to_string();

    if status.is_client_error() || status.is_server_error() {
        let resp = resp.json::<ErrorResponse>().await?;

        return Err(anyhow::anyhow!(
            "Error: {} for ({}) with message ({})\n{}",
            status,
            url,
            resp.msg,
            resp.errors,
        ));
    }

    Ok(resp)
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    api_origin: String,
    api_token: ApiToken,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_origin: config::noco_origin(),
            api_token: config::noco_api_token(),
        }
    }

    fn build_request_v2(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}/api/v2{}", self.api_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    fn build_request_v3(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}/api/v3{}", self.api_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    async fn create_base(&self, title: String) -> anyhow::Result<BaseId> {
        let resp = self
            .build_request_v2(Method::POST, "/meta/bases")
            .json(&json!({
                "title": title
            }))
            .send()
            .await?;

        #[derive(Debug, Deserialize)]
        struct PostBaseResponse {
            id: String,
        }

        let base_id = check_status(resp)
            .await?
            .json::<PostBaseResponse>()
            .await?
            .id;

        console_log!("Created Noco base `{}` with ID `{}`", title, base_id);

        Ok(BaseId(base_id))
    }

    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
        #[derive(Debug)]
        struct TableRequest<'a> {
            body: serde_json::Value,
            table_id: &'a mut Option<TableId>,
        }

        let mut tables = ByTable::<Option<TableId>>::default();

        let requests = vec![
            TableRequest {
                body: json!({
                    "title": "Schedule",
                    "description": "The con schedule.",
                    "fields": [
                        {
                            "title": "Event Name",
                            "type": "SingleLineText",
                            "description": "The name of the panel, workshop, class, etc."
                        }
                    ]
                }),
                table_id: &mut tables.schedule,
            },
            TableRequest {
                body: json!({
                    "title": "Rooms",
                    "description": "The rooms at the con.",
                    "fields": [
                        {
                            "title": "Room",
                            "type": "SingleLineText",
                            "description": "The room where the event is being held."
                        }
                    ]
                }),
                table_id: &mut tables.rooms,
            },
            TableRequest {
                body: json!({
                    "title": "People",
                    "description": "People hosting events at the con.",
                    "fields": [
                        {
                            "title": "Name",
                            "type": "SingleLineText",
                            "description": "The name of the person hosting an event."
                        }
                    ]
                }),
                table_id: &mut tables.people,
            },
            TableRequest {
                body: json!({
                    "title": "Tags",
                    "description": "Tags for events.",
                    "fields": [
                        {
                            "title": "Tag",
                            "type": "SingleLineText",
                            "description": "The name of the tag."
                        }
                    ]

                }),
                table_id: &mut tables.tags,
            },
        ];

        #[derive(Debug, Deserialize)]
        struct PostTableResponse {
            id: String,
        }

        for request in requests {
            let resp = self
                .build_request_v3(
                    Method::POST,
                    &format!("/meta/bases/{0}/bases/{0}/tables", base_id),
                )
                .json(&request.body)
                .send()
                .await?;

            let table_id = check_status(resp)
                .await?
                .json::<PostTableResponse>()
                .await?
                .id;

            let table_name = request
                .body
                .as_object()
                .and_then(|obj| obj.get("title"))
                .and_then(|title| title.as_str())
                .unwrap_or("Unknown");

            console_log!("Created Noco table `{}` with ID `{}`", table_name, table_id);

            *request.table_id = Some(TableId(table_id));
        }

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn populate_tables(&self, base_id: &BaseId, table_ids: &Tables) -> anyhow::Result<()> {
        #[derive(Debug)]
        struct FieldRequest<'a> {
            table_id: &'a TableId,
            body: serde_json::Value,
        }

        let requests = vec![
            FieldRequest {
                table_id: &table_ids.schedule,
                body: json!({
                    "title": "Description",
                    "type": "LongText",
                    "description": "A description of the event.",
                    "options": {
                        "rich_text": true
                    }
                }),
            },
            FieldRequest {
                table_id: &table_ids.schedule,
                body: json!({
                    "title": "Start Time",
                    "type": "DateTime",
                    "description": "The day and time the event starts.",
                    "options": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "12hr_format": IS_TIME_12HR
                    }
                }),
            },
            FieldRequest {
                table_id: &table_ids.schedule,
                body: json!({
                    "title": "End Time",
                    "type": "DateTime",
                    "description": "The day and time the event ends.",
                    "options": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "12hr_format": IS_TIME_12HR
                    }
                }),
            },
            FieldRequest {
                table_id: &table_ids.rooms,
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events being held in this room.",
                    "options": {
                        "relation_type": "hm",
                        "linked_table_id": &table_ids.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &table_ids.people,
                body: json!({
                    "title": "Contact Info",
                    "type": "SingleLineText",
                    "description": "Contact info for this person. Attendees cannot see this."
                }),
            },
            FieldRequest {
                table_id: &table_ids.people,
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events this person is hosting.",
                    "options": {
                        "relation_type": "mm",
                        "linked_table_id": &table_ids.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &table_ids.tags,
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events with this tag.",
                    "options": {
                        "relation_type": "mm",
                        "linked_table_id": &table_ids.schedule
                    }
                }),
            },
        ];

        #[derive(Debug, Deserialize)]
        struct PostFieldResponse {
            id: String,
        }

        for request in requests {
            let resp = self
                .build_request_v3(
                    Method::POST,
                    &format!("/meta/bases/{}/tables/{}/fields", base_id, request.table_id),
                )
                .json(&request.body)
                .send()
                .await?;

            let field_id = check_status(resp)
                .await?
                .json::<PostFieldResponse>()
                .await?
                .id;

            let field_name = request
                .body
                .as_object()
                .and_then(|obj| obj.get("title"))
                .and_then(|title| title.as_str())
                .unwrap_or("Unknown");

            console_log!(
                "Created Noco field `{}` with ID `{}` on table `{}`",
                field_name,
                field_id,
                request.table_id,
            );
        }

        Ok(())
    }

    #[worker::send]
    pub async fn setup_base(&self, title: String) -> anyhow::Result<Url> {
        let base_id = self.create_base(title).await?;
        let table_ids = self.create_tables(&base_id).await?;
        self.populate_tables(&base_id, &table_ids).await?;

        let app_origin = config::noco_origin();

        Ok(Url::parse(&format!(
            "{app_origin}/dashboard/#/nc/{base_id}"
        ))?)
    }
}
