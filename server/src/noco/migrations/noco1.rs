use std::fmt;

use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use worker::console_log;

use super::common::{
    DATE_FORMAT, IS_TIME_12HR, NocoViewType, RefSetter, TIME_FORMAT, ViewId, check_status, set_nop,
    set_ref,
};

use super::common::{self, BaseId, Client, FieldId, TableId, Version};

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

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
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
                    "description": "The con schedule. Events here will appear on the schedule for attendees.",
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
                    "description": "The rooms at the venue. Assign each event to a room, and see which events are being held in each room.",
                    "fields": [
                        {
                            "title": "Room",
                            "type": "SingleLineText",
                            "description": "The name of the room where the event is being held."
                        }
                    ]
                }),
                table_id: &mut tables.rooms,
            },
            TableRequest {
                body: json!({
                    "title": "People",
                    "description": "People hosting events at the con. Panelists, presenters, etc. Assign people to events, and see which events are being hosted by whom.",
                    "fields": [
                        {
                            "title": "Name",
                            "type": "SingleLineText",
                            "description": "The person's name."
                        }
                    ]
                }),
                table_id: &mut tables.people,
            },
            TableRequest {
                body: json!({
                    "title": "Tags",
                    "description": "Tags for events. Label events with tags to help attendees find what they're looking for. You could group events into categories, tag some events as 18+, flag events that cost extra, etc.",
                    "fields": [
                        {
                            "title": "Tag",
                            "type": "SingleLineText",
                            "description": "The label to apply to the event."
                        }
                    ]

                }),
                table_id: &mut tables.tags,
            },
            TableRequest {
                body: json!({
                    "title": "Announcements",
                    "description": "Announcements to send to attendees. Make an announcement, and attendees can see them in the app.",
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
                    "description": "General information about the con. If you have multiple rows in this table, the most recent one is what will be shown in the app.",
                    "fields": [
                        {
                            "title": "Con Name",
                            "type": "SingleLineText",
                            "description": "The name of the con, which appears in the app."
                        }
                    ]
                }),
                table_id: &mut tables.about,
            },
            TableRequest {
                body: json!({
                    "title": "Links",
                    "description": "Links to external resources and information which attendees can view the app.",
                    "fields": [
                        {
                            "title": "Link Name",
                            "type": "SingleLineText",
                            "description": "The link text."
                        }
                    ]
                }),
                table_id: &mut tables.links,
            },
            TableRequest {
                body: json!({
                    "title": "Files",
                    "description": "Images, documents, etc. which attendees can view the app.",
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
                .client
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
                    "description": "The list of events being held in this room.",
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
                    "description": "The list of events this person is hosting.",
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
                    "description": "The list of events with this tag.",
                    "options": {
                        "relation_type": "mm",
                        "linked_table_id": &tables.schedule
                    }
                }),
            },
            FieldRequest {
                table_id: &tables.schedule,
                field_ref: set_nop(),
                body: json!({
                    "title": "Hidden",
                    "type": "Checkbox",
                    "description": "Hide this event from attendees. When checked, this event won't appear on the schedule."
                }),
            },
            FieldRequest {
                table_id: &tables.announcements,
                field_ref: set_nop(),
                body: json!({
                    "title": "Announcement",
                    "type": "LongText",
                    "description": "The text of the announcement itself.",
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
                    "description": "Attach images or other files to be shown alongside the announcement."
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
                    "description": "A brief description of the con, which appears in the app.",
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
                    "description": "A link to the con's website, which appears in the app.",
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
                .client
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
            .client
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
                .client
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
}

impl<'a> common::Migration<'a> for Migration<'a> {
    const INDEX: Version = Version::INITIAL.next();

    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn migrate(&self, base_id: BaseId) -> anyhow::Result<()> {
        let tables = self.create_tables(&base_id).await?;
        let fields = self.create_fields(&tables).await?;
        self.create_views(&fields, &tables).await?;

        Ok(())
    }
}
