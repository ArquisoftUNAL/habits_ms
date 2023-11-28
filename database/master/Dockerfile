FROM bitnami/postgresql:11-debian-10

USER root
RUN mkdir -p /custom-habits-sql
COPY configure.sh configure.sh
COPY create-users.sh create-users.sh
COPY create-reader-user.sql /custom-habits-sql/create-reader-user.sql

CMD /bin/bash -c  "sh configure.sh && /opt/bitnami/scripts/postgresql/run.sh & sh create-users.sh"

# CMD /bin/bash -c "/opt/bitnami/scripts/postgresql/run.sh"