-- Revert calculator:create_merchants_table from pg

BEGIN;

DROP TABLE IF EXISTS merchants;

COMMIT;
