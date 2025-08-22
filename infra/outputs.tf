output "noco_secrets" {
  value = {
    for env in keys(local.environments) : env => {
      # The `ssl=true` param is important.
      NC_DB = "pg://${neon_project.env[env].database_host_pooler}:5432?u=${neon_project.env[env].database_user}&p=${neon_project.env[env].database_password}&d=${neon_project.env[env].database_name}&ssl=true"

      # The extra 's' in the `rediss://` scheme is important.
      NC_REDIS_URL = "rediss://default:${upstash_redis_database.noco.password}@${upstash_redis_database.noco.endpoint}:${upstash_redis_database.noco.port}"

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

      NC_SMTP_FROM                = "notifications@fanjam.live"
      NC_SMTP_HOST                = var.smtp_host
      NC_SMTP_PORT                = var.smtp_port
      NC_SMTP_USERNAME            = var.smtp_username
      NC_SMTP_PASSWORD            = var.smtp_password
      NC_SMTP_REJECT_UNAUTHORIZED = "true"
    }
  }
  sensitive = true
}

output "worker_admin_api_tokens" {
  value = {
    for stage in local.stages : stage => random_bytes.worker_admin_api_token[stage].base64
  }
  sensitive = true
}

output "graphile_migrate" {
  value = {
    for env in keys(local.environments) : env => {
      DATABASE_URL        = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host_pooler}/${neon_database.sparklefish[env].name}?sslmode=require"
      SHADOW_DATABASE_URL = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host_pooler}/${neon_database.sparklefish_shadow[env].name}?sslmode=require"
      ROOT_DATABASE_URL   = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host_pooler}/${neon_project.env[env].database_name}?sslmode=require"
    }
  }
  sensitive = true
}

output "env_config" {
  value = {
    for env in keys(local.environments) : env => {
      config_db_host     = neon_project.env[env].database_host_pooler,
      config_db_port     = 5432,
      config_db_name     = neon_database.sparklefish[env].name,
      config_db_user     = neon_project.env[env].database_user,
      config_db_password = neon_project.env[env].database_password,
    }
  }
  sensitive = true
}
