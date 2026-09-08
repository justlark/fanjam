import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";
import { envContext } from "@/context";
import { maybeCheckForAppUpdate } from "@/utils/appUpdate";
import { onIdle } from "@/utils/idle";

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
    path: "/app/:envId/sync",
    name: "sync",
    redirect: (to) => {
      // Adopt the sync code synchronously so it's present before the schedule
      // view mounts and runs its initial pull. Add a query param that tells
      // the app to show a toast.
      const query: Record<string, string> = {};
      if (typeof to.query.s === "string") {
        localStorage.setItem(`sync:${to.params.envId as string}`, to.query.s);
        query.synced = "true";
      }

      return { name: "schedule", params: { envId: to.params.envId, dayIndex: "all" }, query };
    },
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
    path: "/sync",
    name: "sync",
    redirect: (to) => {
      // On a custom domain the env ID isn't in the path; it comes from `envContext`.
      const query: Record<string, string> = {};
      if (envContext.mode === "custom" && typeof to.query.s === "string") {
        localStorage.setItem(`sync:${envContext.envId}`, to.query.s);
        query.synced = "true";
      }

      return { name: "schedule", params: { dayIndex: "all" }, query };
    },
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

// Poll for a newer client bundle as the user navigates around, rather than
// using a timer.
router.afterEach((_to, _from, failure) => {
  if (failure) return;

  onIdle(() => void maybeCheckForAppUpdate());
});

export default router;
