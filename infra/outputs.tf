output "secrets" {
  value = {
    for env, config in local.environments : config.fly_app => {
      # The `ssl=true` param is important.
      NC_DB = "pg://${neon_project.env[env].database_host_pooler}:5432?u=${neon_project.env[env].database_user}&p=${neon_project.env[env].database_password}&d=${neon_project.env[env].database_name}&ssl=true"

      # The extra 's' in the `rediss://` scheme is important.
      NC_REDIS_URL = "rediss://default:${upstash_redis_database.noco[env].password}@${upstash_redis_database.noco[env].endpoint}:${upstash_redis_database.noco[env].port}"
    }
  }
  sensitive = true
}
