def get-global-config [] {
  let repo_path = $env.FILE_PWD | path dirname
  let config_path = $repo_path | path join "infra" "config.yaml"
  return (open $config_path)
}

def get-env-config [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  return (open $env_file)
}

def get-tofu-env [] {
  let repo_path = $env.FILE_PWD | path dirname
  let secrets_file = $repo_path | path join "infra" "env.enc.yaml"

  sops decrypt $secrets_file | from yaml
}

def get-tofu-vars [] {
  let repo_path = $env.FILE_PWD | path dirname
  let vars_file = $repo_path | path join "infra" "vars.yaml"
  let secrets_file = $repo_path | path join "infra" "secrets.enc.yaml"

  let tf_plaintext_vars = open $vars_file
  let tf_secret_vars = sops decrypt $secrets_file | from yaml
  let tf_vars = $tf_plaintext_vars | merge $tf_secret_vars

  $tf_vars | items {|key, value| [$"TF_VAR_($key)", $value] } | into record
}
