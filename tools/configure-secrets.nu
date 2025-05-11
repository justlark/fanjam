#!/usr/bin/env nu

def main [env_name: string] {
  let all_secrets = terraform -chdir=./infra/ output -json noco_secrets | from json
  let env_secrets = $all_secrets | get $env_name
  let secret_pairs = $env_secrets | transpose name value | each {|secret| $"($secret.name)=($secret.value)" }

  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  let fly_app = open $env_file | get fly_app

  fly secrets set --app $fly_app ...$secret_pairs
}
