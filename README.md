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

See the following architecture diagram, laid out using
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

### Local Development

Run the client locally with `just run-client prod` and access an instance
pre-populated with example data by navigating to
<http://localhost:5173/app/demo/>.

Because instances of the FanJam server worker need to be paired with a NocoDB
instance to be useful, running the server worker locally isn't recommended for
doing frontend development. Instead, it's easier to run the client locally
against a deployed instance of the server worker.

By default, running `just run-client` will run the client locally against the
server worker in the `test` stage, which allows you to access any environment
deployed in that stage. To access environments deployed in the `prod` stage,
run `just run-client prod`.

The `demo` environment is a good choice for testing frontend changes locally.

### Running Linters

Use `just check-server` and `just check-client` to run type checkers and static
analysis for the backend and frontend respectively.

Linters are also run in CI.

### Running Tests

To run Playwright tests for the frontend, run `just start-playwright` to start
the Playwright server. This requires Podman to be installed and set up, but
saves you from having to install any Playwright-specific native dependencies,
which aren't supported on all platforms.

Once the server is running, run `just run-playwright` to run the test suite. It
will take several minutes.

Playwright tests are also run in CI.

## Infrastructure

### Authorization

Secrets for this project are stored in the repo encrypted with maintainers' SSH
keys via [SOPS](https://getsops.io/) and [age](https://age-encryption.org/).

To deploy infrastructure or run `just` recipes that touch deployed
environments, you'll first need your SSH key authorized by adding it to
[./infra/config/.sops.yaml](./infra/config/.sops.yaml) and running these
commands:

```
just sops updatekeys ./infra/config/secret_vars.enc.yaml
just sops updatekeys ./infra/config/secret_globals.enc.yaml
just sops updatekeys ./infra/config/env.enc.yaml
```

Once your key is authorized, set the env var `SOPS_AGE_SSH_PRIVATE_KEY_FILE` to
the path of your private SSH key. You can put this in a `./.env` file in the
root of the repo; it will be ignored by git.

### Advisory Checks

As a safety guard against developers making changes to deployed environments in
stages where they shouldn't, this repo implements _advisory checks_, where you
can configure for each stage which users (by their git email address) have
permission to make changes in that stage.

By "advisory", we mean that these checks are not enforced. Any user whose SSH
key is registered in `./infra/config/.sops.yaml` is assumed to be trusted and
could override it.

Note that these checks do not apply to deploying infrastructure with OpenTofu.

To change which users have permission to which stages, edit the relevant config
file, like this:

```
just sops edit ./infra/config/secret_globals.enc.yaml
```

### OpenTofu

To run `tofu` commands with the secrets pulled into your environment, use the
`just` recipe:

```
just tofu plan
```

You can edit secret OpenTofu variables interactively like this:

```
just sops edit ./infra/config/secret_vars.enc.yaml
```

You can edit plaintext OpenTofu variables by editing
[./infra/config/vars.yaml](./infra/config/vars.yaml).

You can set additional secret env vars to be passed to OpenTofu like this:

```
just sops edit ./infra/config/env.enc.yaml
```

These additional env vars are used to configure the Postgres state backend.

## Deployment

These are the instructions for deploying a new instance of FanJam in the `prod`
environment with the organizer app at `https://foo.fanjam.live` and the
attendee app at `https://fanjam.live/app/bar`

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
just init-env foo bar
```

## Upgrade NocoDB

These are instructions for upgrading NocoDB in deployed environments.

Redeploy the `playground` environment using `just deploy-env playground`. This
environment always uses the latest version of NocoDB. Once you've tested the
new version and are confident it's stable, update the `fly.yaml` of all the
environments you want to upgrade, then redeploy them. Production environments
should always be pinned to a specific version, not `latest`.

Finally, update the [globals.yaml](./infra/config/globals.yaml) with the latest
version tag (not `latest`).

## Copyright

Copyright © 2025-2026 Lark Aster

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.
