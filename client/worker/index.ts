import { type Fetcher, type Request } from "@cloudflare/workers-types";

interface Env {
  ASSETS: Fetcher;
  API_DOMAIN: string;
  // The hostname this worker is canonically served from (e.g. `fanjam.live`).
  // Any other hostname reaching this worker must be a custom domain.
  CLIENT_DOMAIN: string;
  INJECT_METADATA: string;
}

const rootHeaders = {
  "X-Content-Type-Options": "nosniff",
  "X-Frame-Options": "DENY",
  // We need to include all of these origins in the `connect-src` so the
  // service worker can cache them and the app can work offline.
  "Content-Security-Policy":
    "default-src 'self'; img-src 'self' https://api.fanjam.live https://api-test.fanjam.live https://static.fanjam.live https://static-test.fanjam.live; connect-src 'self' https://api.fanjam.live https://api-test.fanjam.live https://umami.fanjam.live https://fonts.googleapis.com https://fonts.gstatic.com https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdn.jsdelivr.net; font-src 'self' https://fonts.gstatic.com https://cdn.jsdelivr.net; frame-ancestors 'none';",
  "Referrer-Policy": "strict-origin",
  "Strict-Transport-Security": "max-age=31536000; includeSubDomains; preload",
};

const appHeaders = {
  "X-Robots-Tag": "none",
};

interface ResponseEnvelope<T> {
  value: T;
}

const cloneResponseWithHeaders = (
  response: Response,
  headers: Record<string, string>,
): Response => {
  // Copy response headers from upstream request.
  const newHeaders = new Headers();
  for (const [key, value] of response.headers.entries()) {
    newHeaders.set(key, value);
  }

  const newResponse = new Response(response.body as ReadableStream<Uint8Array> | null, {
    status: response.status,
    statusText: response.statusText,
    headers: newHeaders,
  });

  for (const [key, value] of Object.entries(headers)) {
    newResponse.headers.set(key, value);
  }

  return newResponse;
};

interface File {
  id: string;
  name: string;
  media_type: string;
  signed_url: string;
}

interface AppInfo {
  env_name?: string;
  name?: string;
  description?: string;
}

const getAppInfo = async (apiDomain: string, envId: string): Promise<AppInfo> => {
  const response = await fetch(`https://${apiDomain}/apps/${envId}/info`);

  if (!response.ok) {
    return {};
  }

  try {
    const body = await response.json();
    return (body as ResponseEnvelope<AppInfo>).value;
  } catch {
    return {};
  }
};

const getFiles = async (apiDomain: string, envId: string): Promise<Array<File>> => {
  const response = await fetch(`https://${apiDomain}/apps/${envId}/files`);

  if (!response.ok) {
    return [];
  }

  try {
    const body = await response.json();
    return (body as ResponseEnvelope<{ files: Array<File> }>).value.files;
  } catch {
    return [];
  }
};

interface AppConfig {
  app_domain?: string;
  use_custom_icon: boolean;
  favicon_name?: string;
  opengraph_icon_name?: string;
  opengraph_icon_type?: string;
  opengraph_icon_alt?: string;
  pwa_short_app_name?: string;
  pwa_background_color?: string;
  pwa_icon_any_name?: string;
  pwa_icon_any_type?: string;
  pwa_icon_any_sizes?: string;
  pwa_icon_maskable_name?: string;
  pwa_icon_maskable_type?: string;
  pwa_icon_maskable_sizes?: string;
}

const getAppConfig = async (apiDomain: string, envId: string): Promise<AppConfig> => {
  const response = await fetch(`https://${apiDomain}/apps/${envId}/config`);

  if (!response.ok) {
    return {
      use_custom_icon: false,
    };
  }

  try {
    const body = await response.json();
    return body as AppConfig;
  } catch {
    return {
      use_custom_icon: false,
    };
  }
};

const resolveCustomHostname = async (
  apiDomain: string,
  hostname: string,
): Promise<string | undefined> => {
  const response = await fetch(`https://${apiDomain}/domains/${encodeURIComponent(hostname)}`);
  if (response.ok) {
    const body = await response.json();
    return (body as { env_id?: string }).env_id;
  }

  return undefined;
};

