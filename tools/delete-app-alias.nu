#!/usr/bin/env nu

source ./http.nu
source ./config.nu

def main [stage: string, alias: string] {
  admin-api delete $stage $"/admin/aliases/($alias)"
}
