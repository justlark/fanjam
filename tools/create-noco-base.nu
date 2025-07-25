#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let title = input "Enter the user-facing name of the environment: "
  let email = input "Enter the email address of the initial user: "
  let env_config = get-env-config $env_name

  admin-api post $env_config.stage $"/admin/env/($env_name)/bases" {
    title: $title,
    email: $email,
  }
}
