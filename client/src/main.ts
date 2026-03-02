import "./assets/main.css";

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import PrimeVue from "primevue/config";
import { definePreset } from "@primeuix/themes";
import Aura from "@primeuix/themes/aura";
import ToastService from "primevue/toastservice";
import "primeicons/primeicons.css";

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
          color: "{blue.300}",
          hoverColor: "{blue.200}",
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
    },
  },
  components: {
    button: {
      colorScheme: {
        dark: {
          outlined: {
            primary: {
              borderColor: "{primary.color}",
            },
          },
        },
      },
    },
    toast: {
      colorScheme: {
        light: {
          root: { blur: "0" },
          info: { background: "{blue.50}", borderColor: "{blue.200}" },
          success: { background: "{green.50}", borderColor: "{green.200}" },
          warn: { background: "{yellow.50}", borderColor: "{yellow.200}" },
          error: { background: "{red.50}", borderColor: "{red.200}" },
          secondary: { background: "{surface.100}", borderColor: "{surface.200}" },
        },
        dark: {
          root: { blur: "0" },
          info: { background: "{blue.950}", borderColor: "{blue.800}", color: "{blue.300}", detailColor: "{surface.100}" },
          success: { background: "{green.950}", borderColor: "{green.800}", color: "{green.300}", detailColor: "{surface.100}" },
          warn: { background: "{yellow.950}", borderColor: "{yellow.800}", color: "{yellow.300}", detailColor: "{surface.100}" },
          error: { background: "{red.950}", borderColor: "{red.800}", color: "{red.300}", detailColor: "{surface.100}" },
          secondary: { background: "{surface.800}", borderColor: "{surface.700}", color: "{surface.200}", detailColor: "{surface.100}" },
        },
      },
    },
  },
});

app.directive("plaintext", {
  beforeMount(el) {
    el.innerHTML = el.innerText;
  },
});

app.use(PrimeVue, { theme: { preset } });
app.use(ToastService);
app.use(router);

app.mount("#app");
