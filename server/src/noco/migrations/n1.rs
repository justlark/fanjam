use reqwest::Method;
use serde_json::json;
use worker::console_log;

use crate::noco::{Client, client::check_status};

use super::common::{
    DATE_FORMAT, FieldRequest, IS_TIME_12HR, TIME_FORMAT, TableRequest, ViewId, ViewRequest,
    ViewType, create_fields, create_tables, create_views, lock_views, set_nop, set_ref,
};

use super::common::{self, BaseId, FieldId, TableId, Version};

#[derive(Debug, Default)]
struct ByTable<T> {
    schedule: T,
    locations: T,
    people: T,
    tags: T,
    announcements: T,
    about: T,
    links: T,
    files: T,
}

type Tables = ByTable<TableId>;

impl<T> ByTable<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByTable<U> {
        ByTable {
            schedule: f(self.schedule),
            locations: f(self.locations),
            people: f(self.people),
            tags: f(self.tags),
            announcements: f(self.announcements),
            about: f(self.about),
            links: f(self.links),
            files: f(self.files),
        }
    }
}

#[derive(Debug, Default)]
struct ByField<T> {
    start_time: T,
}

type Fields = ByField<FieldId>;

impl<T> ByField<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByField<U> {
        ByField {
            start_time: f(self.start_time),
        }
    }
}

#[derive(Debug, Default)]
struct ByView<T> {
    calendar: T,
    add_event: T,
    make_announcement: T,
}

type Views = ByView<ViewId>;

impl<T> ByView<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByView<U> {
        ByView {
            calendar: f(self.calendar),
            add_event: f(self.add_event),
            make_announcement: f(self.make_announcement),
        }
    }
}

pub struct Migration<'a> {
    client: &'a Client,
}

impl Migration<'_> {
    async fn create_tables(&self, base_id: &BaseId) -> anyhow::Result<Tables> {
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
                    "title": "Events",
                    "description": "The con schedule. Events here will appear on the schedule for attendees.",
                    "fields": [
                        {
                            "title": "Event Name",
                            "type": "SingleLineText",
                            "description": "The name of the panel, workshop, class, etc."
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.schedule),
            },
            TableRequest {
                body: json!({
                    "title": "Locations",
                    "description": "The rooms, buildings, stages, etc. at the venue. Assign each event to a location, and see which events are being held where.",
                    "fields": [
                        {
                            "title": "Location",
                            "type": "SingleLineText",
                            "description": "The name of the location where the event is being held."
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.locations),
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
                table_ref: set_ref(&mut tables.people),
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
                table_ref: set_ref(&mut tables.tags),
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
                table_ref: set_ref(&mut tables.announcements),
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
                table_ref: set_ref(&mut tables.about),
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
                table_ref: set_ref(&mut tables.links),
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
                table_ref: set_ref(&mut tables.files),
            },
        ];

        create_tables(self.client, base_id, requests).await?;

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn create_fields(&self, tables: &Tables) -> anyhow::Result<Fields> {
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
                table_id: &tables.locations,
                field_ref: set_nop(),
                body: json!({
                    "title": "Events",
                    "type": "Links",
                    "description": "The list of events being held at this location.",
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
                    "description": "Hide this event from attendees. When checked, this event won't appear on the schedule.",
                    "options": {
                        "icon": "square"
                    }
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

        create_fields(self.client, requests).await?;

        Ok(fields.map(|id| id.expect("expected field ID, found none")))
    }

    async fn create_views(&self, fields: &Fields, tables: &Tables) -> anyhow::Result<Views> {
        let mut views = ByView::<Option<ViewId>>::default();

        let requests = vec![
            ViewRequest {
                body: json!({
                    "title": "Calendar",
                    "type": ViewType::Calendar.code(),
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
                }),
                kind: ViewType::Calendar,
                table_id: tables.schedule.clone(),
                table_ref: set_ref(&mut views.calendar),
            },
            ViewRequest {
                body: json!({
                    "title": "Add Event",
                    "type": ViewType::Form.code()
                }),
                kind: ViewType::Form,
                table_id: tables.schedule.clone(),
                table_ref: set_ref(&mut views.add_event),
            },
            ViewRequest {
                body: json!({
                    "title": "Make Announcement",
                    "type": ViewType::Form.code()
                }),
                kind: ViewType::Form,
                table_id: tables.announcements.clone(),
                table_ref: set_ref(&mut views.make_announcement),
            },
        ];

        create_views(self.client, requests).await?;

        let views = views.map(|id| id.expect("expected view ID, found none"));

        let resp = self
            .client
            .build_request_v2(Method::PATCH, &format!("/meta/forms/{}", &views.add_event))
            .json(&json!({
                "heading": "Add Event",
                "subheading": "Add an event to the schedule.",
                "submit_another_form": true,
                "show_blank_form": true,
                "success_msg": "Event added!"
            }))
            .send()
            .await?;

        check_status(resp).await?;

        console_log!("Updated Noco form view with ID `{}`", views.add_event);

        let resp = self
            .client
            .build_request_v2(
                Method::PATCH,
                &format!("/meta/forms/{}", &views.make_announcement),
            )
            .json(&json!({
                "heading": "Make Announcement",
                "subheading": "Make an announcement which is sent to attendees.",
                "submit_another_form": true,
                "show_blank_form": true,
                "success_msg": "Announcement sent!"
            }))
            .send()
            .await?;

        check_status(resp).await?;

        console_log!(
            "Updated Noco form view with ID `{}`",
            views.make_announcement
        );

        let views_to_lock = vec![
            views.calendar.clone(),
            views.add_event.clone(),
            views.make_announcement.clone(),
        ];

        lock_views(self.client, views_to_lock).await?;

        Ok(views)
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
