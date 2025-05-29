#!/usr/bin/env nu

source ./config.nu

def generate-fly-config [app: string, url: string, bucket: string] {
  let config = get-global-config

  {
    app: $app,
    primary_region: $config.default_fly_region,
    build: {
      image: $"nocodb/nocodb:($config.nocodb_version)",
    },
    env: {
      NC_PUBLIC_URL: $url,
      NC_S3_BUCKET_NAME: $bucket,
      NC_S3_ENDPOINT: $"https://($config.cloudflare_account_id).r2.cloudflarestorage.com/($bucket)",
      NC_INVITE_ONLY_SIGNUP: "true",
      NC_ADMIN_EMAIL: ($config.admin_email),
    },
    http_service: {
      internal_port: 8080,
      force_https: true,
      auto_stop_machines: true,
      auto_start_machines: true,
      min_machines_running: 0,
      processes: ["app"],
    },
    vm: [
      {
        memory: "1gb",
        cpu_kind: "shared",
        cpus: 2,
      }
    ],
  }
}

def generate-app-name [env_name: string] {
  let chars = "0123456789"
  let random_len = 6
  let random_str = seq 1 $random_len | each { $chars | split chars | shuffle | get 0 } | str join

  $"sparklefish-noco-($env_name)-($random_str)"
}

def main [env_name: string] {
  let config = get-global-config

  let repo_path = $env.FILE_PWD | path dirname
  let env_path = $repo_path | path join "infra" "environments" $env_name

  let env_file = $env_path | path join "env.yaml"
  let fly_file = $env_path | path join "fly.yaml"

  let fly_app_name = if ($env_file | path exists) {
    open $env_file | get fly_app
  } else {
    generate-app-name $env_name
  }

  let app_url = $"https://($env_name).($config.app_base_domain)"
  let bucket_name = $"sparklefish-noco-($env_name)"

  let env_config = {
    fly_app: $fly_app_name
    app_domain: $env_name
    neon_region: $config.default_neon_region
    system_password_counter: 1
  }

  mkdir $env_path

  $env_config | to yaml | save --force $env_file
  generate-fly-config $fly_app_name $app_url $bucket_name | to yaml | save --force $fly_file
}
