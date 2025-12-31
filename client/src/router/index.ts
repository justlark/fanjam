import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/app/:envId",
      name: "app",
      redirect: { name: "schedule" },
    },
    {
      path: "/app/:envId/schedule/:dayIndex?",
      name: "schedule",
      component: () => import("../views/ScheduleView.vue"),
      beforeEnter: (to, from) => {
        // When navigating from an event back to the schedule view, we use this
        // fragment to scroll the schedule to the event the user was just
        // looking at. To make sure the browser's native back button works the
        // same way as this app's back button, we intercept the browser
        // navigation here.
        if (from.name !== "event") {
          return;
        }

        const newHash = `#event-${from.params.eventId as string}`;

        if (to.hash !== newHash) {
          return { ...to, hash: newHash };
        }
      },
    },
    {
      path: "/app/:envId/announcements",
      name: "announcements",
      component: () => import("../views/AnnouncementsView.vue"),
    },
    {
      path: "/app/:envId/info",
      name: "info",
      component: () => import("../views/InfoView.vue"),
    },
    {
      path: "/app/:envId/announcements/:announcementId",
      name: "announcement",
      component: () => import("../views/AnnouncementView.vue"),
    },
    {
      path: "/app/:envId/events/:eventId",
      name: "event",
      component: () => import("../views/EventView.vue"),
    },
    {
      path: "/app/:envId/pages/:pageId",
      name: "page",
      component: () => import("../views/PageView.vue"),
    },
    {
      path: "/app/:envId/:catchAll(.*)",
      redirect: { name: "app" },
    },
    {
      path: "/:catchAll(.*)",
      redirect: { name: "home" },
    },
  ],
});

export default router;
