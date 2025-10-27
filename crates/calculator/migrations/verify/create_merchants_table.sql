-- Verify calculator:create_merchants_table on pg

BEGIN;

SELECT id, merchant_reference, live_on, disbursement_frequency, minimum_monthly_fee
FROM merchants
WHERE FALSE;

ROLLBACK;
