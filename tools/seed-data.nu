#!/usr/bin/env nu

source ./config.nu
source ./http.nu

let container_name = "sparklefish-pg-temp"
let pg_db = "sparklefish"
let pg_user = "sparklefish"

# Clean the SQL dump by changing the schema name to match the new base ID and
# unsetting timestamps.
def clean-dump [old_schema: string, new_schema: string] {
  let dump = $in

  let sql = $"
    ALTER SCHEMA ($old_schema) RENAME TO ($new_schema);

    UPDATE ($new_schema).events SET created_by = NULL;
    UPDATE ($new_schema).events SET updated_by = NULL;

    UPDATE ($new_schema).locations SET created_by = NULL;
    UPDATE ($new_schema).locations SET updated_by = NULL;

    UPDATE ($new_schema).people SET created_by = NULL;
    UPDATE ($new_schema).people SET updated_by = NULL;

    UPDATE ($new_schema).categories SET created_by = NULL;
    UPDATE ($new_schema).categories SET updated_by = NULL;

    UPDATE ($new_schema).tags SET created_by = NULL;
    UPDATE ($new_schema).tags SET updated_by = NULL;

    UPDATE ($new_schema).announcements SET created_by = NULL;
    UPDATE ($new_schema).announcements SET updated_by = NULL;
    UPDATE ($new_schema).announcements SET attachment = NULL;

    UPDATE ($new_schema).about SET created_by = NULL;
    UPDATE ($new_schema).about SET updated_by = NULL;
    UPDATE ($new_schema).about SET files = NULL;

    UPDATE ($new_schema).links SET created_by = NULL;
    UPDATE ($new_schema).links SET updated_by = NULL;

    UPDATE ($new_schema).pages SET created_by = NULL;
    UPDATE ($new_schema).pages SET updated_by = NULL;
    UPDATE ($new_schema).pages SET files = NULL;
  "

  echo $dump | podman exec --interactive $container_name psql --username $pg_user --dbname $pg_db | complete | ignore
  echo $sql | podman exec --interactive $container_name psql --username $pg_user --dbname $pg_db | complete | ignore
  podman exec --interactive $container_name pg_dump --data-only --username $pg_user --dbname $pg_db --schema $new_schema | complete | get "stdout"
}

# Get Postgres connection info (non-pooling) from OpenTofu.
def get-pg-creds [env_name: string]: nothing -> record {
  with-env (get-tofu-env) {
    tofu -chdir=./infra/ output -json psql | from json | get $env_name
  }
}

# Query the sparklefish database for the NocoDB base ID.
def get-base-id [pg: record, env_name: string]: nothing -> string {
  let base_id = (
    with-env { PGPASSWORD: $pg.password } {
      (psql
        --host $pg.host
        --port $pg.port
        --username $pg.user
        --dbname sparklefish
        --tuples-only
        --no-align
        --command "SELECT base_id FROM noco_bases ORDER BY sequence DESC LIMIT 1")
    } | str trim
  )

  if ($base_id | is-empty) {
    error make {
      msg: $"No base found for environment '($env_name)'. Create one first with `just create-base ($env_name)`."
    }
  }

  $base_id
}

# Verify the target environment has no existing data.
def check-target-empty [pg: record, base_id: string, env_name: string] {
  let row_count = (
    with-env { PGPASSWORD: $pg.password } {
      (psql
        --host $pg.host
        --port $pg.port
        --username $pg.user
        --dbname noco
        --tuples-only
        --no-align
        --command $"SELECT count\(*\) FROM ($base_id).events")
    } | str trim | into int
  )

  if $row_count > 0 {
    error make {
      msg: $"Target environment '($env_name)' already has data. Delete the base first and recreate it."
    }
  }
}

# Extract the source schema name from a SQL dump file.
def extract-source-schema [dump_file: string]: nothing -> string {
  open --raw $dump_file
    | lines
    | where $it =~ "^CREATE SCHEMA "
    | first
    | parse "CREATE SCHEMA {schema};"
    | get schema | first
}

# Start the local Postgres container (detached) and wait for readiness.
def start-local-postgres [] {
  print "Starting local Postgres container..."

  (podman run
    --name $container_name
    --replace
    --detach
    --env POSTGRES_HOST_AUTH_METHOD=trust
    --env $"POSTGRES_DB=($pg_db)"
    --env $"POSTGRES_USER=($pg_user)"
    docker.io/postgres:latest)

  print "Waiting for Postgres to be ready..."

  mut ready = false

  for _ in 1..30 {
    let result = (podman exec $container_name pg_isready --username $pg_user --dbname $pg_db | complete)

    if $result.exit_code == 0 { $ready = true; break }

    sleep 1sec
  }

  if not $ready {
    podman stop $container_name | ignore
    error make { msg: "Local Postgres did not become ready in 30 seconds." }
  }

  print "Local Postgres is ready."
}

# Stop the local Postgres container.
def stop-local-postgres [] {
  print "Stopping local Postgres container..."
  podman stop $container_name | ignore
}

# Clean the dump and restore it to the target environment.
def clean-and-restore [
  pg: record, dump_file: string, source_schema: string, base_id: string
] {
  print $"Cleaning dump: ($source_schema) -> ($base_id)"

  let cleaned_sql = (open --raw $dump_file | clean-dump $source_schema $base_id)

  print "Restoring to target environment..."

  ($cleaned_sql | with-env { PGPASSWORD: $pg.password } {
    (psql
      --dbname noco
      --host $pg.host
      --port $pg.port
      --username $pg.user
      --quiet)
  })
}

def main [env_name: string, dump_name: string] {
  let repo_path = $env.FILE_PWD | path dirname
  let dump_file = $repo_path | path join "data" $"($dump_name).sql"

  let pg = get-pg-creds $env_name
  let base_id = get-base-id $pg $env_name

  print $"Target base ID: ($base_id)"

  check-target-empty $pg $base_id $env_name

  let source_schema = extract-source-schema $dump_file

  print $"Source schema: ($source_schema)"

  start-local-postgres

  try {
    clean-and-restore $pg $dump_file $source_schema $base_id

    print "Clearing server cache..."

    let env_config = get-env-config $env_name
    admin-api delete $env_config.stage $"/admin/env/($env_name)/cache"

    print "Done! Data seeded successfully."
  } catch {|e|
    print --stderr $"Seed failed: ($e.msg)"
  }

  stop-local-postgres
}
