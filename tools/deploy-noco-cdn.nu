#!/usr/bin/env nu

source ./config.nu

def main [env_name: string] {
  let config = get-global-config
  let repo_path = $env.FILE_PWD | path dirname

  npx wrangler@latest --cwd ($repo_path | "nocodb") deploy --env $env_name --domain $"($env_name).($config.app_base_domain)" --var $"DASHBOARD_PATH:($config.dashboard_path)"
}
