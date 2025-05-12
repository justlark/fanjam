#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  let title = input "Enter the user-facing name of the environment: "
  let email = input "Enter the email address of the initial user: "

  admin-api post $stage_name $"/bases/($env_name)" {
    title: $title,
    email: $email,
  }
}
