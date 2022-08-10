####################################################################################################
## EXAMPLE
####################################################################################################
FROM rust:1.60 AS builder

WORKDIR /quoridor
COPY ./ .
RUN cargo install diesel_cli
RUN touch db.sqlite3
RUN diesel migration run --database-url db.sqlite3
RUN cargo build --release

EXPOSE 3000

CMD ["/quoridor/target/release/corridor_api"]
