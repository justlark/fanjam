locals {
  environments = {
    for env_file in fileset("${path.module}/environments", "*/env.yaml") : dirname(env_file) => yamldecode(file("${path.module}/environments/${env_file}"))
  }
}

resource "random_password" "noco_admin_password" {
  for_each = local.environments

  length  = 24
  special = false
}
