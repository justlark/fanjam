#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let config = get-global-config
  let env_config = get-env-config $env_name
  let app_subdomain = $env_config | get app_domain

  let existing_certs = fly --config $"./infra/environments/($env_name)/fly.yaml" certs list --json | from json

  if ($existing_certs | length) > 0 {
    print $"Certificates already exist for ($app_subdomain).($config.app_base_domain)."
    return
  }

  fly --config $"./infra/environments/($env_name)/fly.yaml" certs add $"($app_subdomain).($config.app_base_domain)"
}
