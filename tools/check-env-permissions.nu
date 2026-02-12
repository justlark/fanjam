#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name
  check-advisory-permissions $env_config.stage
}
