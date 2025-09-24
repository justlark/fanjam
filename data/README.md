# Demo Data

We have a demo environment of FanJam set up at <https://demo.fanjam.live/>.

We keep a dump of the demo data checked into this repo in
[demo-dump.sql](./demo-dump.sql). We might want to update the demo from time to
time, in which case it's easiest to make the changes in NocoDB, get a fresh
database dump, and check it in. Sometimes we might want to wipe and re-seed the
demo environment, or set up a new demo environment for a specific client that
they can mess around in. This directory contains some crude tooling for doing
all of this.

Making a direct connection to the database requires the Postgres endpoint and
password, which you can get from the Neon console. Note that for the purposes
of these scripts you **must** use a **non-pooling** endpoint, otherwise bad
stuff might happen.

To get a fresh database dump, use [./tools/pg-dump.nu](./tools/pg-dump.nu).
You'll need to pass the Postgres endpoint host and the Postgres schema name as
positional arguments, and it will prompt you interactively for the password.
The Postgres schema name is just the NocoDB base ID. The script will print a
SQL dump to stdout.

However, this database dump cannot be imported into another environment
directly. You must:

1. Change the schema name to match the NocoDB base ID of the new environment.
2. Set the values of the `created_by` and `updated_by` columns in each table to
   `NULL`, since those user IDs won't exist in the new environment.

We have a script to automate this, but it works by running against a local
Postgres cluster, which means you first need to run
[./tools/pg-start.nu](./tools/pg-start.nu) to start Postgres in a container.
Wait for the logs to say it's ready to accept connections.

The script to modify the database dump is
[./tools/clean-dump.nu](./tools/clean-dump.nu). It takes the old SQL dump as
stdin, accepts the old NocoDB base ID and new NocoDB base ID as positional
arguments, and prints the modified SQL dump to stdout.

To upload the demo data to an environment, use
[./tools/pg-restore.nu](./tools/pg-restore.nu). It takes the modified database
dump as stdin, accepts the Postgres endpoint host as a positional argument, and
prompts interactively for the password.

After uploading the demo data, make sure to clear the server cache with `just
clear-cache <ENV>`.
