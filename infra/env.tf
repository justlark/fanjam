locals {
  globals = yamldecode(file("${path.module}/config.yaml"))

  stages = toset([for stage in local.globals.stages : stage.name])

  environments = {
    for env_file in fileset("${path.module}/environments", "*/env.yaml") : dirname(env_file) => yamldecode(file("${path.module}/environments/${env_file}"))
  }
}

resource "random_password" "noco_admin_password" {
  for_each = local.environments

  keepers = {
    counter = each.value.system_password_counter
  }

  length  = 20
  special = false
}
