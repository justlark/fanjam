data "cloudflare_zone" "site" {
  zone_id = "abcb5a999500d35cf397d83f2d66363d"
}

resource "cloudflare_zone_dnssec" "site" {
  zone_id = data.cloudflare_zone.site.zone_id
}

# This is necessary to avoid a redirect loop for apps hosted on Fly.io.
# However, after this resource is applied, subsequent applies cause the
# Cloudflare provider to return an error. Because destroying this resource
# doesn't actually unset the value in Cloudflare, we can comment this out until
# we figure out what's going on.

# resource "cloudflare_zone_setting" "ssl_mode" {
#   zone_id    = data.cloudflare_zone.site.zone_id
#   setting_id = "ssl"
#   value      = "strict"
# }
