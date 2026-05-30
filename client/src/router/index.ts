import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import { envContext } from "@/context";

// All view routes use stable names so in-app navigation (`router.push({ name: "schedule" })`)
// works identically on both the default hostname and on custom domains. Only the paths differ:
// on the default hostname the app is mounted under `/app/:envId/`, on a custom domain it's at
// the origin root.

const defaultRoutes: Array<RouteRecordRaw> = [
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
    path: "/app/:envId/share",
    name: "share",
    redirect: (to) => ({
      name: "schedule",
      params: { envId: to.params.envId, dayIndex: "all" },
      query: { star: "true", share: to.query.s },
    }),
  },
  {
    path: "/app/:envId/:catchAll(.*)",
    redirect: { name: "app" },
  },
  {
    path: "/:catchAll(.*)",
    redirect: { name: "home" },
  },
];

// On a custom domain the whole origin belongs to one env, so paths drop the `/app/:envId`
// segment. There's no separate FanJam landing page in this mode — `/` enters the app directly.
const customRoutes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "app",
    redirect: { name: "schedule" },
  },
  {
    path: "/schedule/:dayIndex?",
    name: "schedule",
    component: () => import("../views/ScheduleView.vue"),
  },
  {
    path: "/announcements",
    name: "announcements",
    component: () => import("../views/AnnouncementsView.vue"),
  },
  {
    path: "/info",
    name: "info",
    component: () => import("../views/InfoView.vue"),
  },
  {
    path: "/announcements/:announcementId",
    name: "announcement",
    component: () => import("../views/AnnouncementView.vue"),
  },
  {
    path: "/events/:eventId",
    name: "event",
    component: () => import("../views/EventView.vue"),
  },
  {
    path: "/pages/:pageId",
    name: "page",
    component: () => import("../views/PageView.vue"),
  },
  {
    path: "/share",
    name: "share",
    redirect: (to) => ({
      name: "schedule",
      params: { dayIndex: "all" },
      query: { star: "true", share: to.query.s },
    }),
  },
  {
    path: "/:catchAll(.*)",
    redirect: { name: "app" },
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: envContext.mode === "custom" ? customRoutes : defaultRoutes,
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
