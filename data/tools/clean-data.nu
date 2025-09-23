#!/usr/bin/env -S nu --stdin

use std
use std/log

source ./common.nu

def main [old_schema: string, new_schema: string] {
  let dump = $in

  let sql = $"
    ALTER SCHEMA ($old_schema) RENAME TO ($new_schema);

    UPDATE ($new_schema).events SET created_by = NULL;
    UPDATE ($new_schema).events SET updated_by = NULL;

    UPDATE ($new_schema).locations SET created_by = NULL;
    UPDATE ($new_schema).locations SET updated_by = NULL;

    UPDATE ($new_schema).people SET created_by = NULL;
    UPDATE ($new_schema).people SET updated_by = NULL;

    UPDATE ($new_schema).categories SET created_by = NULL;
    UPDATE ($new_schema).categories SET updated_by = NULL;

    UPDATE ($new_schema).tags SET created_by = NULL;
    UPDATE ($new_schema).tags SET updated_by = NULL;

    UPDATE ($new_schema).announcements SET created_by = NULL;
    UPDATE ($new_schema).announcements SET updated_by = NULL;

    UPDATE ($new_schema).about SET created_by = NULL;
    UPDATE ($new_schema).about SET updated_by = NULL;

    UPDATE ($new_schema).links SET created_by = NULL;
    UPDATE ($new_schema).links SET updated_by = NULL;

    UPDATE ($new_schema).pages SET created_by = NULL;
    UPDATE ($new_schema).pages SET updated_by = NULL;
  "

  psql-exec $dump
  psql-exec $sql
  pg-exec pg_dump --data-only --username $pg_user --dbname $db_name --schema $new_schema
}
