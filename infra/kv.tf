resource "cloudflare_workers_kv_namespace" "sparklefish_test" {
  account_id = var.cloudflare_account_id
  title      = "sparklefish-test"
}

resource "cloudflare_workers_kv_namespace" "sparklefish_prod" {
  account_id = var.cloudflare_account_id
  title      = "sparklefish-prod"
}
