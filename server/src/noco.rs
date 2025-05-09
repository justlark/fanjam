use std::fmt::{self, Display};

use reqwest::{Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::console_log;

const DATE_FORMAT: &str = "YYYY-MM-DD";
const TIME_FORMAT: &str = "HH:mm";
const IS_TIME_12HR: bool = true;

#[derive(Debug, Clone, Copy)]
enum NocoViewType {
    Calendar,
}

impl NocoViewType {
    fn code(&self) -> u32 {
        match self {
            NocoViewType::Calendar => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct BaseId(String);

impl Display for BaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct TableId(String);

impl Display for TableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct FieldId(String);

impl Display for FieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
struct ViewId(String);

impl Display for ViewId {
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

impl ExposeSecret<str> for ApiToken {
    fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

#[derive(Debug, Default)]
struct ByTable<T> {
    schedule: T,
    rooms: T,
    people: T,
    tags: T,
    announcements: T,
    about: T,
    links: T,
    files: T,
}

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            schedule: f(self.schedule),
            rooms: f(self.rooms),
            people: f(self.people),
            tags: f(self.tags),
            announcements: f(self.announcements),
            about: f(self.about),
            links: f(self.links),
            files: f(self.files),
        }
    }
}

type Tables = ByTable<TableId>;

#[derive(Debug, Default)]
struct ByField<T> {
    start_time: T,
}

impl<T> ByField<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByField<U> {
        ByField {
            start_time: f(self.start_time),
        }
    }
}

type Fields = ByField<FieldId>;

type RefSetter<'a, T> = Box<dyn FnOnce(T) + 'a>;

fn set_ref<T>(value_ref: &mut Option<T>) -> RefSetter<T> {
    Box::new(move |id| {
        *value_ref = Some(id);
    })
}

fn set_nop<T>() -> RefSetter<'static, T> {
    Box::new(|_| {})
}

async fn check_status(resp: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        msg: String,
        errors: Option<serde_json::Value>,
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
            resp.errors.unwrap_or_default(),
        ));
    }

    Ok(resp)
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    dash_origin: Url,
    api_token: ApiToken,
}

impl Client {
    pub fn new(dash_origin: Url, api_token: ApiToken) -> Self {
        Self {
            client: reqwest::Client::new(),
            dash_origin,
            api_token,
        }
    }

    // We're building this on top of the new v3 API, but we still need to fall back to the v2 API
    // for some operations that are not yet supported in v3.

