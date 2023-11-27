reader_user=$(echo $POSTGRESQL_READER_USER)
reader_password=$(echo $POSTGRESQL_READER_PASSWORD)
database=$(echo $POSTGRESQL_DATABASE)

echo "Replacing credentials in create-reader-user.sql"
echo "reader_user: $reader_user"
echo "reader_password: $reader_password"
echo "database: $database"

sed -i "s#POSTGRESQL_READER_USER#$reader_user#g" /docker-entrypoint-initdb.d/create-reader-user.sql
sed -i "s#POSTGRESQL_READER_PASSWORD#\'$reader_password\'#g" /docker-entrypoint-initdb.d/create-reader-user.sql
sed -i "s#POSTGRESQL_DATABASE#$database#g" /docker-entrypoint-initdb.d/create-reader-user.sql