#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [stage: string, alias: string, target: string] {
  admin-api put $stage $"/admin/aliases/($alias)" {
    env_id: $target
  }
}
