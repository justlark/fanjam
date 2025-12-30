locals {
  api_subdomains = [for stage in local.stages : stage == "prod" ? "api" : "api-${stage}"]
  api_hosts      = [for subdomain in local.api_subdomains : "${subdomain}.${data.cloudflare_zone.site.name}"]
}

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

    expression  = "(http.host in {${join(" ", [for api_host in local.api_hosts : "\"${api_host}\""])}})"
    description = "Enable Strong ETags"
    enabled     = true
  }
}
