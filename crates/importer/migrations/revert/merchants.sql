-- Revert importer:merchants from pg

BEGIN;

DROP TABLE merchants;

COMMIT;
