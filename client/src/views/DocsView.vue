<script setup lang="ts">
import SitePage from "@/components/SitePage.vue";
import Divider from "primevue/divider";
</script>

<template>
  <SitePage>
    <div class="max-w-4xl mx-auto">
      <h1 class="text-4xl font-light">User Guide</h1>
      <Divider />
      <div id="document">
        <p>
          This is a guide for convention organizers planning to use
          <a href="https://fanjam.live/">FanJam</a> for their con. It outlines some of FanJam's less
          obvious features and things you should keep in mind when using it.
        </p>
        <h2>Terminology</h2>
        <p>
          FanJam is two things: a <em>dashboard</em> where staff can edit the con schedule, and an
          <em>app</em> where attendees can view it. We'll use these terms going forward.
        </p>
        <h2>Using FanJam offline</h2>
        <p>
          The app works completely offline. Once attendees have it open in their browser or
          installed on their phone, they can browse the schedule without an internet connection.
          Obviously they will not receive announcements or updates to the schedule until they are
          back online.
        </p>
        <h2>Installing FanJam on your phone</h2>
        <p>
          The app for attendees works fine in the web browser, but it can also be installed on your
          phone. You don't have to go through the App Store or Google Play; you open the app in your
          browser first, and then you have the option of installing it directly from there. The
          exact steps depend on the browser you're using and whether you're on iOS or Android. Some
          browsers will prompt you with a popup the first time you open the app. Other browsers
          require you to open the options menu and click "Install" or "Add to home screen". Firefox
          currently doesn't support this feature.
        </p>
        <img
          width="40%"
          src="https://static.fanjam.live/guide-fanjam-pwa-install-screenshot.png"
          alt="A screenshot of the FanJam app in Chrome"
        />
        <p>
          If you want to customize FanJam to use your con's logo as the app icon when it's installed
          on a phone, let us know; that's something we set up for you on our end.
        </p>
        <h2>Deciding when days roll over</h2>
        <p>
          Out of the box, each day in the schedule ends at midnight. However, if your con has events
          that stretch into the early hours of the morning, you may prefer that the day ends at
          2:00am or 3:00am instead. If you decide days should end at 2:00am, an event that starts at
          1:00am on Wednesday would appear in the app on Tuesday.
        </p>
        <p>This is something we configure on our end, so let us know your preference.</p>
        <h2>Setting the app's timezone</h2>
        <p>
          When we set up FanJam for your convention, let us know the timezone your con will be
          occurring in. The times for events will appear in that timezone regardless of where a user
          is located. If your con is remote, you may prefer that the app user the user's current
          timezone. Let us know either way.
        </p>
        <h2>Using the dashboard</h2>
        <p>
          The dashboard works a lot like a spreadsheet; you can sort, filter, and group events to
          help make sense of a chaotic schedule. Everything links to everything else, so you can do
          things like see not just which room an event is being held in, but a list of all events
          being held in that room.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-linked-records-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <h2>Sharing information about your con</h2>
        <p>
          The FanJam app isn't just for the schedule; you can put info, FAQs, policies, maps,
          socials, etc. in it as well. There are three main tools that will help you here:
        </p>
        <ul>
          <li>
            <strong>Links</strong>: You can add links to your website, socials, Discord server,
            Telegram group, or anything else.
          </li>
          <li>
            <strong>Pages</strong>: As an alternative to directing users to your website, you can
            add things like FAQs and policies directly to the app, which has the advantage of making
            them available offline.
          </li>
          <li>
            <strong>Files</strong>: In the <strong>About</strong> section of the dashboard, you can
            upload images or other files that attendees can view in the app. Use this to share a map
            of the venue, for example. Attendees can view these files offline too.
          </li>
        </ul>
        <p>
          Make sure you fill out the <strong>About</strong> section in the dashboard. At a minimum,
          make sure your con has a name and description to show in the app. This section is special
          in that trying to add more than one row doesn't do anything.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-about-table-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <h2>Sending announcements</h2>
        <p>
          You can use the dashboard to send out announcements that attendees can view in the app.
          They'll see a notification badge in the app when they have unread notifications, and if
          they opt in, they will also receive push notifications.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-announcements-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <p>
          If you would prefer to send out announcements through some other channel, like Telegram or
          Discord, let us know and we can hide the announcements section in the FanJam app to avoid
          any confusion.
        </p>
        <h2>Importing events from another system</h2>
        <p>
          You can import events into FanJam from another system by uploading a CSV file. This might
          be useful if you have a separate app for accepting panel applications, for example.
        </p>
        <p>
          In the dashboard, navigate to "Events" and click the "Upload" button in the top bar. The
          CSV file should have the following format:
        </p>
        <pre><code>Event Name,Summary,Description,Start Time,End Time,Locations,People,Categories,Tags
