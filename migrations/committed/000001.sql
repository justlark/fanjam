--! Previous: -
--! Hash: sha1:708881ba055f9fb47676abaff1503df488472937

DROP TABLE IF EXISTS noco_bases CASCADE;

CREATE TABLE noco_bases (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    base_id text NOT NULL UNIQUE,
    sequence serial NOT NULL,
    created_at timestamp NOT NULL DEFAULT now()
);

DROP TABLE IF EXISTS noco_migrations CASCADE;

CREATE TABLE noco_migrations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    version integer NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT now()
);
