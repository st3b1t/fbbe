FROM rust:1.65 as builder

WORKDIR /fbbe

COPY ./ ./

RUN cargo build --release

#TODO multistage FROM alpine

EXPOSE 3000

CMD ["./target/release/fbbe"]
