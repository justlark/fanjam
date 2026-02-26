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

let _exitingShare = false;

export const exitShareMode = () => {
  _exitingShare = true;
};

// Preserve the `share` query param across in-app navigations so the shared
// schedule view stays active until the user explicitly dismisses it.
router.beforeEach((to, from) => {
  if (_exitingShare) {
    _exitingShare = false;
    return;
  }
  if (from.query.share && !to.query.share) {
    return { ...to, query: { ...to.query, share: from.query.share } };
  }
});

export default router;
