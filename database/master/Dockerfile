FROM bitnami/postgresql:11-debian-10

# COPY replace-credentials.sh replace-credentials.sh
# COPY create-reader-user.sql /docker-entrypoint-initdb.d/create-reader-user.sql

# CMD /bin/bash -c  "sh replace-credentials.sh && /opt/bitnami/scripts/postgresql/run.sh"

CMD /bin/bash -c "/opt/bitnami/scripts/postgresql/run.sh"