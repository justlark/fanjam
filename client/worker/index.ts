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
      "default-src 'self'; connect-src 'self' https://api.fanjam.live https://api-test.fanjam.live https://fonts.googleapis.com https://fonts.gstatic.com https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://cdn.jsdelivr.net; font-src 'self' https://fonts.gstatic.com https://cdn.jsdelivr.net; manifest-src 'self' data:; frame-ancestors 'none';",
    "Referrer-Policy": "strict-origin",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains; preload",
  },
  "/app/": {
    "X-Robots-Tag": "none",
  },
};

interface AppInfo {
  name?: string;
  description?: string;
}

const getAppInfo = async (apiDomain: string, envId: string): Promise<AppInfo> => {
  const response = await fetch(`https://${apiDomain}/apps/${envId}/summary`);

  if (!response.ok) {
    console.warn(
      `Failed to fetch app summary for app ${envId}: ${response.status.toString()} ${response.statusText}`,
    );
    return {};
  }

  try {
    const body = await response.json();
    return body as AppInfo;
  } catch {
    console.warn(
      `Failed to deserialize app summary for app ${envId}: ${response.status.toString()} ${response.statusText}`,
    );
    return {};
  }
};

const appPathRegex = new RegExp(`^/app/([^/]+)/`);
const manifestPathRegex = new RegExp(`^/app/([^/]+)/app.webmanifest$`);

const webManifestResponse = async (requestUrl: URL, env: Env): Promise<Response | undefined> => {
  const matches = manifestPathRegex.exec(requestUrl.pathname);

  if (!matches) {
    return undefined;
  }

  const envId = matches[1];
  const appInfo = await getAppInfo(env.API_DOMAIN, envId);

  const webManifest = {
    name: appInfo.name ?? "FanJam",
    description: appInfo.description,
    scope: `${requestUrl.origin}/app/${envId}/`,
    start_url: `${requestUrl.origin}/app/${envId}/`,
    display: "standalone",
    icons: [
      {
        src: `${requestUrl.origin}/icons/icon.png`,
        type: "image/png",
      },
      {
        src: `${requestUrl.origin}/icons/icon-maskable.png`,
        type: "image/png",
        purpose: "maskable",
      },
      {
        src: `${requestUrl.origin}/icons/icon-monochrome.png`,
        type: "image/png",
        purpose: "monochrome",
      },
      {
        src: `${requestUrl.origin}/icons/icon-monochrome-maskable.png`,
        type: "image/png",
        purpose: "monochrome maskable",
      },
    ],
    shortcuts: [
      {
        name: "Schedule",
        url: `${requestUrl.origin}/app/${envId}/schedule`,
        icons: [
          {
            src: `${requestUrl.origin}/icons/shortcut-schedule-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-schedule-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-schedule-monochrome-maskable.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome maskable",
          },
        ],
      },
      {
        name: "Program",
        url: `${requestUrl.origin}/app/${envId}/program`,
        icons: [
          {
            src: `${requestUrl.origin}/icons/shortcut-program-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-program-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-program-monochrome-maskable.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome maskable",
          },
        ],
      },
      {
        name: "Info",
        url: `${requestUrl.origin}/app/${envId}/info`,
        icons: [
          {
            src: `${requestUrl.origin}/icons/shortcut-info-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-info-monochrome.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome",
          },
          {
            src: `${requestUrl.origin}/icons/shortcut-info-monochrome-maskable.png`,
            sizes: "96x96",
            type: "image/png",
            purpose: "monochrome maskable",
          },
        ],
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
    return response;
  }

  const envId = matches[1];
  const appInfo = await getAppInfo(env.API_DOMAIN, envId);

  return new HTMLRewriter()
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
    .transform(response);
};

export default {
  async fetch(request: Request, env: Env) {
    const requestUrl = new URL(request.url);

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
