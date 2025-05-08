data "cloudflare_zone" "site" {
  name = "fanjam.live"
}

resource "cloudflare_zone_dnssec" "site" {
  zone_id = data.cloudflare_zone.site.zone_id
}

resource "cloudflare_zone_settings_override" "site" {
  zone_id = data.cloudflare_zone.site.zone_id

  settings {
    # This is necessary to avoid a redirect loop for apps hosted on Fly.io.
    ssl = "strict"

    security_header {
      enabled            = true
      preload            = true
      max_age            = 31536000
      include_subdomains = true
      nosniff            = true
    }
  }
}
