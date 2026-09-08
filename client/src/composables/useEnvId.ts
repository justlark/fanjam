import { computed, type ComputedRef } from "vue";
import { useRoute } from "vue-router";
import { envContext } from "@/context";

// The current env ID. On the default domain, this comes from the URL path. On
// a custom domain, the client worker injects it into the DOM as a `<meta>`
// tag.
//
// In the case of the default domain, this updates dynamically as the user
// navigates between environments. For custom domains, a full page reload is
// required to change environments.
const useEnvId = (): ComputedRef<string> => {
  const route = useRoute();

  return computed(() =>
    envContext.mode === "custom" ? envContext.envId : (route.params.envId as string),
  );
};

export default useEnvId;
