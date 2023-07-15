FROM rust:1.68.1-slim AS builder

# RUN apk add --no-cache musl-dev mpc1-dev pkgconfig openssl-dev mysql-dev
RUN apt update && apt upgrade -y && apt install -y libssl-dev musl-tools pkg-config default-libmysqlclient-dev libmariadb3
# RUN rustup target add x86_64-unknown-linux-musl
RUN update-ca-certificates
WORKDIR /usr/src
RUN cargo new --name listly-backend ./listly-backend
COPY Cargo.lock /usr/src/listly-backend/Cargo.lock
COPY Cargo.toml /usr/src/listly-backend/Cargo.toml
COPY .env /usr/src/listly-backend/.env
COPY database-update.sh /usr/src/listly-backend/database-update.sh
WORKDIR /usr/src/listly-backend
RUN cargo build --release
RUN rm /usr/src/listly-backend/src/*.rs
RUN rm /usr/src/listly-backend/target/release/deps/listly*

ADD src /usr/src/listly-backend/src
ADD .env /usr/src/listly-backend/.env
WORKDIR /usr/src/listly-backend
RUN cargo build --release
#--target x86_64-unknown-linux-musl --release 

FROM ubuntu:23.04
COPY --from=builder /usr/src/listly-backend/target/release/listly target/release/listly-backend
COPY --from=builder /usr/src/listly-backend/.env /.env
RUN apt update && apt upgrade -y && apt install -y mariadb-server mariadb-client
RUN export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/usr/lib/x86_64-linux-gnu/"

EXPOSE 8000
CMD ["/target/release/listly-backend"]