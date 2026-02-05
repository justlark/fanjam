#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [stage: string] {
  let response = admin-api get $stage $"/admin/aliases"
  $response.aliases
}