Event 1,My event summary,My event description,2026-01-01T00:00:00Z,2026-01-01T01:00:00Z,Location 1,"Person 1, Person 2",Category 1,"Tag 1, Tag 2, Tag 3"</code></pre>
        <p>Dates should be in the format <code>YYYY-MM-DDTHH:MM:SSZ</code>.</p>
        <p>
          The app will prompt you to manually map the column names if they don't match the format
          above exactly. You can also change the separator character for the People and Tags columns
          from commas to something else.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-upload-csv-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <p>
          You can upload data this way for other tables in the dashboard as well, like Locations,
          People, Categories, and Tags.
        </p>
        <p>
          When you upload a CSV file, you have the option to merge the data in the CSV file with the
          data already in the dashboard. Lets say you uploaded a list of events via CSV import,
          which added the speakers from each event to the People table. If you wanted to import a
          description for each speaker, you could upload a CSV file with the names and descriptions
          of each speaker and choose to merge the data. The dashboard will match the names of the
          speakers in the CSV file with the names of the speakers already in the People table and
          add the descriptions.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-merge-csv-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <h2>Leaving comments</h2>
        <p>
          In the dashboard, you can leave comments for other organizers. Comments are not visible to
          attendees in the app. You can right-click on any row to comment on events, rooms,
          panelists, and so on. You can give staff members the <strong>Commenter</strong> role to
          grant them permission to leave comments without the ability to edit the schedule itself.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-comments-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <h2>Hiding or cancelling events</h2>
        <p>
          You can check the <strong>Hidden</strong> box next to an event to hide it from attendees
          in the app. If an event is cancelled last-minute, this lets you remove it from the app
          while keeping the information visible to staff in the dashboard.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-hidden-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <h2>Linking to locations on a map</h2>
        <p>
          In the dashboard, you can optionally give each room a link to where attendees can find it
          on a map, which appears as a button in the app.
        </p>
        <img
          src="https://static.fanjam.live/guide-nocodb-map-links-screenshot.png"
          alt="A screenshot of the FanJam dashboard"
        />
        <img
          width="40%"
          src="https://static.fanjam.live/guide-fanjam-map-link-screenshot.png"
          alt="A screenshot of the FanJam app"
        />
        <h2>Publishing your con app</h2>
        <p>
          Attendees will find your con's app at <code>https://fanjam.live/app/geekcon</code>,
          replacing "geekcon" with whatever you want (within reason).
        </p>
        <p>
          However, before you're ready to announce your schedule to attendees, you might want a URL
          that only staff know. In that case, we'll give you a secret URL like
          <code>https://fanjam.live/app/25483810</code>. When you're ready to announce your
          schedule, we can change it to something more friendly.
        </p>
        <p>
          If you need your app's URL changed at any point, let us know. Also let us know if you want
          the old URL to redirect to the new one.
        </p>
        <p>
          If you prefer, you can also host FanJam at your own domain (e.g.
          <code>https://schedule.example.org</code>). All you need to do is add a
          <code>CNAME</code> record pointing to <code>fanjam.live</code>; whoever runs your website
          will know how to do this. Then let us know when you would like to make the switch. Your
          old
          <code>https://fanjam.live/app/geekcon</code>
          URL will redirect to the new one.
        </p>
        <h2>Preparing to announce your schedule</h2>
        <p>
          When you're editing the con schedule, changes you make in the dashboard should appear in
          the app in real time. However, when thousands of attendees are using the app at the same
          time, we may need to add a small delay to keep things running smoothly.
        </p>
        <p>
          Coordinate with us before you announce your con schedule publicly so we can tweak some
          settings on our end. Once we do, changes you make in the dashboard may take up to 30
          seconds to appear in the app.
        </p>
      </div>
    </div>
  </SitePage>
</template>

<style scoped>
@import "tailwindcss";

img {
  @apply rounded overflow-hidden shadow-lg;
}

pre {
  @apply overflow-x-scroll;
}

pre > code {
  @apply whitespace-pre;
}
</style>
