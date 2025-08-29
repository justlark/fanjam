#!/usr/bin/env nu

source ./config.nu

def --wrapped main [...rest] {
  get-tofu-env | load-env
  get-tofu-vars | load-env

  tofu ...$rest
}
