#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [env_name: string] {
  let env_config = get-env-config $env_name

  let config = admin-api get $env_config.stage $"/admin/env/($env_name)/config"
  let spec_keys = admin-api get $env_config.stage "/admin/config-spec" | get key

  # Drop any keys the server returns that aren't in the config spec. They're
  # generated values that cannot be edited interactively.
  let editable_columns = $config | columns | where {|key| $key in $spec_keys}
  let editable_config = $config | select ...$editable_columns

  let temp_file_path = mktemp "fanjam-config-XXXXXX.json"
  $editable_config | to json | save --force $temp_file_path

  let editor = $env | get --optional "VISUAL" | default ($env | get --optional "EDITOR") | default "nano"
  run-external $editor $temp_file_path
  let edited_config = open $temp_file_path

  admin-api put $env_config.stage $"/admin/env/($env_name)/config" ($edited_config | to json --raw)

  rm --force $temp_file_path

  nu ($env.FILE_PWD | path join "get-config.nu") $env_name
}
