#!/usr/bin/env nu

const app_base_domain = "fanjam.live"

def generate_fly_config [app: string, url: string] {
$"app = '($app)'
primary_region = 'bos'

[build]
  image = 'ghcr.io/justlark/nocodb:latest'

[env]
  NC_PUBLIC_URL = '($url)'

[http_service]
  internal_port = 8080
  force_https = true
  auto_stop_machines = 'stop'
  auto_start_machines = true
  min_machines_running = 0
  processes = ['app']

[[vm]]
  memory = '1gb'
  cpu_kind = 'shared'
  cpus = 1
"
}

def main [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let env_path = $repo_path | path join "infra" "environments" $env_name

  let env_file = $env_path | path join "env.yaml"
  let fly_file = $env_path | path join "fly.toml"

  let fly_app_name = $"sparklefish-noco-($env_name)"
  let app_url = $"https://($env_name).($app_base_domain)"

  let env_config = {
    fly_app: $fly_app_name
    app_domain: $env_name
  }

  mkdir $env_path

  $env_config | to yaml | save $env_file
  generate_fly_config $fly_app_name $app_url | save $fly_file
}
