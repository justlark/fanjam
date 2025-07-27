import "./assets/main.css";

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import PrimeVue from "primevue/config";
import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";
import ToastService from "primevue/toastservice";

const app = createApp(App);

const preset = definePreset(Aura, {
  semantic: {
    colorScheme: {
      light: {
        primary: {
          color: "{blue.600}",
          hoverColor: "{blue.700}",
          50: "{blue.50}",
          100: "{blue.100}",
          200: "{blue.200}",
          300: "{blue.300}",
          400: "{blue.400}",
          500: "{blue.500}",
          600: "{blue.600}",
          700: "{blue.700}",
          800: "{blue.800}",
          900: "{blue.900}",
          950: "{blue.950}",
        },
      },
      dark: {
        primary: {
          50: "{teal.50}",
          100: "{teal.100}",
          200: "{teal.200}",
          300: "{teal.300}",
          400: "{teal.400}",
          500: "{teal.500}",
          600: "{teal.600}",
          700: "{teal.700}",
          800: "{teal.800}",
          900: "{teal.900}",
          950: "{teal.950}",
        },
      },
    },
  },
});

app.use(router);
app.use(PrimeVue, { theme: { preset } });
app.use(ToastService);

app.mount("#app");
