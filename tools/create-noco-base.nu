#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let title = input "Enter the user-facing name of the environment: "
  let email = input "Enter the email address of the initial user: "
  let env_config = get-env-config $env_name

  admin-api post $env_config.stage $"/bases/($env_name)" {
    title: $title,
    email: $email,
  }
}
