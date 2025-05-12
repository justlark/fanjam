#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  admin-api delete $stage_name $"/bases/($env_name)"
}
