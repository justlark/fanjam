resource "cloudflare_r2_bucket" "noco" {
  for_each = local.environments

  account_id = var.cloudflare_account_id
  name       = "sparklefish-noco-${each.key}"
}

resource "cloudflare_r2_bucket" "assets_prod" {
  account_id = var.cloudflare_account_id
  name       = "sparklefish-assets-prod"
}

resource "cloudflare_r2_bucket" "assets_test" {
  account_id = var.cloudflare_account_id
  name       = "sparklefish-assets-test"
}

resource "cloudflare_r2_bucket" "static_prod" {
  account_id = var.cloudflare_account_id
  name       = "sparklefish-static-prod"
}


resource "cloudflare_r2_bucket" "static_test" {
  account_id = var.cloudflare_account_id
  name       = "sparklefish-static-test"
}

// TODO: Manage custom domains for R2 buckets with Tofu. This is only available
// on v5.x of the Cloudflare provider.
