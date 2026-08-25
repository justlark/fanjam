resource "cloudflare_queue" "push" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  name       = "sparklefish-push-${each.key}"
}

resource "cloudflare_queue" "push_dlq" {
  for_each   = local.stages
  account_id = var.cloudflare_account_id
  name       = "sparklefish-push-dlq-${each.key}"
}
