#!/usr/bin/env nu

def main [env_name: string] {
  let all_secrets = terraform -chdir=./infra/ output -json secrets | from json
  let env_secrets = $all_secrets | get $env_name
  let secret_pairs = $env_secrets | transpose name value | each {|secret| $"($secret.name)=($secret.value)" }

  fly secrets set --app $"sparklefish-noco-($env_name)" ...$secret_pairs
}
