resource "neon_project" "env" {
  for_each = local.environments
  name     = each.key
  org_id   = var.neon_org_id

  branch {
    name          = "prod"
    database_name = "noco"
    role_name     = "sparklefish"
  }
}
