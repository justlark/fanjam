# list recipes
default:
  @just --list

# install npm dependencies for the client
[working-directory: "./client/"]
_install-client:
  npm install

# run the client locally
[working-directory: "./client/"]
[group("test locally")]
run-client stage="test": _install-client
  npm run dev:{{ stage }}

# run the server locally
[group("test locally")]
[working-directory: "./server/"]
run-server stage="test":
  npx wrangler --env {{ stage }} dev --remote

# type check and lint the client
[working-directory: "./client/"]
[group("test locally")]
check-client: _install-client
  npm run type-check
  npm run lint

# type check and lint the server
[working-directory: "./server/"]
[group("test locally")]
check-server:
  cargo check
  cargo clippy

# deploy the client
[working-directory: "./client/"]
[group("deploy changes")]
[confirm("Deploy the client now?")]
deploy-client stage: _install-client
  npm run deploy:{{ stage }}

# deploy the server
[working-directory: "./server/"]
[group("deploy changes")]
[confirm("Deploy the server now?")]
deploy-server stage:
  npx wrangler deploy --env {{ stage }}

# tail the server logs
[working-directory: "./server/"]
[group("deploy changes")]
tail-server stage:
  npx wrangler tail --env {{ stage }}

# tail the client worker logs
[working-directory: "./client/"]
[group("deploy changes")]
tail-client stage:
  npx wrangler tail --env {{ stage }}

# generate the configuration for an environment
[group("deploy environments")]
configure-env env stage:
  ./tools/configure-env.nu {{ env }} {{ stage}}

# update environment secrets passed to the NocoDB instance
[group("deploy environments")]
update-secrets env:
  ./tools/update-secrets.nu {{ env }}

# create a new NocoDB instance
[group("deploy environments")]
create-env env:
  ./tools/create-fly-app.nu {{ env }}
  ./tools/update-secrets.nu {{ env }}
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy
  ./tools/deploy-certs.nu {{ env }}

# get the system user login credentials for an environment
[group("manage environments")]
get-creds env:
  ./tools/get-creds.nu {{ env }}

# redeploy an existing NocoDB instance
[group("deploy environments")]
deploy-env env:
  ./tools/create-deploy-backup.nu {{ env }}
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy

# configure an environment with a NocoDB API token
[group("initialize environments")]
set-noco-token env:
  ./tools/set-noco-token.nu {{ env }}

# create a new empty base in a NocoDB instance
[group("initialize environments")]
create-base env:
  ./tools/create-noco-base.nu {{ env }}

# initialize a new environment
[group("initialize environments")]
[confirm("Are you sure? Make sure you're only using this recipe for one-time setup of new environments.")]
init-env env:
  ./tools/set-noco-token.nu {{ env }}
  ./tools/create-noco-base.nu {{ env }}
  ./tools/generate-app-link.nu {{ env }}

# generate a new app link for an environment
[group("manage environments")]
[confirm("Are you sure? The old link will stop working for attendees.")]
generate-app-link env:
  ./tools/generate-app-link.nu {{ env }}

# delete an environment's NocoDB base and all its data
[group("manage environments")]
[confirm("Are you sure? This will delete all data in the environment.")]
delete-base env:
  ./tools/delete-base.nu {{ env }}

# get the app link for an environment
[group("manage environments")]
get-app-link env:
  ./tools/get-app-link.nu {{ env }}

# apply any pending schema migrations to an environment
[group("manage environments")]
[confirm("Are you sure? This will apply any pending schema migrations to the environment.")]
migrate-env env:
  ./tools/migrate-env.nu {{ env }}

# get the current schema version of an environment
[group("manage environments")]
get-schema-version env:
  ./tools/get-schema-version.nu {{ env }}

# clear the server cache for an environment
[group("manage environments")]
clear-cache env:
  ./tools/clear-cache.nu {{ env }}

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
edit-config env:
  ./tools/edit-config.nu {{ env }}
