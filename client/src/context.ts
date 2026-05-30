// Detects whether the app is running on a custom domain (e.g. `app.example.org`) or on the
// default `<client_domain>/app/<env_id>/...` URL. The client worker injects a `fanjam-env` meta
// tag into the SPA shell on custom domains; its presence is what flips the SPA into
// custom-domain mode, where the env id is fixed for the whole session and routes live at the
// origin root.

type EnvContext =
  | {
      mode: "default";
    }
  | {
      mode: "custom";
      // The env id baked into the page by the worker. The whole session belongs to this env;
      // changing it requires a full page reload (so the worker can re-resolve the hostname).
      envId: string;
    };

const readMetaTag = (): string | undefined => {
  if (typeof document === "undefined") return undefined;
  const meta = document.querySelector<HTMLMetaElement>('meta[name="fanjam-env"]');
  return meta?.content || undefined;
};

const detect = (): EnvContext => {
  const envId = readMetaTag();
  return envId ? { mode: "custom", envId } : { mode: "default" };
};

export const envContext: EnvContext = detect();
