resource "random_password" "load_test_secret" {
  length           = 32
  special          = false
}

resource "cloudflare_ruleset" "load_test_bypass" {
  zone_id = data.cloudflare_zone.site.zone_id
  name    = "Load Test Bypass"
  kind    = "zone"
  phase   = "http_request_firewall_custom"

  rules {
    description = "Bypass security products for authenticated load tests"
    enabled     = true

    expression = format(
      "(http.request.headers[\"x-load-test-secret\"][0] eq \"%s\")",
      random_password.load_test_secret.result
    )

    action = "skip"

    action_parameters {
      phases = [
        "http_ratelimit",
        "http_request_firewall_managed",
        "http_request_sbfm",
      ]
    }
  }
}
