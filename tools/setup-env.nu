#!/usr/bin/env nu

def main [stage_name: string, env_name: string] {
  let api_base = match $stage_name {
    "test" => "https://api-test.fanjam.live",
    "prod" => "https://api.fanjam.live",
    _ => {
      print --stderr $"Invalid stage name: ($stage_name)"
      exit 1
    }
  }

  let title = input "Enter the human-readable name of the environment: "
  let api_token = input --suppress-output "Enter the NocoDB API token: "

  print

  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  let app_domain = open $env_file | get app_domain

  let request_body = {
    title: $title,
    dash_domain: $app_domain,
    api_token: $api_token,
  }

  let api_endpoint = $"($api_base)/bases"

  http post --content-type "application/json" $api_endpoint $request_body
}
