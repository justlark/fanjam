use std::marker::PhantomData;

use serde_json::json;
use worker::{Method, console_log};

use crate::noco::Client;

use super::common::{
    ColumnRequest, DATE_FORMAT, IS_TIME_12HR, TIME_FORMAT, TableRequest, ViewId, ViewRequest,
    ViewType, create_columns, create_tables, create_views, lock_views, set_nop, set_ref,
};

use super::common::{self, BaseId, ColumnId, TableId, Version};

#[derive(Debug, Default)]
struct ByTable<T> {
    events: T,
    locations: T,
    people: T,
    categories: T,
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
            events: f(self.events),
            locations: f(self.locations),
            people: f(self.people),
            categories: f(self.categories),
            tags: f(self.tags),
            announcements: f(self.announcements),
            about: f(self.about),
            links: f(self.links),
            files: f(self.files),
        }
    }
}

// Placeholder in case we ever want to return generated column IDs.
#[derive(Debug, Default)]
struct ByColumn<T> {
    _phantom: PhantomData<T>,
}

type Columns = ByColumn<ColumnId>;

impl<T> ByColumn<T> {
    fn map<U>(self, _f: impl Fn(T) -> U) -> ByColumn<U> {
        ByColumn {
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default)]
struct ByView<T> {
    add_event: T,
    make_announcement: T,
}

type Views = ByView<ViewId>;

impl<T> ByView<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> ByView<U> {
        ByView {
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

        // NocoDB has a concept of a "primary value" (different from a primary key), which is the
        // column that appears in the UI as the short representation of a table row. We must set
        // the primary value here, otherwise the `ID` column (the primary key) will be designated
        // the primary value, which we don't want, because we want it to be hidden.
        //
        // We create the rest of the columns elsewhere, because they need to be created in a
        // specific order and after all the tables have been created so that we can set up the
        // links between them while controlling the column order within each table.
        let requests = vec![
            TableRequest {
                body: json!({
                    "table_name": "events",
                    "title": "Events",
                    "description": "The con schedule. Events here will appear on the schedule for attendees.",
                    "meta": {
                        "icon": "🗓️"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Event Name",
                            "uidt": "SingleLineText",
                            "description": "The name of the panel, workshop, class, etc.",
                            "pv": true,
                            "rqd": true,
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.events),
            },
            TableRequest {
                body: json!({
                    "table_name": "locations",
                    "title": "Locations",
                    "description": "The rooms, buildings, stages, etc. at the venue. Assign each event to a location, and see which events are being held where.",
                    "meta": {
                        "icon": "📍"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Location",
                            "uidt": "SingleLineText",
                            "description": "The name of the location where the event is being held.",
                            "rqd": true,
                            "pv": true
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.locations),
            },
            TableRequest {
                body: json!({
                    "table_name": "people",
                    "title": "People",
                    "description": "People hosting events at the con. Panelists, presenters, etc. Assign people to events, and see which events are being hosted by whom.",
                    "meta": {
                        "icon": "🧒"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Name",
                            "uidt": "SingleLineText",
                            "description": "The person's name.",
                            "rqd": true,
                            "pv": true,
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.people),
            },
            TableRequest {
                body: json!({
                    "table_name": "categories",
                    "title": "Categories",
                    "description": "Event categories. Group events into categories to help attendees find what they're looking for. Events can only belong to a single category. Use tags to add more.",
                    "meta": {
                        "icon": "📁"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Category",
                            "uidt": "SingleLineText",
                            "description": "The name of the category.",
                            "rqd": true,
                            "pv": true,
                        }
                    ]

                }),
                table_ref: set_ref(&mut tables.categories),
            },
            TableRequest {
                body: json!({
                    "table_name": "tags",
                    "title": "Tags",
                    "description": "Tags for events. Label events with tags to help attendees find what they're looking for. You could tag some events as 18+, flag events that cost extra, etc.",
                    "meta": {
                        "icon": "🏷️"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Tag",
                            "uidt": "SingleLineText",
                            "description": "The label to apply to the event.",
                            "rqd": true,
                            "pv": true,
                        }
                    ]

                }),
                table_ref: set_ref(&mut tables.tags),
            },
            TableRequest {
                body: json!({
                    "table_name": "announcements",
                    "title": "Announcements",
                    "description": "Announcements to send to attendees. Make an announcement, and attendees can see them in the app.",
                    "meta": {
                        "icon": "📣"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "title",
                            "title": "Title",
                            "uidt": "SingleLineText",
                            "description": "The title of the announcement.",
                            "rqd": true,
                            "pv": true
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.announcements),
            },
            TableRequest {
                body: json!({
                    "table_name": "about",
                    "title": "About",
                    "description": "General information about the con. If you have multiple rows in this table, the most recent one is what will be shown in the app.",
                    "meta": {
                        "icon": "ℹ️"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "con_name",
                            "title": "Con Name",
                            "uidt": "SingleLineText",
                            "description": "The name of the con, which appears in the app.",
                            "rqd": true,
                            "pv": true
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.about),
            },
            TableRequest {
                body: json!({
                    "table_name": "links",
                    "title": "Links",
                    "description": "Links to external resources and information which attendees can view the app.",
                    "meta": {
                        "icon": "🔗"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "Link Name",
                            "uidt": "SingleLineText",
                            "description": "The link text.",
                            "rqd": true,
                            "pv": true
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.links),
            },
            TableRequest {
                body: json!({
                    "table_name": "files",
                    "title": "Files",
                    "description": "Images, documents, etc. which attendees can view the app.",
                    "meta": {
                        "icon": "🗃️"
                    },
                    "columns": [
                        {
                            "column_name": "id",
                            "title": "ID",
                            "uidt": "ID"
                        },
                        {
                            "column_name": "name",
                            "title": "File Name",
                            "uidt": "SingleLineText",
                            "description": "The name of the file.",
                            "rqd": true,
                            "pv": true
                        }
                    ]
                }),
                table_ref: set_ref(&mut tables.files),
            },
        ];

        create_tables(self.client, base_id, requests).await?;

        Ok(tables.map(|id| id.expect("expected table ID, found none")))
    }

    async fn create_columns(&self, tables: &Tables) -> anyhow::Result<Columns> {
        let columns = ByColumn::<Option<ColumnId>>::default();

        let requests = vec![
            ColumnRequest {
                table_id: &tables.events,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "description",
                    "title": "Description",
                    "uidt": "LongText",
                    "description": "A description of the event.",
                    "meta": {
                        "richMode": true
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.events,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "start_time",
                    "title": "Start Time",
                    "uidt": "DateTime",
                    "description": "The day and time the event starts.",
                    "meta": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "is12hrFormat": IS_TIME_12HR
                    },
                    "rqd": true,
                }),
            },
            ColumnRequest {
                table_id: &tables.events,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "end_time",
                    "title": "End Time",
                    "uidt": "DateTime",
                    "description": "The day and time the event ends.",
                    "meta": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "is12hrFormat": IS_TIME_12HR
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.locations,
                column_ref: set_nop(),
                body: json!({
                    "colunn_name": "events",
                    "title": "Events",
                    "uidt": "Links",
                    "description": "The list of events being held at this location.",
                    "type": "hm",
                    "parentId": &tables.locations,
                    "childId": &tables.events
                }),
            },
            ColumnRequest {
                table_id: &tables.people,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "contact_info",
                    "title": "Contact Info",
                    "uidt": "SingleLineText",
                    "description": "Contact info for this person. Attendees cannot see this."
                }),
            },
            ColumnRequest {
                table_id: &tables.people,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "events",
                    "title": "Events",
                    "uidt": "Links",
                    "description": "The list of events this person is hosting.",
                    "type": "mm",
                    "parentId": &tables.people,
                    "childId": &tables.events
                }),
            },
            ColumnRequest {
                table_id: &tables.categories,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "events",
                    "title": "Events",
                    "uidt": "Links",
                    "description": "The list of events in this category.",
                    "type": "hm",
                    "parentId": &tables.categories,
                    "childId": &tables.events
                }),
            },
            ColumnRequest {
                table_id: &tables.tags,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "events",
                    "title": "Events",
                    "uidt": "Links",
                    "description": "The list of events with this tag.",
                    "type": "mm",
                    "parentId": &tables.tags,
                    "childId": &tables.events
                }),
            },
            ColumnRequest {
                table_id: &tables.events,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "hidden",
                    "title": "Hidden",
                    "uidt": "Checkbox",
                    "description": "Hide this event from attendees. When checked, this event won't appear on the schedule.",
                    "meta": {
                        "iconIdx": 0
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.announcements,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "description",
                    "title": "Announcement",
                    "uidt": "LongText",
                    "description": "The text of the announcement itself.",
                    "meta": {
                        "richMode": true
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.announcements,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "attachment",
                    "title": "Files",
                    "uidt": "Attachment",
                    "description": "Attach images or other files to be shown alongside the announcement."
                }),
            },
            ColumnRequest {
                table_id: &tables.announcements,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "created",
                    "title": "Created",
                    "uidt": "CreatedTime",
                    "description": "When this announcement was first created.",
                    "meta": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "is12hrFormat": IS_TIME_12HR
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.announcements,
                column_ref: set_nop(),
                body: json!({
                    "column": "last_edited",
                    "title": "Last Edited",
                    "uidt": "LastModifiedTime",
                    "description": "When this announcement was last edited.",
                    "meta": {
                        "date_format": DATE_FORMAT,
                        "time_format": TIME_FORMAT,
                        "is12hrFormat": IS_TIME_12HR
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.about,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "con_description",
                    "title": "Con Description",
                    "uidt": "LongText",
                    "description": "A brief description of the con, which appears in the app.",
                    "meta": {
                        "richMode": false
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.about,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "website",
                    "title": "Website",
                    "uidt": "URL",
                    "description": "A link to the con's website, which appears in the app.",
                    "meta": {
                        "validation": true
                    }
                }),
            },
            ColumnRequest {
                table_id: &tables.links,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "url",
                    "title": "URL",
                    "uidt": "URL",
                    "description": "The link URL.",
                    "meta": {
                        "validation": true
                    },
                    "rqd": true,
                }),
            },
            ColumnRequest {
                table_id: &tables.files,
                column_ref: set_nop(),
                body: json!({
                    "column_name": "file",
                    "title": "File",
                    "uidt": "Attachment",
                    "description": "The image, document, etc. to upload.",
                    "rqd": true,
                }),
            },
        ];

        create_columns(self.client, requests).await?;

        Ok(columns.map(|id| id.expect("expected column ID, found none")))
    }

