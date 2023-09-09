# Taken from: https://github.com/docker-library/postgres/issues/151

#!/bin/bash

set -e

POSTGRES="psql ${POSTGRES_DB} --username ${POSTGRES_USER}"

$POSTGRES <<-EOSQL
CREATE USER WITH PASSWORD '${MS_HABITS_PASSWORD}';
EOSQL

$POSTGRES <<-EOSQL
CREATE USER WITH PASSWORD '${MS_STATISTICS_PASSWORD}';
EOSQL