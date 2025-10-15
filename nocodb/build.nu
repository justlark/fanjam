#!/usr/bin/env nu

source ../tools/config.nu

def main [tag: string] {
  rm --recursive --force ./public/
  mkdir ./public/

  podman pull $"nocodb/nocodb:($tag)"
  let container_id = podman create $"nocodb/nocodb:($tag)" | complete | get "stdout" | str trim

  podman cp $"($container_id):/usr/src/app/node_modules/nc-lib-gui/lib/dist/_nuxt/" ./public/

  podman rm $container_id
}
