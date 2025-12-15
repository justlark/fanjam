import { type Fetcher, type Request } from "@cloudflare/workers-types";

interface Env {
  ASSETS: Fetcher;
  API_DOMAIN: string;
  INJECT_METADATA: string;
}

const headerPatterns = {
  "/": {
    "X-Content-Type-Options": "nosniff",
    "X-Frame-Options": "DENY",
    // We need to include all of these origins in the `connect-src` so the
    // service worker can cache them and the app can work offline.
    "Content-Security-Policy":
      "default-src 'self'; img-src 'self' https://api.fanjam.live https://api-test.fanjam.live; connect-src 'self' https://api.fanjam.live https://api-test.fanjam.live https://umami.fanjam.live https://fonts.googleapis.com https://fonts.gstatic.com https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdn.jsdelivr.net; font-src 'self' https://fonts.gstatic.com https://cdn.jsdelivr.net; frame-ancestors 'none';",
    "Referrer-Policy": "strict-origin",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains; preload",
  },
  "/app/": {
    "X-Robots-Tag": "none",
  },
};

interface AppInfo {
  env_name?: string;
  name?: string;
  description?: string;
}

const getAppInfo = async (apiDomain: string, envId: string): Promise<AppInfo> => {
  const response = await fetch(`https://${apiDomain}/apps/${envId}/summary`);

  if (!response.ok) {
    return {};
  }

  try {
    const body = await response.json();
    return body as AppInfo;
  } catch {
    return {};
  }
};

interface AppConfig {
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
  pwa_icon_monochrome_name?: string;
  pwa_icon_monochrome_type?: string;
  pwa_icon_monochrome_sizes?: string;
  pwa_icon_monochrome_maskable_name?: string;
  pwa_icon_monochrome_maskable_type?: string;
  pwa_icon_monochrome_maskable_sizes?: string;
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

const assetUrl = (apiDomain: string, envId: string, assetName: string): string =>
  `https://${apiDomain}/apps/${envId}/assets/${assetName}`;

const appPathRegex = new RegExp(`^/app/([^/]+)/`);
const manifestPathRegex = new RegExp(`^/app/([^/]+)/app.webmanifest$`);

const webManifestResponse = async (requestUrl: URL, env: Env): Promise<Response | undefined> => {
  const matches = manifestPathRegex.exec(requestUrl.pathname);

  if (!matches) {
    return undefined;
  }

  const envId = matches[1];
  const appInfo = await getAppInfo(env.API_DOMAIN, envId);
  const appConfig = await getAppConfig(env.API_DOMAIN, envId);

  const webManifest = {
    name: appInfo.name ?? "FanJam",
    short_name: appConfig.pwa_short_app_name ?? appInfo.name ?? "FanJam",
    description: appInfo.description,
    scope: `${requestUrl.origin}/app/${envId}/`,
    start_url: `${requestUrl.origin}/app/${envId}/`,
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
      {
        src:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_name !== undefined
            ? assetUrl(env.API_DOMAIN, envId, appConfig.pwa_icon_monochrome_name)
            : `${requestUrl.origin}/icons/icon-monochrome.png`,
        type:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_type !== undefined
            ? appConfig.pwa_icon_monochrome_type
            : "image/png",
        sizes:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_sizes !== undefined
            ? appConfig.pwa_icon_monochrome_sizes
            : undefined,
        purpose: "monochrome",
      },
      {
        src:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_maskable_name !== undefined
            ? assetUrl(env.API_DOMAIN, envId, appConfig.pwa_icon_monochrome_maskable_name)
            : `${requestUrl.origin}/icons/icon-monochrome-maskable.png`,
        type:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_maskable_type !== undefined
            ? appConfig.pwa_icon_monochrome_maskable_type
            : "image/png",
        sizes:
          appConfig.use_custom_icon && appConfig.pwa_icon_monochrome_maskable_sizes !== undefined
            ? appConfig.pwa_icon_monochrome_maskable_sizes
            : undefined,
        purpose: "monochrome maskable",
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

const injectWebManifestLink = (requestUrl: URL, response: Response): Response => {
  const matches = appPathRegex.exec(requestUrl.pathname);

  if (!matches) {
    return response;
  }

  const envId = matches[1];
  const webManifestUrl = `${requestUrl.origin}/app/${envId}/app.webmanifest`;

  return new HTMLRewriter()
    .on("head", {
      element(element: Element) {
        element.append(`<link rel="manifest" href="${webManifestUrl}" />`, { html: true });
      },
    })
    .transform(response);
};

const injectMetadata = async (requestUrl: URL, env: Env, response: Response): Promise<Response> => {
  const matches = appPathRegex.exec(requestUrl.pathname);

  if (!matches) {
    // This isn't an app page.
    //
    // Set the Umami tag to "home" for non-app pages so we can track traffic to
    // the homepage. The reason why we don't set this as the default `data-tag`
    // in the `index.html` is to prevent Playwright tests and invalid app IDs
    // from polluting our Umami metrics. Those requests will still be reported
    // to Umami, but they won't have a tag, so we can filter them out.
    return new HTMLRewriter()
      .on("head > script[src='/stats.js']", {
        element(element: Element) {
          element.setAttribute("data-tag", "home");
        },
      })
      .transform(response);
  }

  const envId = matches[1];
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

    // If this is the URL of a web manifest, generate it instead of forwarding
    // the request to the CDN.
    const webManifest = await webManifestResponse(requestUrl, env);

    if (webManifest) {
      return webManifest;
    }

    const response = await env.ASSETS.fetch(request);

    // Copy response headers from origin.
    const newHeaders = new Headers();
    for (const [key, value] of response.headers.entries()) {
      newHeaders.set(key, value);
    }

    let newResponse = new Response(response.body as ReadableStream<Uint8Array> | null, {
      status: response.status,
      statusText: response.statusText,
      headers: newHeaders,
    });

    // Add response headers.
    for (const [prefix, patternHeaders] of Object.entries(headerPatterns)) {
      if (requestUrl.pathname.startsWith(prefix)) {
        for (const [key, value] of Object.entries(patternHeaders)) {
          newResponse.headers.set(key, value);
        }
      }
    }

    // Replace the default metadata with user-configured instance-specific
    // metadata if this is a page in the app. This has performance implications
    // for page load time.
    if (env.INJECT_METADATA === "true") {
      newResponse = await injectMetadata(requestUrl, env, newResponse);
    }

    // Inject the web manifest URL into the response if this is a page in the app.
    newResponse = injectWebManifestLink(requestUrl, newResponse);

    return newResponse;
  },
};
