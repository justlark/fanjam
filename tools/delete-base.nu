#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  let confirmed_env_name = input "Enter the name of the environment to confirm: "

  if $env_name != $confirmed_env_name {
    print --stderr "Environment name does not match. Aborting."
    exit 1
  }

  admin-api delete $stage_name $"/bases/($env_name)"
}
