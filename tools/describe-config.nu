#!/usr/bin/env nu

source ./http.nu

def main [stage: string] {
  let spec = admin-api get $stage $"/admin/config-spec"
  $spec | each {|item| { $item.key: $item.help }} | into record
}