    async fn create_views(&self, tables: &Tables) -> anyhow::Result<Views> {
        let mut views = ByView::<Option<ViewId>>::default();

        let requests = vec![
            ViewRequest {
                body: json!({
                    "title": "Add Event",
                    "type": ViewType::Form.code()
                }),
                kind: ViewType::Form,
                table_id: tables.events.clone(),
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

        self.client
            .build_request(Method::Patch, &format!("/meta/forms/{}", &views.add_event))
            .with_json(&json!({
                "heading": "Add Event",
                "subheading": "Add an event to the schedule.",
                "submit_another_form": true,
                "show_blank_form": true,
                "success_msg": "Event added!"
            }))?
            .exec()
            .await?;

        console_log!("Updated Noco form view with ID `{}`", views.add_event);

        self.client
            .build_request(
                Method::Patch,
                &format!("/meta/forms/{}", &views.make_announcement),
            )
            .with_json(&json!({
                "heading": "Make Announcement",
                "subheading": "Make an announcement which is sent to attendees.",
                "submit_another_form": true,
                "show_blank_form": true,
                "success_msg": "Announcement sent!"
            }))?
            .exec()
            .await?;

        console_log!(
            "Updated Noco form view with ID `{}`",
            views.make_announcement
        );

        let views_to_lock = vec![views.add_event.clone(), views.make_announcement.clone()];

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
        self.create_columns(&tables).await?;
        self.create_views(&tables).await?;

        Ok(())
    }
}
