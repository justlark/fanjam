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
      path: "/app/:id",
      name: "app",
      redirect: { name: "schedule" },
    },
    {
      path: "/app/:id/schedule",
      name: "schedule",
      component: () => import("../views/ScheduleView.vue"),
    },
    {
      path: "/app/:id/info",
      name: "info",
      component: () => import("../views/InfoView.vue"),
    },
  ],
});

export default router;
