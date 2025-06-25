#!/bin/bash
set -e
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE DATABASE importer_db;
    CREATE DATABASE importer_test_db;
EOSQL