const fileToResponseHeaders = (file: File): Record<string, string> => ({
  "Content-Type": file.media_type,
  "Content-Disposition": `inline; filename="${file.name}"`,
});

const assetUrl = (apiDomain: string, envId: string, assetName: string): string =>
  `https://${apiDomain}/apps/${envId}/assets/${assetName}`;

const escapeRegex = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

const appPathRegex = new RegExp(`^/app/([^/]+)(?:/|$)`);

const matchFilePath = (pathname: string, publicPrefix: string): string | undefined => {
  const escaped = escapeRegex(publicPrefix);
  const match = new RegExp(`^${escaped}files/([^/]+)/?$`).exec(pathname);
  return match?.[1];
};

const matchManifestPath = (pathname: string, publicPrefix: string): boolean => {
  const escaped = escapeRegex(publicPrefix);
  return new RegExp(`^${escaped}app\\.webmanifest/?$`).test(pathname);
};

const interceptFileRequest = async (
  requestUrl: URL,
  env: Env,
  envId: string,
  publicPrefix: string,
): Promise<Response | undefined> => {
  const fileId = matchFilePath(requestUrl.pathname, publicPrefix);
  if (!fileId) return undefined;

  const files = await getFiles(env.API_DOMAIN, envId);
  const file = files.find((f) => f.id === fileId);
  if (!file) return undefined;

  const fileResponse = await fetch(file.signed_url);
  return cloneResponseWithHeaders(fileResponse, fileToResponseHeaders(file));
};

const webManifestResponse = async (
  requestUrl: URL,
  env: Env,
  envId: string,
  publicPrefix: string,
): Promise<Response | undefined> => {
  if (!matchManifestPath(requestUrl.pathname, publicPrefix)) {
    return undefined;
  }

  const appInfo = await getAppInfo(env.API_DOMAIN, envId);
  const appConfig = await getAppConfig(env.API_DOMAIN, envId);

  const mountUrl = `${requestUrl.origin}${publicPrefix}`;

  const webManifest = {
    name: appInfo.name ?? "FanJam",
    short_name: appConfig.pwa_short_app_name ?? appInfo.name ?? "FanJam",
    description: appInfo.description,
    scope: mountUrl,
    start_url: mountUrl,
    display: "standalone",
    background_color: appConfig.pwa_background_color,
    icons: [
      {
        src:
          appConfig.use_custom_icon && appConfig.pwa_icon_any_name !== undefined
            ? assetUrl(env.API_DOMAIN, envId, appConfig.pwa_icon_any_name)
            : `${requestUrl.origin}/icons/icon.png`,
        type:
          appConfig.use_custom_icon && appConfig.pwa_icon_any_type !== undefined
            ? appConfig.pwa_icon_any_type
            : "image/png",
        sizes:
          appConfig.use_custom_icon && appConfig.pwa_icon_any_sizes !== undefined
            ? appConfig.pwa_icon_any_sizes
            : undefined,
        purpose: "any",
      },
      {
        src:
          appConfig.use_custom_icon && appConfig.pwa_icon_maskable_name !== undefined
            ? assetUrl(env.API_DOMAIN, envId, appConfig.pwa_icon_maskable_name)
            : `${requestUrl.origin}/icons/icon-maskable.png`,
        type:
          appConfig.use_custom_icon && appConfig.pwa_icon_maskable_type !== undefined
            ? appConfig.pwa_icon_maskable_type
            : "image/png",
        sizes:
          appConfig.use_custom_icon && appConfig.pwa_icon_maskable_sizes !== undefined
            ? appConfig.pwa_icon_maskable_sizes
            : undefined,
        purpose: "maskable",
      },
    ],
  };

  return new Response(JSON.stringify(webManifest), {
    status: 200,
    headers: {
      "Content-Type": "application/manifest+json",
      "Cache-Control": "no-cache",
    },
  });
};

const injectWebManifestLink = (
  requestUrl: URL,
  publicPrefix: string,
  response: Response,
): Response => {
  const webManifestUrl = `${requestUrl.origin}${publicPrefix}app.webmanifest`;

  return new HTMLRewriter()
    .on("head", {
      element(element: Element) {
        element.append(`<link rel="manifest" href="${webManifestUrl}" />`, { html: true });
      },
    })
    .transform(response);
};

