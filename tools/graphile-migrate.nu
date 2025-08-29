#!/usr/bin/env nu

source ./config.nu

def --wrapped main [env_name: string, ...rest] {
  let env_vars = tofu -chdir=./infra/ output -json graphile_migrate | from json | get $env_name
  load-env $env_vars
  npx graphile-migrate --config ./migrations/.gmrc ...$rest
}
