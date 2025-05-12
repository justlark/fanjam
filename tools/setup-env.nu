#!/usr/bin/env nu

source ./api.nu

def main [stage_name: string, env_name: string] {
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

  call_admin_api $stage_name $"/links/($env_name)"

  call_admin_api $stage_name "/" $request_body

  call_admin_api $stage_name "/" $request_body
}
