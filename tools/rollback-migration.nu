#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name

  admin-api post $env_config.stage $"/admin/env/($env_name)/backups/restore" {
    type: "migration",
  }
}
