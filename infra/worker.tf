resource "random_bytes" "worker_admin_api_token" {
  for_each = local.stages

  length = 32
}

resource "cloudflare_workers_secret" "neon_api_token" {
  for_each = local.stages

  account_id  = var.cloudflare_account_id
  name        = "NEON_API_TOKEN"
  script_name = "sparklefish-server-${each.key}"
  secret_text = var.neon_api_token
}

# This isn't really a secret, but it's convenient to pass it here.
resource "cloudflare_workers_secret" "neon_org_id" {
  for_each = local.stages

  account_id  = var.cloudflare_account_id
  name        = "NEON_ORG_ID"
  script_name = "sparklefish-server-${each.key}"
  secret_text = var.neon_org_id
}

resource "cloudflare_workers_secret" "admin_api_token" {
  for_each = local.stages

  account_id  = var.cloudflare_account_id
  name        = "ADMIN_API_TOKEN"
  script_name = "sparklefish-server-${each.key}"
  secret_text = random_bytes.worker_admin_api_token[each.key].base64
}
