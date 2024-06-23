FROM rust:1.78 as builder

WORKDIR /usr/src/olly
COPY . .
RUN cargo install --path .

CMD [ "olly-server" ]