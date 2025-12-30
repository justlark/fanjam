resource "cloudflare_workers_kv_namespace" "sparklefish" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  title      = "sparklefish-${each.key}"
}

moved {
  from = cloudflare_workers_kv_namespace.sparklefish_prod
  to   = cloudflare_workers_kv_namespace.sparklefish["prod"]
}

moved {
  from = cloudflare_workers_kv_namespace.sparklefish_test
  to   = cloudflare_workers_kv_namespace.sparklefish["test"]
}
