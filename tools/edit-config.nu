#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name

  let config = admin-api get $env_config.stage $"/admin/env/($env_name)/config"

  let temp_file_path = mktemp "fanjam-config-XXXXXX.json"
  $config | to json | save --force $temp_file_path

  let editor = $env | get --optional "VISUAL" | default ($env | get --optional "EDITOR") | default "nano"
  run-external $editor $temp_file_path
  let edited_config = open $temp_file_path

  admin-api put $env_config.stage $"/admin/env/($env_name)/config" ($edited_config | to json --raw)

  rm --force $temp_file_path

  admin-api get $env_config.stage $"/admin/env/($env_name)/config"
}
