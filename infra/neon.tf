locals {
  neon_default_branch_name = "main"
}

// TODO: Once this issue in the Neon Terraform provider is addressed, we can
// protect the default branch so it cannot be deleted.
//
// https://github.com/kislerdm/terraform-provider-neon/issues/153

resource "neon_project" "env" {
  for_each  = local.environments
  name      = each.key
  org_id    = var.neon_org_id
  region_id = each.value.neon_region

  branch {
    name          = local.neon_default_branch_name
    database_name = "noco"
    role_name     = "sparklefish"
  }

  default_endpoint_settings {
    autoscaling_limit_min_cu = 0.25
    autoscaling_limit_max_cu = 0.5
    suspend_timeout_seconds  = 300
  }
}

resource "neon_database" "sparklefish" {
  for_each   = local.environments
  project_id = neon_project.env[each.key].id
  branch_id  = neon_project.env[each.key].default_branch_id
  name       = "sparklefish"
  owner_name = neon_project.env[each.key].database_user
}

resource "neon_database" "sparklefish_shadow" {
  for_each   = local.environments
  project_id = neon_project.env[each.key].id
  branch_id  = neon_project.env[each.key].default_branch_id
  name       = "sparklefish-shadow"
  owner_name = neon_project.env[each.key].database_user
}
