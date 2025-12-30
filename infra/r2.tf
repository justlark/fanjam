resource "cloudflare_r2_bucket" "noco" {
  for_each = local.environments

  account_id = var.cloudflare_account_id
  name       = "sparklefish-noco-${each.key}"
}

resource "cloudflare_r2_bucket" "assets" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  name       = "sparklefish-assets-${each.key}"
}

moved {
  from = cloudflare_r2_bucket.assets_prod
  to   = cloudflare_r2_bucket.assets["prod"]
}

moved {
  from = cloudflare_r2_bucket.assets_test
  to   = cloudflare_r2_bucket.assets["test"]
}

resource "cloudflare_r2_bucket" "static" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  name       = "sparklefish-static-${each.key}"
}

moved {
  from = cloudflare_r2_bucket.static_prod
  to   = cloudflare_r2_bucket.static["prod"]
}

moved {
  from = cloudflare_r2_bucket.static_test
  to   = cloudflare_r2_bucket.static["test"]
}

// TODO: Manage custom domains for R2 buckets with Tofu. This is only available
// on v5.x of the Cloudflare provider.
