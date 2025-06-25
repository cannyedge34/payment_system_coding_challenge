-- Verify importer:add_unique_index_to_merchants on pg

BEGIN;

SELECT conname
FROM pg_constraint
WHERE conname = 'merchants_merchant_reference_unique';

ROLLBACK;
