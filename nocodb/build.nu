#!/usr/bin/env nu

source ../tools/config.nu

def main [env_name: string] {
  let config = get-global-config
  let out_dir = $env.FILE_PWD | path join "public"

  let fly_file = $env.FILE_PWD | path dirname | path join "infra" "environments" $env_name "fly.yaml"
  let fly_config = open $fly_file
  let noco_image = $fly_config.build.image

  let assets_path = if ($noco_image | str starts-with "ghcr.io/justlark/nocodb-fanjam") {
    "/usr/src/app/docker/nc-gui/"
  } else {
    error make { msg: $"This is not a supported NocoDB image: ($noco_image)" }
  }


  rm --recursive --force $out_dir
  mkdir $out_dir

  podman pull $noco_image
  let container_id = podman create $noco_image | complete | get "stdout" | str trim

  podman cp $"($container_id):($assets_path)" ($out_dir | path join $config.dashboard_path)

  podman rm $container_id
}
