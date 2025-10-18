#!/usr/bin/env nu

def main [env_name: string] {
  let fly_file = $env.FILE_PWD | path dirname | path join "infra" "environments" $env_name "fly.yaml"
  let fly_config = open $fly_file
  let noco_image = $fly_config.build.image

  rm --recursive --force ./public/

  podman pull $noco_image
  let container_id = podman create $noco_image | complete | get "stdout" | str trim

  podman cp $"($container_id):/usr/src/app/node_modules/nc-lib-gui/lib/dist/" ./public

  podman rm $container_id
}
