# FanJam

FanJam is a free event planning app for conventions. Con organizers have a
dashboard where they can schedule panels, assign rooms, and make announcements,
and that information is shared with attendees in real time via a web app.

## Architecture

See the [architecture diagram](./docs/architecture.svg). This diagram is
rendered using the [D2](https://d2lang.com/) diagramming language; the source
is [here](./docs/architecture.d2).

- The client is written in Vue and TypeScript and hosted on [Cloudflare
  Workers](https://developers.cloudflare.com/workers/).
- There is a serverless function written in TypeScript and hosted on Cloudflare
  Workers which sits in front of the CDN to serve headers and dynamic metadata.
- The backend is a serverless function written in Rust and hosted on Cloudflare
  Workers.
- The storage providers for the backend are [Cloudflare Workers
  KV](https://developers.cloudflare.com/kv/) and [Neon
  Postgres](https://neon.tech).
- The database and interface for con organizers is provided by a
  [NocoDB](https://nocodb.com/) instance hosted on [Fly.io](https://fly.io/).
- The Postgres provider for NocoDB is Neon.
- The Redis provider for NocoDB is [Upstash](https://upstash.com/).
- The object storage provider for NocoDB is [Cloudflare
  R2](https://developers.cloudflare.com/r2/).
- The SMTP provider for NocoDB is [MailerSend](https://mailersend.com).

FanJam is single-tenant, meaning we have a separate NocoDB instance, Postgres
cluster, and object storage bucket per tenant environment. All environments
share a Redis database.

We have a single deployment of the frontend and a single deployment of the
backend which are shared across tenant environments. We have separate `prod`
and `test` deployments of each.

## Development

To build and run the app, you'll need to install:

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

To deploy the app, you'll additionally need to install:

- [Nu](https://www.nushell.sh/book/installation.html)
- [flyctl](https://fly.io/docs/flyctl/install/)
- [OpenTofu](https://opentofu.org/docs/intro/install/)
- [SOPS](https://getsops.io/)
- [age](https://age-encryption.org/)

You can use `just` to build and deploy the app. Run `just` to see a list of
recipes.

`just` recipes that accept a `stage` accept either `prod` or `test`. This is
for infrastructure that is shared between tenant environments. `just` recipes
that accept an `env` accept the name of a tenant environment.

This project is referred to by the codename "sparklefish" throughout the
codebase and infrastructure.

## OpenTofu

Secrets for OpenTofu are stored in the repo encrypted with maintainers' SSH
keys via [SOPS](https://getsops.io/) and [age](https://age-encryption.org/).

To deploy infrastructure, you'll first need your SSH key authorized by adding
it to [./infra/.sops.yaml](./infra/.sops.yaml) and running this command:

```
just sops updatekeys ./infra/secrets.enc.yaml
```

Once your key is authorized, set the env var `SOPS_AGE_SSH_PRIVATE_KEY_FILE` to
the path of your private SSH key. You can put this in a `./.env` file in the
root of the repo; it will be ignored by git.

To run `tofu` commands with the secrets pulled into your environment, use the
`just` recipe:

```
just tofu plan
```

You can edit secret OpenTofu variables interactively like this:

```
just sops edit secrets.enc.yaml
```

You can edit plaintext OpenTofu variables by editing
[./infra/vars.yaml](./infra/vars.yaml).

You can set additional secret env vars to be passed to OpenTofu like this:

```
just sops edit env.enc.yaml
```

These additional env vars are used to configure the Postgres state backend.

## Deployment

These are the instructions for deploying a new instance of FanJam at
`https://foo.fanjam.live` in the `prod` environment.

Start by generating the necessary configuration for the environment.

```
just configure-env foo prod
```

This will generate some files, which can be edited as necessary. Check them
into the repo.

Deploy the supporting infrastructure using Terraform.

```
just tofu plan
just tofu apply
```

Create a new NocoDB instance.

```
just create-env foo
```

View the NocoDB system user login credentials for the new environment.

```
just get-creds foo
```

At this point, you'll need to log into the NocoDB instance manually to generate
an API token. Call it "Worker".

Finally, initialize the NocoDB instance with a new base. It will prompt you for
the API token interactively. Once you've done this, you can lose the token.

```
just init-env foo
```

## Upgrade NocoDB

These are instructions for upgrading NocoDB in deployed environments.

Redeploy the `playground` environment using `just deploy-env playground`. This
environment always uses the latest version of NocoDB. Once you've tested the
new version and are confident it's stable, update the `fly.yaml` of all the
environments you want to upgrade, then redeploy them. Production environments
should always be pinned to a specific version, not `latest`.

Finally, update the [config.yaml](./infra/config.yaml) with the latest version
tag (not `latest`).

## Copyright

Copyright © 2025 Lark Aster

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.
