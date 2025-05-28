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
run-client: _install-client
  npm run dev

# run the server locally
[group("test locally")]
[working-directory: "./server/"]
run-server:
  npx wrangler --env test dev

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

# generate the configuration for an environment
[group("manage infrastructure")]
configure-env env:
  ./tools/configure-env.nu {{ env }}

# create a new NocoDB instance without deploying
[group("manage infrastructure")]
create-env env:
  fly -c ./infra/environments/{{ env }}/fly.yaml launch --org sparklefish --copy-config --yaml --no-deploy --yes

# pass environment secrets to the NocoDB instance
[group("manage infrastructure")]
deploy-secrets env:
  ./tools/configure-secrets.nu {{ env }}

# deploy an existing NocoDB instance
[group("manage infrastructure")]
deploy-env env stage="prod":
  ./tools/create-deploy-backup.nu {{ stage }} {{ env }}
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy

# set up TLS certificates for an environment
[group("manage infrastructure")]
deploy-certs env:
  fly -c ./infra/environments/{{ env }}/fly.yaml certs add {{ env }}.fanjam.live

# get the system user login credentials for an environment
[group("manage infrastructure")]
get-creds env:
  ./tools/get-creds.nu {{ env }}

# configure an environment with a NocoDB API token
[group("manage environments")]
set-noco-token env stage="prod":
  ./tools/set-noco-token.nu {{ stage }} {{ env }}

# generate a new app link for an environment
[group("manage environments")]
[confirm("Are you sure? The old link will stop working for attendees.")]
generate-app-link env stage="prod":
  ./tools/generate-app-link.nu {{ stage }} {{ env }}

# delete an environment's NocoDB base and all its data
[group("manage environments")]
[confirm("Are you sure? This will delete all data in the environment.")]
delete-base env stage="prod":
  ./tools/delete-base.nu {{ stage }} {{ env }}

# get the app link for an environment
[group("manage environments")]
get-app-link env stage="prod":
  ./tools/get-app-link.nu {{ stage }} {{ env }}

# apply any pending schema migrations to an environment
[group("manage environments")]
[confirm("Are you sure? This will apply any pending schema migrations to the environment.")]
migrate-env env stage="prod":
  ./tools/migrate-env.nu {{ stage }} {{ env }}

# get the current schema version of an environment
[group("manage environments")]
get-schema-version env stage="prod":
  ./tools/get-schema-version.nu {{ stage }} {{ env }}

# initialize a new environment
[group("manage environments")]
[confirm("Are you sure? Make sure you're only using this recipe for one-time setup of new environments.")]
init-env env stage="prod":
  ./tools/set-noco-token.nu {{ stage }} {{ env }}
  ./tools/create-noco-base.nu {{ stage }} {{ env }}
  ./tools/generate-app-link.nu {{ stage }} {{ env }}
