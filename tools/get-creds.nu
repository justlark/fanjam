#!/usr/bin/env nu

def main [env_name: string] {
  let all_secrets = terraform -chdir=./infra/ noco_output -json secrets | from json
  let env_secrets = $all_secrets | get $env_name
  let admin_password = $env_secrets | get "NC_ADMIN_PASSWORD"
  let creds = {
    username: "system@fanjam.live",
    password: $admin_password,
  }

  echo $creds
}
