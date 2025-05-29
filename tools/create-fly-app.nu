#!/usr/bin/env nu

def main [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let config_path = $repo_path | path join "config.yaml"
  let config = open $config_path

  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  let fly_app = open $env_file | get fly_app

  let existing_apps = fly apps list --org $config.fly_org --json | from json

  if ($existing_apps | any {|app| $app.ID == $fly_app}) {
    print $"Fly app ($fly_app) already exists."
    return
  }

  fly apps create --org $config.fly_org --name $fly_app
}
