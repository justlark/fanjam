resource "cloudflare_ruleset" "etags" {
  zone_id = data.cloudflare_zone.site.zone_id
  name    = "strong_etags"
  kind    = "zone"
  phase   = "http_request_cache_settings"

  rules {
    action = "set_cache_settings"

    action_parameters {
      respect_strong_etags = true
    }

    expression  = "(http.host in {\"api.${data.cloudflare_zone.site.name}\" \"api-test.${data.cloudflare_zone.site.name}\"})"
    description = "Enable Strong ETags"
    enabled     = true
  }
}

resource "cloudflare_ruleset" "noco_brotli" {
  zone_id = data.cloudflare_zone.site.zone_id
  name    = "noco_brotli_compression"
  kind    = "zone"
  phase   = "http_response_compression"

  rules {
    description = "Prefer Brotli compression"
    expression  = true
    action      = "compress_response"

    action_parameters {
      algorithms {
        name = "brotli"
      }
    }
  }
}
