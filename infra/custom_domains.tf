resource "cloudflare_custom_hostname" "client" {
  for_each = local.custom_client_domains

  zone_id  = data.cloudflare_zone.site.zone_id
  hostname = each.value.domain

  ssl {
    method = "http"
    type   = "dv"

    settings {
      min_tls_version = "1.2"
      http2           = "on"
    }
  }
}

resource "cloudflare_custom_hostname_fallback_origin" "client" {
  zone_id = data.cloudflare_zone.site.zone_id
  origin  = data.cloudflare_zone.site.name
}

resource "cloudflare_workers_route" "client_custom_domain" {
  for_each = local.custom_client_domains

  zone_id     = data.cloudflare_zone.site.zone_id
  pattern     = "${each.value.domain}/*"
  script_name = "sparklefish-client-${each.value.stage}"
  depends_on  = [cloudflare_custom_hostname.client]
}
