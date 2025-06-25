-- Deploy importer:add_unique_index_to_merchants to pg

BEGIN;

ALTER TABLE merchants
ADD CONSTRAINT merchants_merchant_reference_unique UNIQUE (merchant_reference);

COMMIT;
