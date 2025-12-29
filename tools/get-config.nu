#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name

  let spec = admin-api get $env_config.stage "/admin/config-spec"
  let config = admin-api get $env_config.stage $"/admin/env/($env_name)/config"

  $config | items {|key, value| { $key: (if ($spec | where key == $key | first | get sensitive) { "[REDACTED]" } else { $value }) }} | into record
}
