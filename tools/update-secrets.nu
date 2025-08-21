#!/usr/bin/env nu

source ./config.nu
source ./http.nu

def push_fly_secrets [env_name: string] {
  let noco_secrets = terraform -chdir=./infra/ output -json noco_secrets | from json | get $env_name
  let noco_secret_pairs = $noco_secrets | transpose name value | each {|secret| $"($secret.name)=($secret.value)" }

  let env_config = get-env-config $env_name
  let fly_app = $env_config | get fly_app

  fly secrets set --app $fly_app ...$noco_secret_pairs
}

def push_env_secrets [env_name: string] {
  let new_env_secrets = terraform -chdir=./infra/ output -json env_config | from json | get $env_name
  let env_config = get-env-config $env_name

  let current_env_secrets = admin-api get $env_config.stage $"/admin/env/($env_name)/config"
  let patched_env_secrets = $current_env_secrets | merge $new_env_secrets

  admin-api put $env_config.stage $"/admin/env/($env_name)/config" ($patched_env_secrets | to json --raw)
}

def main [env_name: string] {
  push_fly_secrets $env_name
  push_env_secrets $env_name
}
