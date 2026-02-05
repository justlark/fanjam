#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string, slug: string] {
  let env_config = get-env-config $env_name

  admin-api delete $env_config.stage $"/admin/aliases/($slug)"
}
