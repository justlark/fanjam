#!/usr/bin/env nu

source ./config.nu

def --wrapped main [...rest] {
  let tf_plaintext_vars = open ./vars.yaml
  let tf_secret_vars = sops decrypt ./secrets.enc.yaml | from yaml
  let tf_vars = $tf_plaintext_vars | merge $tf_secret_vars

  let tf_env_vars = $tf_vars | items {|key, value| [$"TF_VAR_($key)", $value] } | into record
  let extra_env_vars = sops decrypt ./env.enc.yaml | from yaml
  let env_vars = $tf_env_vars | merge $extra_env_vars

  load-env $env_vars

  tofu ...$rest
}
