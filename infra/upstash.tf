resource "upstash_redis_database" "noco" {
  for_each = local.environments

  database_name  = "noco-${each.key}"
  region         = "global"
  primary_region = "us-east-1"
  tls            = true
}
