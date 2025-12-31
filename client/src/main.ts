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
