# sparklefish

Sparklefish is the codename for a yet-to-be-named app designed for con
organizers and attendees to make organizing and navigating cons easier.

## Stack

- The client is written in Vue and TypeScript and hosted on [Cloudflare
  Workers](https://developers.cloudflare.com/workers/).
- The backend is written in Rust and hosted on Cloudflare Workers.
- The interface for con organizers is provided by a
  [NocoDB](https://nocodb.com/) instance hosted on [Fly.io](https://fly.io/).
- The Postgres database backing NocoDB is hosted on [Neon](https://neon.tech).

## Setup

There are some quirks with the Redis and Postgres connection strings that are
not addressed in the NocoDB documentation.

The Postgres connection string should look like this. The `ssl=true` is
necessary for NocoDB to talk to Neon.

```
pg://DOMAIN:PORT?u=USER&p=PASSWORD&d=DATABASE&ssl=true
```

The Redis connection string should look like this. The `family=6` is necessary
for NocoDB to talk to Fly.io.

```
redis://USER:PASSWORD@DOMAIN:PORT/?family=6
```
