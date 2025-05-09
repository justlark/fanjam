output "secrets" {
  value = {
    for env in keys(local.environments) : env => {
      # The `ssl=true` param is important.
      NC_DB = "pg://${neon_project.env[env].database_host_pooler}:5432?u=${neon_project.env[env].database_user}&p=${neon_project.env[env].database_password}&d=${neon_project.env[env].database_name}&ssl=true"

      # The extra 's' in the `rediss://` scheme is important.
      NC_REDIS_URL = "rediss://default:${upstash_redis_database.noco[env].password}@${upstash_redis_database.noco[env].endpoint}:${upstash_redis_database.noco[env].port}"

      # Ideally we generate a separate access key for each environment, scoped
      # to that environment's bucket. However, Cloudflare does not provide an API
      # for generating S3-compatible access keys. So instead we rely on a single
      # account-scoped access key that's stored in the Terraform state and shared
      # between environments.
      NC_S3_ACCESS_KEY    = var.cloudflare_r2_access_key_id
      NC_S3_ACCESS_SECRET = var.cloudflare_r2_secret_access_key

      # Users are never given this password. We need it to generate an API
      # token for the sparklefish backend. After that, we shouldn't need it
      # again.
      NC_ADMIN_PASSWORD = random_password.noco_admin_password[env].result
    }
  }
  sensitive = true
}
