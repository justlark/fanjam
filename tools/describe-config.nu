#!/usr/bin/env nu

source ./http.nu

def main [stage: string] {
  admin-api get $stage $"/admin/config-docs"
}