// Inject the resolved env ID as a meta tag so the Vue app can read it
// synchronously at boot and choose its routing mode:
// - `/` routes when using a custom domain.
// - `/app/:envId/` routes when using the default domain.
const injectEnvIdMeta = (envId: string, response: Response): Response => {
  return new HTMLRewriter()
    .on("head", {
      element(element: Element) {
        element.append(`<meta name="fanjam-env" content="${envId}" />`, { html: true });
      },
    })
    .transform(response);
};

const injectAppMetadata = async (
  env: Env,
  envId: string,
  response: Response,
): Promise<Response> => {
  const appInfo = await getAppInfo(env.API_DOMAIN, envId);
  const appConfig = await getAppConfig(env.API_DOMAIN, envId);

  return (
    new HTMLRewriter()
      .on("head > title", {
        element(element: Element) {
          if (appInfo.name) {
            element.setInnerContent(appInfo.name);
          }
        },
      })
      .on("head > meta[name='description']", {
        element(element: Element) {
          if (appInfo.description) {
            element.setAttribute("content", appInfo.description);
          }
        },
      })
      .on("head > link[rel='icon']", {
        element(element: Element) {
          if (appConfig.use_custom_icon && appConfig.favicon_name) {
            const iconUrl = assetUrl(env.API_DOMAIN, envId, appConfig.favicon_name);
            element.setAttribute("href", iconUrl);
          }
        },
      })
      .on("head > meta[property='og:title']", {
        element(element: Element) {
          if (appInfo.name) {
            element.setAttribute("content", appInfo.name);
          }
        },
      })
      .on("head > meta[property='og:description']", {
        element(element: Element) {
          if (appInfo.description) {
            element.setAttribute("content", appInfo.description);
          }
        },
      })
      .on("head > meta[property='og:image']", {
        element(element: Element) {
          if (appConfig.use_custom_icon && appConfig.opengraph_icon_name) {
            const iconUrl = assetUrl(env.API_DOMAIN, envId, appConfig.opengraph_icon_name);
            element.setAttribute("content", iconUrl);
          }
        },
      })
      .on("head > meta[property='og:image:type']", {
        element(element: Element) {
          if (appConfig.use_custom_icon) {
            if (appConfig.opengraph_icon_type) {
              element.setAttribute("content", appConfig.opengraph_icon_type);
            } else {
              element.remove();
            }
          }
        },
      })
      .on("head > meta[property='og:image:alt']", {
        element(element: Element) {
          if (appConfig.use_custom_icon) {
            if (appConfig.opengraph_icon_alt) {
              element.setAttribute("content", appConfig.opengraph_icon_alt);
            } else {
              element.remove();
            }
          }
        },
      })
      .on("head > meta[property='twitter:title']", {
        element(element: Element) {
          if (appInfo.name) {
            element.setAttribute("content", appInfo.name);
          }
        },
      })
      .on("head > meta[property='twitter:description']", {
        element(element: Element) {
          if (appInfo.description) {
            element.setAttribute("content", appInfo.description);
          }
        },
      })
      .on("head > meta[property='twitter:image']", {
        element(element: Element) {
          if (appConfig.use_custom_icon && appConfig.opengraph_icon_name) {
            const iconUrl = assetUrl(env.API_DOMAIN, envId, appConfig.opengraph_icon_name);
            element.setAttribute("content", iconUrl);
          }
        },
      })
      .on("head > meta[property='twitter:image:alt']", {
        element(element: Element) {
          if (appConfig.use_custom_icon) {
            if (appConfig.opengraph_icon_alt) {
              element.setAttribute("content", appConfig.opengraph_icon_alt);
            } else {
              element.remove();
            }
          }
        },
      })
      // We use Umami for privacy-preserving analytics. Here we're configuring it
      // to tag requests by environment name.
      .on("head > script[src='/stats.js']", {
        element(element: Element) {
          if (appInfo.env_name) {
            element.setAttribute("data-tag", `env/${appInfo.env_name}`);
          }
        },
      })
      .transform(response)
  );
};

