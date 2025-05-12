#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  let api_token = input "Enter the NocoDB API token: "

  admin-api put $stage_name $"/tokens/($env_name)" {
    token: $api_token,
  }
}
