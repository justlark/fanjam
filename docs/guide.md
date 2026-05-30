# User Guide

This is a guide for convention organizers planning to use
[FanJam](https://fanjam.live) for their con. It outlines some of FanJam's less
obvious features and things you should keep in mind when using it.

## Terminology

FanJam is two things: a _dashboard_ where staff can edit the con schedule, and
an _app_ where attendees can view it. We'll use these terms going forward.

## Using the dashboard

![Screenshot](https://static.fanjam.live/guide-nocodb-linked-records-screenshot.png)

The dashboard works a lot like a spreadsheet; you can sort, filter, and group
events to help make sense of a chaotic schedule. Everything links to everything
else, so you can do things like see not just which room an event is being held
in, but a list of all events being held in that room.

## Sharing information about your con

The FanJam app isn't just for the schedule; you can put info, FAQs, policies,
maps, socials, and pretty much anything else you want in it. There are three
main tools that will help you here:

- **Links**: You can add links to your website, socials, Discord server,
  Telegram group, or anything else.
- **Pages**: As an alternative to directing users to your website, you can add
  things like FAQs and policies directly to the app, which has the advantage of
  making them available offline.
- **Files**: In the **About** section of the dashboard, you can upload images
  or other files that attendees can view in the app. Use this to share a map of
  the venue, for example. Attendees can view these files offline too, but unlike
  everything else in the app, the user needs to open a file while they're online
  before it's available offline. This is a known limitation that may get fixed in
  the future.

Make sure you fill out the **About** section in the dashboard. At a minimum,
make sure your con has a name and description to show in the app. This section
is special in that trying to add more than one row doesn't do anything.

## Sending announcements

You can use the dashboard to send out announcements that attendees can view in
the app. They'll see a notification badge in the app when they have unread
notifications, but they will not receive push notifications. We may add support
for push notifications in the future.

If you would prefer to send out announcements through some other channel, like
Telegram or Discord, let us know and we can hide the announcements section in
the FanJam app to avoid any confusion.

## Importing events from another system

![Screenshot](https://static.fanjam.live/guide-nocodb-csv-import-screenshot.png)

You can import events into FanJam from another system by uploading a CSV file.
This might be useful if you have a separate app for accepting panel
applications, for example.

In the dashboard, navigate to `Events > ⋮ > Upload > CSV`. The CSV file should
have the following format:

```csv
Event Name,Summary,Description,Start Time,End Time,Locations,People,Categories,Tags
Event 1,My event summary,My event description,2026-01-01T00:00:00Z,2026-01-01T01:00:00Z,Location 1,"Person 1, Person 2",Category 1,"Tag 1, Tag 2, Tag 3"
```

Dates should be in the format `YYYY-MM-DDTHH:MM:SSZ`.

The app will prompt you to manually map the column names if they don't match
the format above exactly. You can also change the separator character for the
People and Tags columns from commas to something else.

## Leaving comments

![Screenshot](https://static.fanjam.live/guide-nocodb-comments-screenshot.png)

In the dashboard, you can leave comments for other organizers. Comments are not
visible to attendees in the app. You can right-click on any row to comment on
events, rooms, panelists, and so on. You can give staff members the
**Commenter** role to grant them permission to leave comments without the
ability to edit the schedule itself.

## Hiding or cancelling events

![Screenshot](https://static.fanjam.live/guide-nocodb-hidden-screenshot.png)

You can check the **Hidden** box next to an event to hide it from attendees in
the app. If you're trying to add a last-minute event after your schedule has
been announced, this lets you hide the new event until you're done editing. If
an event is cancelled last-minute, this lets you remove it from the app while
keeping the information visible to staff in the dashboard.

## Publishing your con app

Attendees will find your con's app at `https://fanjam.live/app/geekcon`,
replacing "geekcon" with whatever you want (within reason).

However, before you're ready to announce your schedule to attendees, you might
want a URL that only staff know. In that case, we'll give you a secret URL like
`https://fanjam.live/app/25483810`. When you're ready to announce your
schedule, we can change it to something more friendly.

If you need your app's URL changed at any point, let us know. Also let us know
if you want the old URL to redirect to the new one.

## Preparing to announce your schedule

When you're editing the con schedule, changes you make in the dashboard should
appear in the app in real time. However, when thousands of attendees are using
the app at the same time, we may need to add a small delay to keep things
running smoothly.

Coordinate with us before you announce your con schedule publicly so we can
tweak some settings on our end. Once we do, changes you make in the dashboard
may take up to 30 seconds to appear in the app.
