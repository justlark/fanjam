set shell := ["nu", "-c"]
set dotenv-load

# list recipes
default:
  @just --list

# install npm dependencies for the client
[working-directory: "./client/"]
_install-client:
  npm install

# install npm dependencies for Playwright
[working-directory: "./playwright/"]
_install-playwright:
  npm install

# prompt the user to confirm before they modify this environment
_confirm-env env:
  ./tools/check-env-permissions.nu {{ env }}

# prompt the user to confirm before they modify this stage
_confirm-stage stage:
  ./tools/check-stage-permissions.nu {{ stage }}

# run the client locally
[working-directory: "./client/"]
[group("test locally")]
run-client stage="test": _install-client
  npm run dev:{{ stage }}

# run the server locally
[group("test locally")]
[working-directory: "./server/"]
run-server stage="test":
  npx wrangler@latest --env {{ stage }} dev --remote

# type check and lint the client
[working-directory: "./client/"]
[group("test locally")]
check-client: _install-client
  npm run type-check
  npm run lint
  npx prettier --check ./src/ ./worker/

# type check and lint the server
[working-directory: "./server/"]
[group("test locally")]
check-server:
  cargo check
  cargo clippy
  cargo fmt --check

# deploy the client
[working-directory: "./client/"]
[group("deploy changes")]
[confirm("Deploy the client now?")]
deploy-client stage: (_confirm-stage stage) _install-client
  npm run deploy:{{ stage }}

# deploy the server
[working-directory: "./server/"]
[group("deploy changes")]
[confirm("Deploy the server now?")]
deploy-server stage: (_confirm-stage stage)
  npx wrangler@latest deploy --env {{ stage }}

# tail the server logs
[working-directory: "./server/"]
[group("deploy changes")]
tail-server stage:
  npx wrangler@latest tail --env {{ stage }}

# tail the client worker logs
[working-directory: "./client/"]
[group("deploy changes")]
tail-client stage:
  npx wrangler@latest tail --env {{ stage }}

# generate the configuration for an environment
[group("manage environments")]
configure-env env stage:
  ./tools/configure-env.nu {{ env }} {{ stage }}

# update environment secrets passed to the NocoDB instance
[group("manage environments")]
update-secrets env: (_confirm-env env)
  ./tools/update-secrets.nu {{ env }}

# create a new NocoDB instance
[group("manage environments")]
create-env env: (_confirm-env env)
  ./tools/graphile-migrate.nu {{ env }} migrate
  ./tools/create-fly-app.nu {{ env }}
  ./tools/update-secrets.nu {{ env }}
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy
  ./tools/deploy-certs.nu {{ env }}
  ./nocodb/build.nu {{ env }}
  ./tools/deploy-noco-cdn.nu {{ env }}

# get the system user login credentials for an environment
[group("manage environments")]
get-creds env:
  ./tools/get-creds.nu {{ env }}

# redeploy an existing NocoDB instance
[group("manage environments")]
deploy-env env: (_confirm-env env)
  ./tools/create-deploy-backup.nu {{ env }}
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy
  ./nocodb/build.nu {{ env }}
  ./tools/deploy-noco-cdn.nu {{ env }}

# configure an environment with a NocoDB API token
[group("manage environments")]
set-noco-token env: (_confirm-env env)
  ./tools/set-noco-token.nu {{ env }}

# create a new empty base in a NocoDB instance
[group("manage environments")]
create-base env: (_confirm-env env)
  ./tools/create-noco-base.nu {{ env }}

# initialize a new environment
[group("manage environments")]
[confirm("Are you sure? Make sure you're only using this recipe for one-time setup of new environments.")]
init-env env slug: (_confirm-env env)
  ./tools/set-noco-token.nu {{ env }}
  ./tools/create-noco-base.nu {{ env }}
  ./tools/set-app-slug.nu {{ env }} {{ slug }}

# update the app link for an environment
[group("manage environments")]
[confirm("Are you sure? The app will no longer be accessible via the original link. If you want a redirect, you will need to add an alias manually.")]
set-app-link env slug: (_confirm-env env)
  ./tools/set-app-slug.nu {{ env }} {{ slug }}

