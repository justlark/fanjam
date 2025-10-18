import type { Request, Env } from "@cloudflare/workers-types";

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url);

    // Proxy API endpoints to the Matrix server.
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

    if (url.pathname === "/dashboard" || url.pathname.startsWith("/dashboard/")) {
      const dashboardUrl = new URL(request.url);
      dashboardUrl.pathname = dashboardUrl.pathname.replace("/dashboard", "");
      return Response.redirect(dashboardUrl, 302);
    }

    return await env.ASSETS.fetch(request);
  }
}
