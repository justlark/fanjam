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
          50: "{orange.50}",
          100: "{orange.100}",
          200: "{orange.200}",
          300: "{orange.300}",
          400: "{orange.400}",
          500: "{orange.500}",
          600: "{orange.600}",
          700: "{orange.700}",
          800: "{orange.800}",
          900: "{orange.900}",
          950: "{orange.950}",
        },
      },
    },
  },
});

app.use(router);
app.use(PrimeVue, { theme: { preset } });
app.use(ToastService);

app.mount("#app");
