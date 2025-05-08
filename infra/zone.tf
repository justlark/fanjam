data "cloudflare_zone" "site" {
  zone_id = "abcb5a999500d35cf397d83f2d66363d"
}

resource "cloudflare_zone_dnssec" "site" {
  zone_id = data.cloudflare_zone.site.zone_id
}

# This is necessary to avoid a redirect loop for apps hosted on Fly.io.
resource "cloudflare_zone_setting" "ssl_mode" {
  zone_id    = data.cloudflare_zone.site.zone_id
  setting_id = "ssl"
  value      = "strict"
}
