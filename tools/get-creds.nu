#!/usr/bin/env nu

def main [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let config_path = $repo_path | path join "config.yaml"
  let config = open $config_path

  let all_secrets = terraform -chdir=./infra/ output -json noco_secrets | from json
  let env_secrets = $all_secrets | get $env_name
  let admin_password = $env_secrets | get "NC_ADMIN_PASSWORD"

  let creds = {
    username: ($config | get "admin_email"),
    password: $admin_password,
  }

  echo $creds
}
