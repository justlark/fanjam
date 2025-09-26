<script setup lang="ts">
import Card from "primevue/card";
import Button from "primevue/button";
import { RouterLink } from "vue-router";
</script>

<template>
  <main class="flex flex-col justify-center items-center gap-8 m-6">
    <h1 class="text-8xl text-center mt-8 lg:mt-14 mb-8 font-black">
      <span class="text-orange-400 dark:text-orange-300">Fan</span
      ><span class="text-sky-400 dark:text-sky-300">Jam</span>
    </h1>
    <div class="text-2xl text-center">
      <p>FanJam is an event planning app for conventions.</p>
    </div>
    <div class="text-lg text-justify max-w-4xl">
      <p>
        For small to medium-size cons, FanJam bridges the gap between a single person with a
        spreadsheet and the kinds of people who can afford enterprise event planning software. Free
        for small cons.
      </p>
    </div>
    <Card>
      <template #content>
        <div class="text-xl text-center">
          <p>Reach out if you're interested!</p>
          <p>
            <a
              class="text-primary decoration-2 underline underline-offset-2 hover:decoration-3 font-bold"
              href="mailto:lark@fanjam.live"
              >lark@fanjam.live</a
            >
          </p>
        </div>
      </template>
    </Card>
    <div class="flex flex-wrap gap-8 justify-evenly">
      <Card class="overflow-hidden">
        <template #header>
          <img
            class="max-h-100 object-contain"
            alt="Organizer app screenshot"
            src="/images/nocodb-demo-screenshot.webp"
          />
        </template>
        <template #title>For Organizers</template>
        <template #subtitle>
          A Google Docs-like spreadsheet app for putting together the con schedule
        </template>
        <template #content>
          <ul class="mb-2 list-disc list-inside">
            <li>Keep track of who's running what, when, and where.</li>
            <li>Publish event info, FAQs, policies, maps, socials, flyers, and anything else.</li>
            <li>Have as many organizers in the app as you want, and control who has access.</li>
            <li>Leave comments and notes that only organizers can see.</li>
          </ul>
        </template>
        <template #footer>
          <Button
            as="a"
            label="Open Demo"
            icon="pi pi-external-link"
            icon-pos="right"
            raised
            target="_blank"
            href="https://demo.fanjam.live/dashboard/#/base/be0bcc16-fbf9-4156-a19d-43fc20b86f95"
          />
        </template>
      </Card>
      <Card class="overflow-hidden">
        <template #header>
          <div class="h-100 object-contain flex">
            <img
              class="w-full"
              alt="Attendee app screenshot"
              src="/images/fanjam-schedule-demo-screenshot.webp"
            />
            <img
              class="w-full"
              alt="Attendee app screenshot"
              src="/images/fanjam-info-demo-screenshot.webp"
            />
          </div>
        </template>
        <template #title>For Attendees</template>
        <template #subtitle>A website and mobile app for viewing the con schedule</template>
        <template #content>
          <ul class="mb-2 list-disc list-inside">
            <li>See the schedule of events.</li>
            <li>Filter, search, and build a personal itinerary.</li>
            <li>See live announcements from the organizers.</li>
            <li>Works completely offline.</li>
          </ul>
        </template>
        <template #footer>
          <Button
            as="a"
            label="Open Demo"
            icon="pi pi-external-link"
            icon-pos="right"
            raised
            target="_blank"
            href="https://fanjam.live/app/804008/"
          />
        </template>
      </Card>
    </div>
  </main>
</template>
