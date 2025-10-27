-- Deploy calculator:merchants to pg

BEGIN;

-- Enable pgcrypto extension for gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create merchants table
CREATE TABLE merchants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    merchant_reference TEXT UNIQUE NOT NULL,
    live_on DATE NOT NULL,
    disbursement_frequency TEXT NOT NULL CHECK (disbursement_frequency IN ('DAILY', 'WEEKLY')),
    minimum_monthly_fee INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

COMMIT;
