locals {
  environments = {
    for env_file in fileset("${path.module}/environments", "*/env.yaml") : dirname(env_file) => yamldecode(file("${path.module}/environments/${env_file}"))
  }
}
