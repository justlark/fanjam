#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  admin-api get $stage_name $"/migrations/($env_name)"
}
