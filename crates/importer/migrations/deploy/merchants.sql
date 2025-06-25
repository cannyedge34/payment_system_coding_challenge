-- Deploy importer:merchants to pg

BEGIN;

CREATE TABLE merchants (
  id UUID PRIMARY KEY,
  merchant_reference TEXT NOT NULL,
  email TEXT NOT NULL,
  live_on DATE NOT NULL,
  disbursement_frequency TEXT NOT NULL CHECK (disbursement_frequency IN ('DAILY', 'WEEKLY')),
  minimum_monthly_fee INTEGER NOT NULL
);

COMMIT;
