resource "cloudflare_r2_bucket" "noco" {
  for_each = local.environments

  account_id = var.cloudflare_account_id
  name       = "sparklefish-noco-${each.key}"
}

resource "cloudflare_r2_bucket" "assets" {
  account_id = var.cloudflare_account_id
  name       = "sparklefish-assets"
}
