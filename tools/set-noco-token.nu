#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let api_token = input "Enter the NocoDB API token: "
  let env_config = get-env-config $env_name

  admin-api put $env_config.stage $"/admin/env/($env_name)/tokens" {
    token: $api_token,
  }
}
