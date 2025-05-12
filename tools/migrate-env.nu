#!/usr/bin/env nu

source ./http.nu

def main [stage_name: string, env_name: string] {
  admin-api post $stage_name $"/migrations/($env_name)"
}
