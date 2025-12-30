resource "cloudflare_workers_kv_namespace" "sparklefish" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  title      = "sparklefish-${each.key}"
}
