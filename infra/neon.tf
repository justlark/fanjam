locals {
  neon_default_branch_name = "main"
}

// TODO: Once this issue in the Neon Terraform provider is addressed, we can
// protect the default branch so it cannot be deleted.
//
// https://github.com/kislerdm/terraform-provider-neon/issues/153

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
