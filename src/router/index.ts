import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import GameView from "@/views/GameView.vue";
import HistoryView from "@/views/HistoryView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: GameView,
  },
  {
    path: "/history",
    name: "history",
    component: HistoryView,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
