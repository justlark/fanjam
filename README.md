# sparklefish

## Setup

There are some quirks with the Redis and Postgres connection strings that are
not addressed in the NocoDB documentation.

The Postgres connection string should look like this. The `ssl=true` is
necessary for NocoDB to talk to [Neon](https://neon.tech/).

```
pg://DOMAIN:PORT?u=USER&p=PASSWORD&d=DATABASE&ssl=true
```

The Redis connection string should look like this. The `family=6` is necessary
for NocoDB to talk to [fly.io](https://fly.io/).

```
redis://USER:PASSWORD@DOMAIN:PORT/?family=6
```
