#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let config = get-global-config

  let all_secrets = with-env (get-tofu-env) {
    tofu -chdir=./infra/ output -json noco_secrets | from json
  }
  let env_secrets = $all_secrets | get $env_name
  let admin_password = $env_secrets | get "NC_ADMIN_PASSWORD"

  let creds = {
    username: ($config.admin_email),
    password: $admin_password,
  }

  echo $creds
}
