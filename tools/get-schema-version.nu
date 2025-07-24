#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name

  admin-api get $env_config.stage $"/migrations/($env_name)/current"
}
