FROM bitnami/postgresql:11-debian-10

COPY configure.sh configure.sh
COPY create-users.sh create-users.sh
COPY create-reader-user.sql /docker-entrypoint-initdb.d/create-reader-user.sql

CMD /bin/bash -c  "sh configure.sh && /opt/bitnami/scripts/postgresql/run.sh & sh create-users.sh"

# CMD /bin/bash -c "/opt/bitnami/scripts/postgresql/run.sh"