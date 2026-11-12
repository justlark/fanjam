<script setup lang="ts">
import SitePage from "@/components/SitePage.vue";
import Divider from "primevue/divider";
</script>

<template>
  <SitePage>
    <div class="max-w-4xl mx-auto">
      <h1 class="text-4xl font-light">Privacy Policy</h1>
      <Divider />
      <div id="document">
        <p>
          FanJam is designed to prioritize user privacy; we limit the information we collect, and
          further limit the information we share. We do not sell your data.
        </p>
        <h2>Data we collect</h2>
        <p>
          No personally identifiable information is collected from convention attendees; attendees
          do not need to provide a name, email address, or any other personally identifying data to
          use the service. Convention organizers are required to provide an email address and set a
          password in order to log into the dashboard and receive notification emails. These email
          addresses are not shared or used for any other purpose.
        </p>
        <p>
          FanJam uses a self-hosted instance of
          <a href="https://umami.is/" target="_blank">Umami</a>—an open-source analytics solution—to
          collect anonymized data about convention attendees. This includes information about the
          user's browser, device, operating system, and approximate location. Umami does not store
          IP addresses. Each visitor is given a unique identifier to track their usage across
          sessions. These identifiers rotate monthly, which limits the time period over which Umami
          can know that two visits were from the same user.
        </p>
        <p>
          We use these analytics to learn more about how different app features are utilized; how
          visitors are discovering FanJam; what browsers, operating systems, and screen sizes are
          most commonly used; and the volume of traffic a convention is receiving. This data may be
          shared with convention organizers, but <strong>in aggregate form only</strong>. Otherwise,
          this data is not shared. Because we self-host Umami, data is not sent to third-party
          analytics providers.
        </p>
        <p>
          In the app, convention attendees can save events to build a personal schedule. These
          schedules are stored for 90 days, after which they are automatically deleted. Each
          schedule is identified by a random ID which is not associated with the analytics collected
          by Umami, meaning that schedules cannot be linked to individual users. These schedules are
          stored to allow attendees to sync their schedule between devices and to perform analytics
          about convention attendance. These analytics may be shared with convention organizers, but
          <strong>in aggregate form only</strong>, meaning convention organizers cannot see
          individual schedules.
        </p>
        <h2>Data subprocessors</h2>
        <p>FanJam works with the following data subprocessors.</p>
        <ul>
          <li>
            <a href="https://www.cloudflare.com/" target="_blank">Cloudflare</a> provides core
            hosting infrastructure.
          </li>
          <li><a href="https://fly.io/" target="_blank">Fly.io</a> provides server hosting.</li>
          <li><a href="https://neon.com/" target="_blank">Neon</a> provides database hosting.</li>
          <li>
            <a href="https://mailersend.com/" target="_blank">MailerSend</a> provides email
            services.
          </li>
        </ul>
      </div>
    </div>
  </SitePage>
</template>
