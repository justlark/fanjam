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
