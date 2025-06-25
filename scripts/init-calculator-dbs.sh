#!/bin/bash
set -e
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE DATABASE calculator_db;
    CREATE DATABASE calculator_test_db;
EOSQL
