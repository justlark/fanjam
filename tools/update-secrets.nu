#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let all_secrets = terraform -chdir=./infra/ output -json noco_secrets | from json
  let env_secrets = $all_secrets | get $env_name
  let secret_pairs = $env_secrets | transpose name value | each {|secret| $"($secret.name)=($secret.value)" }

  let env_config = get-env-config $env_name
  let fly_app = $env_config | get fly_app

  fly secrets set --app $fly_app ...$secret_pairs
}
