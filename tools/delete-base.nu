#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let confirmed_env_name = input "Enter the name of the environment to confirm: "

  if $env_name != $confirmed_env_name {
    print --stderr "Environment name does not match. Aborting."
    exit 1
  }

  let env_config = get-env-config $env_name

  admin-api delete $env_config.stage $"/bases/($env_name)"
}
