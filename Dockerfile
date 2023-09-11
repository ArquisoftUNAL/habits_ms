FROM rust

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo install --path .

WORKDIR /src/

CMD bash -c "diesel migration run && cargo run"

EXPOSE 3030