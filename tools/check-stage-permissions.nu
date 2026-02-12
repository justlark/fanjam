#!/usr/bin/env nu

source ./config.nu

def main [stage: string] {
  check-advisory-permissions $stage
}
