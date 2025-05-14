locals {
  neon_default_branch_name = "prod"
}

resource "neon_project" "env" {
  for_each = local.environments
  name     = each.key
  org_id   = var.neon_org_id

  branch {
    name          = local.neon_default_branch_name
    database_name = "noco"
    role_name     = "sparklefish"
  }
}