// Set the Umami tag to "home" for non-app pages so we can track traffic to the
// homepage. The reason why we don't set this as the default `data-tag` in the
// `index.html` is to prevent Playwright tests and invalid app IDs from
// polluting our Umami metrics. Those requests will still be reported to Umami,
// but they won't have a tag, so we can filter them out.
const injectHomeMetadata = (response: Response): Response => {
  return new HTMLRewriter()
    .on("head > script[src='/stats.js']", {
      element(element: Element) {
        element.setAttribute("data-tag", "home");
      },
    })
    .transform(response);
};

const isHtmlResponse = (response: Response): boolean => {
  const contentType = response.headers.get("Content-Type") ?? "";
  return contentType.toLowerCase().split(";")[0].trim() == "text/html";
};

export default {
  async fetch(request: Request, env: Env) {
    const requestUrl = new URL(request.url);

    // uBlock Origin seems to block Umami's tracking script. To work around
    // that, we proxy it through this worker. I'm generally not a fan of trying
    // to circumvent privacy tools, but under the circumstances I think it's
    // justified. We're deliberately using a self-hosted analytics solution
    // because I care about my users' privacy.
    if (requestUrl.pathname === "/stats.js") {
      return await fetch("https://umami.fanjam.live/script.js");
    }

    const onDefaultHostname = requestUrl.hostname === env.CLIENT_DOMAIN;

    // Resolve the env ID and the path prefix the app is mounted at from the
    // request hostname and path.
    let envId: string | undefined;
    let publicPrefix: string | undefined;

    if (onDefaultHostname) {
      const match = appPathRegex.exec(requestUrl.pathname);
      if (match) {
        envId = match[1];
        publicPrefix = `/app/${envId}/`;
      }
    } else {
      const resolved = await resolveCustomHostname(env.API_DOMAIN, requestUrl.hostname);
      if (resolved === undefined) {
        // The request reached the worker on a hostname that isn't the default
        // and isn't mapped to any env. Return with a 404 Not Found instead of serving
        // the client app with broken metadata.
        return new Response("Not Found", { status: 404 });
      }

      envId = resolved;
      publicPrefix = "/";
    }

    // When an environment has a custom domain configured, requests to the
    // default domain should redirect to the custom domain. We send `Cache-Control:
    // no-cache` so that if an environment gets a custom domain added or removed,
    // the change takes effect immediately.
    if (onDefaultHostname && envId) {
      const config = await getAppConfig(env.API_DOMAIN, envId);

      if (config.app_domain) {
        const remainingPath = requestUrl.pathname.slice(`/app/${envId}`.length) || "/";
        const target = `https://${config.app_domain}${remainingPath}${requestUrl.search}${requestUrl.hash}`;

        return new Response(null, {
          status: 301,
          headers: {
            Location: target,
            "Cache-Control": "no-cache",
          },
        });
      }
    }

    // If this is the URL of a NocoDB attachment, get its signed URL and return
    // the file instead of forwarding the request to the CDN. We do this so
    // that attachments from NocoDB are served from this domain (instead of
    // from the signed S3 URL) and therefore can be cached by the service
    // worker.
    if (envId && publicPrefix) {
      const fileResponse = await interceptFileRequest(requestUrl, env, envId, publicPrefix);
      if (fileResponse) {
        return fileResponse;
      }

      // If this is the URL of a web manifest, generate it instead of forwarding
      // the request to the CDN.
      const webManifest = await webManifestResponse(requestUrl, env, envId, publicPrefix);
      if (webManifest) {
        return webManifest;
      }
    }

    const response = await env.ASSETS.fetch(request);
    let newResponse = cloneResponseWithHeaders(response, {});

    for (const [key, value] of Object.entries(rootHeaders)) {
      newResponse.headers.set(key, value);
    }

    if (envId !== undefined) {
      for (const [key, value] of Object.entries(appHeaders)) {
        newResponse.headers.set(key, value);
      }
    }

    // Make sure we only rewrite HTML responses.
    if (envId && publicPrefix && isHtmlResponse(newResponse)) {
      if (env.INJECT_METADATA === "true") {
        newResponse = await injectAppMetadata(env, envId, newResponse);
      }
      newResponse = injectWebManifestLink(requestUrl, publicPrefix, newResponse);
      newResponse = injectEnvIdMeta(envId, newResponse);
    } else if (!envId && isHtmlResponse(newResponse) && env.INJECT_METADATA === "true") {
      newResponse = injectHomeMetadata(newResponse);
    }

    return newResponse;
  },
};
