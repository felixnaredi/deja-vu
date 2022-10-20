import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "@/pages/App.vue";
import "@/style.css";

createApp(App).use(createPinia()).mount("#app");
