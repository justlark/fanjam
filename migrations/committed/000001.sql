--! Previous: -
--! Hash: sha1:5c92a140e01942ceed4e59f01acc3d53732cb8c2

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
    version integer NOT NULL,
    base uuid NOT NULL REFERENCES noco_bases (id) ON DELETE CASCADE,
    created_at timestamp NOT NULL DEFAULT now(),

    UNIQUE (version, base)
);
