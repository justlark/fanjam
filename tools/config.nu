def get-user-email [] {
  let repo_path = $env.FILE_PWD | path dirname
  let jj_path = $repo_path | path join ".jj"
  let git_path = $repo_path | path join ".git"

  if ($jj_path | path exists) {
    jj config get user.email
  } else if ($git_path | path exists) {
    git config get user.email
  } else {
    error make { msg: "This is not a git or jj repo!" }
  }
}

def get-global-config [] {
  let repo_path = $env.FILE_PWD | path dirname
  let config_path = $repo_path | path join "infra" "config" "globals.yaml"

  open $config_path
}

def get-secret-global-config [] {
  let repo_path = $env.FILE_PWD | path dirname
  let secrets_file = $repo_path | path join "infra" "config" "secret_globals.enc.yaml"
  let sops_config_file = $repo_path | path join "infra" "config" ".sops.yaml"

  sops --config $sops_config_file decrypt $secrets_file | from yaml
}

def check-advisory-permissions [stage: string] {
  let config = get-global-config
  let secret_globals = get-secret-global-config
  let current_user = get-user-email
  let stage_admins = $secret_globals.advisory_stage_admins | default [] | get $stage

  if (not ($current_user in $stage_admins)) {
    error make { msg: $"You are not authorized to make changes in the `($stage)` stage. This is only an advisory check and can be overridden." }
  }

  let needs_confirmation = $config.stages | where name == $stage | first | get "confirm"

  if (not $needs_confirmation) {
    return
  }

  let given_stage = input $"Enter the name of the stage \(`($stage)`\) to confirm: "

  if ($given_stage != $stage) {
    error make { msg: "Stage name does not match. Aborting." }
  }
}

def get-env-config [env_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let env_file = $repo_path | path join "infra" "environments" $env_name "env.yaml"
  open $env_file
}

def get-tofu-env [] {
  let repo_path = $env.FILE_PWD | path dirname
  let secrets_file = $repo_path | path join "infra" "config" "secret_env.enc.yaml"
  let sops_config_file = $repo_path | path join "infra" "config" ".sops.yaml"

  sops --config $sops_config_file decrypt $secrets_file | from yaml
}

def get-tofu-vars [] {
  let repo_path = $env.FILE_PWD | path dirname
  let vars_file = $repo_path | path join "infra" "config" "vars.yaml"
  let secrets_file = $repo_path | path join "infra" "config" "secret_vars.enc.yaml"
  let sops_config_file = $repo_path | path join "infra" "config" ".sops.yaml"

  let tf_plaintext_vars = open $vars_file
  let tf_secret_vars = sops --config $sops_config_file decrypt $secrets_file | from yaml
  let tf_vars = $tf_plaintext_vars | merge $tf_secret_vars

  $tf_vars | items {|key, value| [$"TF_VAR_($key)", $value] } | into record
}
