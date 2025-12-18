# FanJam

FanJam is an event planning app that bridges the gap between a single person
with a spreadsheet and the kinds of people who can afford enterprise event
planning software.

## Architecture

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
- The object storage provider for NocoDB is [Cloudflare
  R2](https://developers.cloudflare.com/r2/).
- The SMTP provider for NocoDB is [MailerSend](https://mailersend.com).
- The privacy-preserving analytics solution is a self-hosted instance of
  [Umami](https://umami.is/).

Out of the box, NocoDB hosts static assets from within the container via a Node
server. When deploying a NocoDB instance for FanJam, we instead host those
assets on a CDN (Cloudflare Workers), forwarding NocoDB API requests to the
container via an edge function. When we deploy a NocoDB instance, the pipeline
extracts the client bundle from the NocoDB image and pushes it to the CDN. It's
important to note that NocoDB _is not_ intended to be deployed this way.

FanJam is single-tenant, meaning we have a separate NocoDB instance, Postgres
cluster, and object storage bucket per tenant environment.

We have a single deployment of the frontend and a single deployment of the
backend which are shared across tenant environments. We have separate `prod`
and `test` deployments of each.

See the following architecture diagrams, laid out using
[D2](https://d2lang.com/):

[System Architecture](./docs/architecture.svg) ([source](./docs/architecture.d2))

## Development

To build and run the app, you'll need to install:

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

To deploy the app, you'll additionally need to install:

- [Podman](https://podman.io/docs/installation#installing-on-linux)
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
it to [./infra/.sops.yaml](./infra/.sops.yaml) and running these commands:

```
just sops updatekeys ./secrets.enc.yaml
just sops updatekeys ./env.enc.yaml
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
just sops edit ./secrets.enc.yaml
```

You can edit plaintext OpenTofu variables by editing
[./infra/vars.yaml](./infra/vars.yaml).

You can set additional secret env vars to be passed to OpenTofu like this:

```
just sops edit ./env.enc.yaml
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

Deploy the supporting infrastructure using OpenTofu.

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
