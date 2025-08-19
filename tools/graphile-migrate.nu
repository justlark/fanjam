#!/usr/bin/env nu

source ./config.nu

def main [env_name: string, ...rest] {
  let env_vars = terraform -chdir=./infra/ output -json graphile_migrate | from json
  load-env $env_vars
  npx graphile-migrate ...$rest
}
