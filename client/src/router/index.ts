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
      path: "/app/:envId/events/:eventId",
      name: "event",
      component: () => import("../views/EventView.vue"),
    },
    {
      path: "/app/:envId/info",
      name: "info",
      component: () => import("../views/InfoView.vue"),
    },
  ],
});

export default router;
