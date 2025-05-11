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

  let admin_api_tokens = terraform -chdir=./infra/ output -json worker_admin_api_tokens | from json
  let admin_api_token = $admin_api_tokens | get $stage_name

  let title = input "Enter the user-facing name of the environment: "
  let email = input "Enter the email address of the initial user: "
  let api_token = input "Enter the NocoDB API token: "

  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  let app_domain = open $env_file | get app_domain

  let request_body = {
    title: $title,
    dash_domain: $app_domain,
    api_token: $api_token,
    email: $email,
  }

  let headers = ["Authorization", $"Bearer ($admin_api_token)"]

  let api_endpoint = $"($api_base)/bases"

  http post --content-type "application/json" --headers $headers $api_endpoint $request_body
}
