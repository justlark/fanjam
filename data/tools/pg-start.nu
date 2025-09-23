#!/usr/bin/env nu

source ./common.nu

def main [] {
  podman run --name $container_name --replace --env POSTGRES_HOST_AUTH_METHOD=trust --env $"POSTGRES_DB=($db_name)" --env $"POSTGRES_USER=($pg_user)" docker.io/postgres:latest
}
