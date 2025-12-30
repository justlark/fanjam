# Not all of these env vars are secrets, but it's easier to combine them into
# one output.
output "noco_secrets" {
  value = {
    for env, config in local.environments : env => {
      NC_PUBLIC_URL = "https://${config.app_domain}.${data.cloudflare_zone.site.name}"

      # The `ssl=true` param is important.
      NC_DB = "pg://${neon_project.env[env].database_host_pooler}:5432?u=${neon_project.env[env].database_user}&p=${neon_project.env[env].database_password}&d=${neon_project.env[env].database_name}&ssl=true"

      # In my testing, with the data sets we're working with, NocoDB is just as
      # fast with Redis disabled. This could be because we're using a beefy
      # dedicated Postgres cluster per environment. Additionally, for unknown
      # reasons, when loading data into the database manually, such as when
      # seeding demo data, the performance tanks to the point of being unusable
      # unless we disable Redis caching.
      #
      # So, because managed Redis is expensive, we might as well disable it.
      # However, we're keeping the terraform for managing Redis around, but
      # commented out, in case we need it again in the future.
      NC_DISABLE_CACHE = "true"

      # Ideally we generate a separate access key for each environment, scoped
      # to that environment's bucket. However, Cloudflare does not provide an API
      # for generating S3-compatible access keys. So instead we rely on a single
      # account-scoped access key that's stored in the Terraform state and shared
      # between environments.
      NC_S3_ACCESS_KEY    = var.cloudflare_r2_access_key_id
      NC_S3_ACCESS_SECRET = var.cloudflare_r2_secret_access_key
      NC_S3_ENDPOINT      = "https://${local.globals.cloudflare_account_id}.r2.cloudflarestorage.com/sparklefish-noco-${env}"
      NC_S3_BUCKET_NAME   = "sparklefish-noco-${env}"
      NC_S3_REGION        = "auto"

      # Signed file URLs are valid for 12 hours.
      NC_ATTACHMENT_EXPIRE_SECONDS = "43200"

      # Users are never given this password. We need it to generate an API
      # token for the sparklefish backend. After that, we shouldn't need it
      # again.
      NC_ADMIN_PASSWORD = random_password.noco_admin_password[env].result
      NC_ADMIN_EMAIL    = local.globals.admin_email

      # There is additionally a `NC_SMTP_SECURE` env var that defaults to
      # `false`, but I wasn't able to get it working with MailerSend.
      NC_SMTP_FROM                = local.globals.notifications_email
      NC_SMTP_HOST                = var.smtp_host
      NC_SMTP_PORT                = var.smtp_port
      NC_SMTP_USERNAME            = var.smtp_username
      NC_SMTP_PASSWORD            = var.smtp_password
      NC_SMTP_REJECT_UNAUTHORIZED = "true"

      # This is important so that different NocoDB instances can share a Redis
      # database, saving us on infra costs.
      NC_CACHE_PREFIX = "sparklefish:env:${env}:noco"

      NC_INVITE_ONLY_SIGNUP   = "true"
      NC_DISABLE_SUPPORT_CHAT = "true"
      NC_DISABLE_TELE         = "true"
    }
  }
  sensitive = true
}

output "worker_admin_api_tokens" {
  value = {
    for stage, _ in local.stages : stage => random_bytes.worker_admin_api_token[stage].base64
  }
  sensitive = true
}

output "graphile_migrate" {
  value = {
    for env in keys(local.environments) : env => {
      DATABASE_URL        = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host}/${neon_database.sparklefish[env].name}?sslmode=require"
      SHADOW_DATABASE_URL = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host}/${neon_database.sparklefish_shadow[env].name}?sslmode=require"
      ROOT_DATABASE_URL   = "postgres://${neon_project.env[env].database_user}:${neon_project.env[env].database_password}@${neon_project.env[env].database_host}/${neon_project.env[env].database_name}?sslmode=require"
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

output "stages" {
  value = {
    for name, config in local.stages : name => {
      api_url = "https://${config.api_host}",
    }
  }
}
