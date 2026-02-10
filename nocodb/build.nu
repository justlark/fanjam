#!/usr/bin/env nu

source ../tools/config.nu

def main [env_name: string] {
  let config = get-global-config
  let out_dir = $env.FILE_PWD | path join "public"

  let fly_file = $env.FILE_PWD | path dirname | path join "infra" "environments" $env_name "fly.yaml"
  let fly_config = open $fly_file
  let noco_image = $fly_config.build.image

  # TODO: We can remove this once we have fully migrated off of the upstream
  # NocoDB image.
  let base_image = $noco_image | split row ":" | get 0
  let assets_path = if ($base_image | str starts-with "ghcr.io/justlark/nocodb-fanjam") {
    "/usr/src/app/docker/nc-gui/"
  } else if ($base_image | str starts-with "nocodb/nocodb") {
    "/usr/src/app/node_modules/nc-lib-gui/lib/dist/"
  } else {
    error make { msg: $"This is not a valid NocoDB image: ($noco_image)" }
  }


  rm --recursive --force $out_dir
  mkdir $out_dir

  podman pull $noco_image
  let container_id = podman create $noco_image | complete | get "stdout" | str trim

  podman cp $"($container_id):($assets_path)" ($out_dir | path join $config.dashboard_path)

  podman rm $container_id
}
