-- Revert importer:add_unique_index_to_merchants from pg

BEGIN;

ALTER TABLE merchants
DROP CONSTRAINT merchants_merchant_reference_unique;

COMMIT;