    fn build_request_v2(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}api/v2{}", self.dash_origin, path))
            .header("Xc-Token", self.api_token.0.expose_secret())
    }

    fn build_request_v3(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, format!("{}api/v3{}", self.dash_origin, path))
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
            id: BaseId,
        }

        let base_id = check_status(resp)
            .await?
            .json::<PostBaseResponse>()
            .await?
            .id;

        console_log!("Created Noco base `{}` with ID `{}`", title, base_id);

        Ok(base_id)
    }

    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
        #[derive(Debug)]
        struct TableRequest<'a> {
            body: serde_json::Value,
            table_id: &'a mut Option<TableId>,
        }

        let mut tables = ByTable::<Option<TableId>>::default();

        // NocoDB has a concept of a "display field", which is the field that appears in the UI as
        // the short representation of a table row. Setting the display field after we create the
        // table is annoying and requires additional API calls, so we implicitly set it here
        // instead. When adding fields at table creation, the first field automatically becomes the
        // display field.
        //
        // We create the rest of the fields elsewhere, because they need to be created in a
        // specific order and after all the tables have been created so that we can set up the
        // links between them while controlling the field order within each table.
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
            TableRequest {
                body: json!({
                    "title": "Announcements",
                    "description": "Announcements that are sent to attendees.",
                    "fields": [
                        {
                            "title": "Title",
                            "type": "SingleLineText",
                            "description": "The title of the announcement."
                        }
                    ]
                }),
                table_id: &mut tables.announcements,
            },
            TableRequest {
                body: json!({
                    "title": "About",
                    "description": "Information about the con.",
                    "fields": [
                        {
                            "title": "Con Name",
                            "type": "SingleLineText",
                            "description": "The name of the con."
                        }
                    ]
                }),
                table_id: &mut tables.about,
            },
            TableRequest {
                body: json!({
                    "title": "Links",
                    "description": "Links to external resources and information.",
                    "fields": [
                        {
                            "title": "Link Name",
                            "type": "SingleLineText",
                            "description": "The text of the link."
                        }
                    ]
                }),
                table_id: &mut tables.links,
            },
            TableRequest {
                body: json!({
                    "title": "Files",
                    "description": "Images, documents, etc. to show attendees.",
                    "fields": [
                        {
                            "title": "File Name",
                            "type": "SingleLineText",
                            "description": "The name of the file."
                        }
                    ]
                }),
                table_id: &mut tables.files,
            },
        ];

        #[derive(Debug, Deserialize)]
        struct PostTableResponse {
            id: TableId,
        }

        for request in requests {
            let resp = self
                .build_request_v3(Method::POST, &format!("/meta/bases/{}/tables", base_id))
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

            *request.table_id = Some(table_id);
        }

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn create_fields(&self, tables: &Tables) -> anyhow::Result<Fields> {
        struct FieldRequest<'a> {
            table_id: &'a TableId,
            field_ref: RefSetter<'a, FieldId>,
            body: serde_json::Value,
        }

        impl fmt::Debug for FieldRequest<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("FieldRequest")
                    .field("table_id", &self.table_id)
                    .field("body", &self.body)
                    .finish_non_exhaustive()
            }
        }

        let mut fields = ByField::<Option<FieldId>>::default();

        let requests = vec![
            FieldRequest {
                table_id: &tables.schedule,
                field_ref: set_nop(),
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
                table_id: &tables.schedule,
                field_ref: set_ref(&mut fields.start_time),
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
                table_id: &tables.schedule,
                field_ref: set_nop(),
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
                table_id: &tables.rooms,
                field_ref: set_nop(),
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events being held in this room.",
                    "options": {
                        "relation_type": "hm",
                        "linked_table_id": &tables.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.people,
                field_ref: set_nop(),
                body: json!({
                    "title": "Contact Info",
                    "type": "SingleLineText",
                    "description": "Contact info for this person. Attendees cannot see this."
                }),
            },
            FieldRequest {
                table_id: &tables.people,
                field_ref: set_nop(),
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events this person is hosting.",
                    "options": {
                        "relation_type": "mm",
                        "linked_table_id": &tables.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.tags,
                field_ref: set_nop(),
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The events with this tag.",
                    "options": {
                        "relation_type": "mm",
                        "linked_table_id": &tables.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.announcements,
                field_ref: set_nop(),
                body: json!({
                    "title": "Announcement",
                    "type": "LongText",
                    "description": "The announcement itself.",
                    "options": {
                        "rich_text": true
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.announcements,
                field_ref: set_nop(),
                body: json!({
                    "title": "Files",
                    "type": "Attachment",
                    "description": "Attach images or other files with the announcement."
                }),
            },
            FieldRequest {
                table_id: &tables.announcements,
                field_ref: set_nop(),
                body: json!({
                    "title": "Created",
                    "type": "CreatedTime",
                    "description": "When this announcement was first created.",
                }),
            },
            FieldRequest {
                table_id: &tables.announcements,
                field_ref: set_nop(),
                body: json!({
                    "title": "Last Edited",
                    "type": "LastModifiedTime",
                    "description": "When this announcement was last edited.",
                }),
            },
            FieldRequest {
                table_id: &tables.about,
                field_ref: set_nop(),
                body: json!({
                    "title": "Con Description",
                    "type": "LongText",
                    "description": "A brief description of the con.",
                    "options": {
                        "rich_text": false
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.about,
                field_ref: set_nop(),
                body: json!({
                    "title": "Website",
                    "type": "URL",
                    "description": "A link to the con's website.",
                    "options": {
                        "validation": true
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.links,
                field_ref: set_nop(),
                body: json!({
                    "title": "URL",
                    "type": "URL",
                    "description": "The link URL.",
                    "options": {
                        "validation": true
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.files,
                field_ref: set_nop(),
                body: json!({
                    "title": "File",
                    "type": "Attachment",
                    "description": "The image, document, etc. to upload."
                }),
            },
        ];

        #[derive(Debug, Deserialize)]
        struct PostFieldResponse {
            id: FieldId,
        }

        for request in requests {
            let resp = self
                .build_request_v3(
                    Method::POST,
                    &format!("/meta/tables/{}/fields", request.table_id),
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

            (request.field_ref)(field_id.clone());

            console_log!(
                "Created Noco field `{}` with ID `{}` on table `{}`",
                field_name,
                field_id,
                request.table_id,
            );
        }

        Ok(fields.map(|id| id.expect("expected field ID, found none")))
    }

    async fn create_views(&self, fields: &Fields, tables: &Tables) -> anyhow::Result<()> {
        #[derive(Debug, Deserialize)]
        struct PostViewResponse {
            id: ViewId,
        }

        let resp = self
            .build_request_v2(
                Method::POST,
                &format!("/meta/tables/{}/calendars", tables.schedule),
            )
            .json(&json!({
                "title": "Calendar",
                "type": NocoViewType::Calendar.code(),
                "calendar_range": [
                    {
                        // The community version of NocoDB does not currently support date ranges
                        // in calendar views. This feature exists in the enterprise version, but it
                        // only support dates, not datetimes. Once support for datetime ranges
                        // lands in the enterprise edition, we might want to see if we can enable
                        // it in on our fork.
                        "fk_from_column_id": fields.start_time,
                    }
                ]
            }))
            .send()
            .await?;

        let calendar_view_id = check_status(resp)
            .await?
            .json::<PostViewResponse>()
            .await?
            .id;

        console_log!(
            "Created Noco calendar view with ID `{}` on table `{}`",
            calendar_view_id,
            tables.schedule,
        );

        let lock_requests = vec![calendar_view_id];

        for view_id in lock_requests {
            let resp = self
                .build_request_v2(Method::PATCH, &format!("/meta/views/{}", view_id))
                .json(&json!({
                    "lock_type": "locked",
                }))
                .send()
                .await?;

            check_status(resp).await?;

            console_log!("Locked Noco view with ID `{}`", view_id,);
        }

        Ok(())
    }

    #[worker::send]
    pub async fn setup_base(&self, title: String) -> anyhow::Result<Url> {
        let base_id = self.create_base(title).await?;
        let tables = self.create_tables(&base_id).await?;
        let fields = self.create_fields(&tables).await?;
        self.create_views(&fields, &tables).await?;

        Ok(Url::parse(&format!(
            "{}dashboard/#/nc/{}",
            self.dash_origin, base_id
        ))?)
    }
}
