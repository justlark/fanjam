#
# `ttl = 1` means "automatic".
#

resource "cloudflare_dns_record" "noco_cname" {
  for_each = local.environments

  zone_id = data.cloudflare_zone.site.zone_id
  type    = "CNAME"
  name    = each.value.app_domain
  content = "${each.value.fly_app}.fly.dev"
  ttl     = 1
  proxied = false
}

resource "cloudflare_dns_record" "apex_txt_sl_verification" {
  zone_id = data.cloudflare_zone.site.zone_id
  type    = "TXT"
  name    = "@"
  content = "sl-verification=xfmceosgvrookthuefxniybjngiice"
  ttl     = 1
  proxied = false
}

resource "cloudflare_dns_record" "apex_mx" {
  for_each = {
    route1 = {
      value    = "mx1.simplelogin.co."
      priority = 10
    }
    route2 = {
      value    = "mx2.simplelogin.co."
      priority = 20
    }
  }

  zone_id  = data.cloudflare_zone.site.zone_id
  type     = "MX"
  name     = "@"
  content  = each.value.value
  priority = each.value.priority
  ttl      = 1
  proxied  = false
}

resource "cloudflare_dns_record" "apex_txt_spf" {
  zone_id = data.cloudflare_zone.site.zone_id
  type    = "TXT"
  name    = "@"
  content = "v=spf1 include:simplelogin.co ~all"
  ttl     = 1
  proxied = false
}

resource "cloudflare_dns_record" "apex_cname_dkim" {
  for_each = {
    record1 = {
      name  = "dkim._domainkey"
      value = "dkim._domainkey.simplelogin.co."
    }

    record2 = {
      name  = "dkim02._domainkey"
      value = "dkim02._domainkey.simplelogin.co."
    }

    record3 = {
      name  = "dkim03._domainkey"
      value = "dkim03._domainkey.simplelogin.co."
    }
  }

  zone_id = data.cloudflare_zone.site.zone_id
  type    = "CNAME"
  name    = each.value.name
  content = each.value.value
  ttl     = 1
  proxied = false
}

resource "cloudflare_dns_record" "apex_txt_dmarc" {
  zone_id = data.cloudflare_zone.site.zone_id
  type    = "TXT"
  name    = "_dmarc"
  content = "v=DMARC1; p=quarantine; pct=100; adkim=s; aspf=s"
  ttl     = 1
  proxied = false
}