# list app link aliases across all environments
[group("manage environments")]
list-app-aliases stage:
  ./tools/list-app-aliases.nu {{ stage }}

# add an app link alias
[group("manage environments")]
[confirm("Are you sure? The app will be accessible via this link.")]
add-app-alias stage alias target: (_confirm-stage stage)
  ./tools/add-app-alias.nu {{ stage }} {{ alias }} {{ target }}

# delete an app link alias
[group("manage environments")]
[confirm("Are you sure? The app will no longer be accessible via this link.")]
delete-app-alias stage alias: (_confirm-stage stage)
  ./tools/delete-app-alias.nu {{ stage }} {{ alias }}

# delete an environment's NocoDB base and all its data
[group("manage environments")]
[confirm("Are you sure? This will delete all data in the environment.")]
delete-base env: (_confirm-env env)
  ./tools/delete-base.nu {{ env }}

# get the app and dashboard link for an environment
[group("manage environments")]
get-links env:
  ./tools/get-app-link.nu {{ env }}

# apply any pending schema migrations to an environment
[group("manage environments")]
[confirm("Are you sure? This will apply any pending schema migrations to the environment.")]
migrate-env env: (_confirm-env env)
  ./tools/migrate-env.nu {{ env }}

# run a graphile-migrate command
[group("manage environments")]
graphile-migrate env +params: (_confirm-env env)
  ./tools/graphile-migrate.nu {{ env }} {{ params }}

# get the current schema version of an environment
[group("manage environments")]
get-schema-version env:
  ./tools/get-schema-version.nu {{ env }}

# clear the server cache for an environment
[group("manage environments")]
clear-cache env: (_confirm-env env)
  ./tools/clear-cache.nu {{ env }}

# seed an environment with demo data from a SQL dump
[group("manage environments")]
[confirm("Are you sure? This will overwrite the environment's data with demo data.")]
seed-data env dump: (_confirm-env env)
  ./tools/seed-data.nu {{ env }} {{ dump }}

# upload an environment-specific asset (from stdin)
[group("manage environments")]
upload-asset env name: (_confirm-env env)
  ./tools/upload-asset.nu {{ env }} {{ name }}

# show the documentation for the environment config
[group("configure environments")]
describe-config stage:
  ./tools/describe-config.nu {{ stage }}

# show the config for an environment
[group("configure environments")]
get-config env:
  ./tools/get-config.nu {{ env }}

# edit the config for an environment interactively
[group("configure environments")]
edit-config env: (_confirm-env env)
  ./tools/edit-config.nu {{ env }}

# run an OpenTofu command
[group("manage infrastructure")]
[working-directory: "./infra/"]
tofu *args:
  ../tools/run-tofu.nu {{ args }}

# run a SOPS command
[group("manage infrastructure")]
sops *args:
  sops --config ./infra/config/.sops.yaml {{ args }}

# run the Playwright server in a container
[group("run playwright")]
[working-directory: "./playwright/"]
[linux]
start-playwright:
  podman run --add-host=hostmachine:host-gateway -p 3001:3000 --rm --init -it --name playwright-server  --workdir /home/pwuser --user pwuser mcr.microsoft.com/playwright:v1.55.0-noble /bin/sh -c "npx -y playwright@1.55.0 run-server --port 3000 --host 0.0.0.0"


# run the Playwright server in a container
[group("run playwright")]
[working-directory: "./playwright/"]
[macos]
start-playwright:
  podman run -p 3001:3000 --rm --init -it --name playwright-server --workdir /home/pwuser --user pwuser mcr.microsoft.com/playwright:v1.55.0-noble /bin/sh -c "npx -y playwright@1.55.0 run-server --port 3000 --host 0.0.0.0"

# run Playwright tests against a local instance of the app
[group("run playwright")]
[working-directory: "./playwright/"]
run-playwright *args: _install-playwright
  with-env { PW_TEST_CONNECT_WS_ENDPOINT: "ws://127.0.0.1:3001/" } { npx playwright@1.55.0 test {{ args }} }
