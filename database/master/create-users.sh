sleep 10

echo "Executing create-reader-user.sql"
psql $POSTGRESQL_DATABASE -U $POSTGRESQL_USERNAME -f /docker-entrypoint-initdb.d/create-reader-user.sql

# Wait indefinitely
while true; do sleep 60000; done