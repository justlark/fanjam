locals {
  globals = yamldecode(file("${path.module}/config.yaml"))

  stages = {
    for stage in local.globals.stages : stage.name => {
      api_host = stage.name == "prod" ? "api.${data.cloudflare_zone.site.name}" : "api-${stage.name}.${data.cloudflare_zone.site.name}"
    }
  }

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
