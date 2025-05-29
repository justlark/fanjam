#!/usr/bin/env nu

def main [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let config_path = $repo_path | path join "config.yaml"
  let config = open $config_path

  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  let app_subdomain = open $env_file | get app_domain

  let existing_certs = fly --config $"./infra/environments/($env_name)/fly.yaml" certs list --json | from json

  if ($existing_certs | length) > 0 {
    print $"Certificates already exist for ($app_subdomain).($config.app_base_domain)."
    return
  }

  fly --config $"./infra/environments/($env_name)/fly.yaml" certs add $"($app_subdomain).($config.app_base_domain)"
}
