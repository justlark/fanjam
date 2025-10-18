import type { Request, Env, Fetcher } from "@cloudflare/workers-types";

interface Env {
  ASSETS: Fetcher;
  DASHBOARD_PATH: string;
}

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url);

    if (url.pathname.startsWith("/api/")) {
      const requestUrl = new URL(request.url);
      const segments = requestUrl.hostname.split(".");
      requestUrl.hostname = `${segments[0]}.noco.${segments.slice(1).join(".")}`;

      return await fetch(requestUrl, {
        method: request.method,
        headers: request.headers,
        body: request.body,
      });
    }

    if (url.pathname === "/") {
      const dashboardUrl = new URL(request.url);
      dashboardUrl.pathname = env.DASHBOARD_PATH;
      return Response.redirect(dashboardUrl, 302);
    }

    return await env.ASSETS.fetch(request);
  }
}
