#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let config = get-global-config
  let env_config = get-env-config $env_name
  let fly_app = $env_config | get fly_app

  let existing_apps = fly apps list --org $config.fly_org --json | from json

  if ($existing_apps | any {|app| $app.ID == $fly_app}) {
    print $"Fly app ($fly_app) already exists."
    return
  }

  fly apps create --org $config.fly_org --name $fly_app
}
