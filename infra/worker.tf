locals {
  # Increment this to roll the worker admin API token.
  admin_api_token_counter = 1
}

resource "random_bytes" "worker_admin_api_token" {
  for_each = local.stages

  keepers = {
    counter = local.admin_api_token_counter
  }

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

# This isn't really a secret, but it's convenient to pass it here.
resource "cloudflare_workers_secret" "neon_default_branch_name" {
  for_each = local.stages

  account_id  = var.cloudflare_account_id
  name        = "NEON_DEFAULT_BRANCH_NAME"
  script_name = "sparklefish-server-${each.key}"
  secret_text = local.neon_default_branch_name
}

resource "cloudflare_workers_secret" "admin_api_token" {
  for_each = local.stages

  account_id  = var.cloudflare_account_id
  name        = "ADMIN_API_TOKEN"
  script_name = "sparklefish-server-${each.key}"
  secret_text = random_bytes.worker_admin_api_token[each.key].base64
}

# resource "cloudflare_workers_secret" "upstash_endpoint" {
#   for_each = local.stages
#
#   account_id  = var.cloudflare_account_id
#   name        = "UPSTASH_ENDPOINT"
#   script_name = "sparklefish-server-${each.key}"
#   secret_text = "https://${upstash_redis_database.noco.endpoint}"
# }

# resource "cloudflare_workers_secret" "upstash_api_token" {
#   for_each = local.stages
#
#   account_id  = var.cloudflare_account_id
#   name        = "UPSTASH_API_TOKEN"
#   script_name = "sparklefish-server-${each.key}"
#   secret_text = upstash_redis_database.noco.rest_token
# }
