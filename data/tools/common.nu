let container_name = "sparklefish-pg-temp"
let db_name = "sparklefish"
let pg_user = "sparklefish"

def --wrapped pg-exec [cmd: string, ...args] {
  podman exec --interactive $container_name $cmd ...$args | complete | get "stdout"
}

def psql-exec [sql: string] {
  echo $sql | pg-exec psql --username $pg_user --dbname $db_name | (std null-device)
}
